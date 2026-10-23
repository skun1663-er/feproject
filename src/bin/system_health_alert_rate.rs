// project 41: System Health Alert Rate Analyzer
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut records: Vec<(u64, bool, bool)> = Vec::new();

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let timestamp: u64 = match columns[0].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let cpu = columns[3].contains("HIGH_CPU");
        let memory = columns[3].contains("HIGH_MEMORY");

        records.push((timestamp, cpu, memory));
    }

    if records.is_empty() {
        println!("No alert records found.");
        return Ok(());
    }

    records.sort_by_key(|record| record.0);

    let first_timestamp = records.first().unwrap().0;
    let last_timestamp = records.last().unwrap().0;

    let elapsed_hours =
        (last_timestamp.saturating_sub(first_timestamp) as f64 / 3600.0).max(1.0 / 3600.0);

    let cpu_alerts = records.iter().filter(|record| record.1).count();
    let memory_alerts = records.iter().filter(|record| record.2).count();

    println!("\nSYSTEM HEALTH ALERT RATE REPORT");
    println!("===============================");
    println!("Total alert records: {}", records.len());
    println!("Elapsed time: {:.2} hours", elapsed_hours);
    println!("CPU alert records: {}", cpu_alerts);
    println!("Memory alert records: {}", memory_alerts);
    println!(
        "CPU alerts per hour: {:.2}",
        cpu_alerts as f64 / elapsed_hours
    );
    println!(
        "Memory alerts per hour: {:.2}",
        memory_alerts as f64 / elapsed_hours
    );

    Ok(())
}
