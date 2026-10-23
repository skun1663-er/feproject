
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut hourly_counts = [0u32; 24];
    let mut total_alerts = 0u32;

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let timestamp: u64 = match columns[0].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let alert = columns[3];

        if !alert.contains("HIGH_CPU") && !alert.contains("HIGH_MEMORY") {
            continue;
        }

        // Unix timestamps are converted to UTC hours.
        let hour = ((timestamp % 86_400) / 3_600) as usize;

        hourly_counts[hour] += 1;
        total_alerts += 1;
    }

    println!("\nSYSTEM HEALTH ALERTS BY HOUR (UTC)");
    println!("==================================");

    if total_alerts == 0 {
        println!("No alert records found in system_health_alerts.csv.");
        return Ok(());
    }

    for (hour, count) in hourly_counts.iter().enumerate() {
        println!("{:02}:00 | {:>4} | {}", hour, count, "#".repeat((*count).min(50) as usize));
    }

    let busiest_hour = hourly_counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .map(|(hour, _)| hour)
        .unwrap_or(0);

    println!("\nTotal alert records: {}", total_alerts);
    println!("Busiest hour: {:02}:00 UTC", busiest_hour);

    Ok(())
}
