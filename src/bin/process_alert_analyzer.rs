
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("process_alerts.csv")?;

    let mut process_counts: HashMap<String, u32> = HashMap::new();
    let mut cpu_alerts = 0;
    let mut memory_alerts = 0;
    let mut total_alerts = 0;

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 6 {
            continue;
        }

        let process = columns[2].trim_matches('"').to_string();
        let alert = columns[5].trim_matches('"');

        *process_counts.entry(process).or_insert(0) += 1;
        total_alerts += 1;

        if alert.contains("HIGH_CPU") {
            cpu_alerts += 1;
        }

        if alert.contains("HIGH_MEMORY") {
            memory_alerts += 1;
        }
    }

    let mut ranked: Vec<_> = process_counts.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1));

    println!("=== Process Alert Analysis ===");
    println!("Total alert records: {}", total_alerts);
    println!("High CPU records: {}", cpu_alerts);
    println!("High memory records: {}", memory_alerts);

    println!("\n=== Top 10 Processes by Alert Count ===");

    for (process, count) in ranked.iter().take(10) {
        println!("{}: {} alerts", process, count);
    }

    Ok(())
}
