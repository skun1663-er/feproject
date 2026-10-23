// project 42: System Health Alert Severity Classifier
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;

    println!("\nSYSTEM HEALTH SEVERITY REPORT");
    println!("=============================");

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
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

        let alert = columns[3];

        let severity = if cpu >= 95.0 || memory >= 95.0 {
            high += 1;
            "HIGH"
        } else if cpu >= 85.0 || memory >= 85.0 {
            medium += 1;
            "MEDIUM"
        } else {
            low += 1;
            "LOW"
        };

        println!(
            "CPU: {:>6.2}% | RAM: {:>6.2}% | Alert: {:<20} | Severity: {}",
            cpu, memory, alert, severity
        );
    }

    println!("\nSUMMARY");
    println!("High severity: {}", high);
    println!("Medium severity: {}", medium);
    println!("Low severity: {}", low);

    Ok(())
}
