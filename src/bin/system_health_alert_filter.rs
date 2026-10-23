
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

    let mut records = 0;
    let mut cpu_alerts = 0;
    let mut memory_alerts = 0;
    let mut max_cpu: f64 = 0.0;
    let mut max_memory: f64 = 0.0;

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
            Ok(value) => value,
            Err(_) => continue,
        };

        let memory: f64 = match columns[2].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        records += 1;

        if columns[3].contains("HIGH_CPU") {
            cpu_alerts += 1;
        }

        if columns[3].contains("HIGH_MEMORY") {
            memory_alerts += 1;
        }

        max_cpu = max_cpu.max(cpu);
        max_memory = max_memory.max(memory);
    }

    println!("\nFILTERED SYSTEM ALERT REPORT");
    println!("============================");
    println!("Matching records: {}", records);
    println!("CPU alerts: {}", cpu_alerts);
    println!("Memory alerts: {}", memory_alerts);
    println!("Maximum CPU: {:.2}%", max_cpu);
    println!("Maximum memory: {:.2}%", max_memory);

    Ok(())
}
