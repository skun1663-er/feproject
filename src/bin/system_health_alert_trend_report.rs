
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut records: Vec<(u64, u32, u32)> = Vec::new();

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let timestamp: u64 = match columns[0].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let cpu_alert = columns[3].contains("HIGH_CPU");
        let memory_alert = columns[3].contains("HIGH_MEMORY");

        records.push((
            timestamp,
            u32::from(cpu_alert),
            u32::from(memory_alert),
        ));
    }

    if records.is_empty() {
        println!("No alert records found.");
        return Ok(());
    }

    records.sort_by_key(|record| record.0);

    let midpoint = records.len() / 2;

    if midpoint == 0 {
        println!("Not enough records to compare two periods.");
        println!("Total alert records: {}", records.len());
        return Ok(());
    }

    let first_half = &records[..midpoint];
    let second_half = &records[midpoint..];

    let count_alerts = |period: &[(u64, u32, u32)]| -> (u32, u32) {
        let cpu = period.iter().map(|record| record.1).sum();
        let memory = period.iter().map(|record| record.2).sum();
        (cpu, memory)
    };

    let (first_cpu, first_memory) = count_alerts(first_half);
    let (second_cpu, second_memory) = count_alerts(second_half);

    println!("\nSYSTEM HEALTH ALERT TREND REPORT");
    println!("================================");
    println!("Total records: {}", records.len());
    println!("\nFirst half of recorded history:");
    println!("CPU alerts: {}", first_cpu);
    println!("Memory alerts: {}", first_memory);

    println!("\nSecond half of recorded history:");
    println!("CPU alerts: {}", second_cpu);
    println!("Memory alerts: {}", second_memory);

    println!("\nTrend comparison:");
    println!(
        "CPU alerts: {}",
        if second_cpu > first_cpu {
            "INCREASING"
        } else if second_cpu < first_cpu {
            "DECREASING"
        } else {
            "UNCHANGED"
        }
    );

    println!(
        "Memory alerts: {}",
        if second_memory > first_memory {
            "INCREASING"
        } else if second_memory < first_memory {
            "DECREASING"
        } else {
            "UNCHANGED"
        }
    );

    Ok(())
}
