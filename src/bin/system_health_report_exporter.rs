
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut start_input = String::new();
    let mut end_input = String::new();

    print!("Enter start Unix timestamp: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut start_input)?;

    print!("Enter end Unix timestamp: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut end_input)?;

    let start: u64 = start_input.trim().parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid start timestamp"))?;

    let end: u64 = end_input.trim().parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid end timestamp"))?;

    if start > end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Start timestamp must not exceed end timestamp",
        ));
    }

    let mut report = String::from(
        "Timestamp,CPU_Percent,Memory_Percent,Alert\n"
    );

    let mut exported = 0;

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let timestamp: u64 = match columns[0].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if timestamp < start || timestamp > end {
            continue;
        }

        let cpu: f64 = match columns[1].parse() {
            Ok(value) if (0.0..=100.0).contains(&value) => value,
            _ => continue,
        };

        let memory: f64 = match columns[2].parse() {
            Ok(value) if (0.0..=100.0).contains(&value) => value,
            _ => continue,
        };

        report.push_str(&format!(
            "{},{:.2},{:.2},{}\n",
            timestamp, cpu, memory, columns[3]
        ));

        exported += 1;
    }

    fs::write("filtered_system_health_report.csv", report)?;

    println!("\nExport complete.");
    println!("Records exported: {}", exported);
    println!("File: filtered_system_health_report.csv");

    Ok(())
}
