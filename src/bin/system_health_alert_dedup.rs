// project 44: System Health Alert Deduplication
use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut seen = HashSet::new();
    let mut unique_rows = Vec::new();

    let mut lines = content.lines();

    let header = lines.next().unwrap_or(
        "Timestamp,CPU_Percent,Memory_Percent,Alert"
    );

    unique_rows.push(header.to_string());

    let mut total_rows = 0;
    let mut duplicates = 0;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        total_rows += 1;

        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        // Ignore the timestamp when comparing alert content.
        let fingerprint = format!(
            "{}|{}|{}",
            columns[1], columns[2], columns[3]
        );

        if seen.insert(fingerprint) {
            unique_rows.push(line.to_string());
        } else {
            duplicates += 1;
        }
    }

    let output = unique_rows.join("\n") + "\n";

    fs::write("system_health_alerts_deduplicated.csv", output)?;

    println!("\nSYSTEM HEALTH ALERT DEDUPLICATION");
    println!("=================================");
    println!("Input data rows: {}", total_rows);
    println!("Duplicate rows removed: {}", duplicates);
    println!("Unique rows retained: {}", seen.len());
    println!("Output: system_health_alerts_deduplicated.csv");

    Ok(())
}
