// Project 43: System Health Alert Notification Simulator

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut notifications = 0;

    println!("\nSYSTEM HEALTH NOTIFICATION SIMULATOR");
    println!("====================================");

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let timestamp: u64 = match columns[0].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let cpu: f64 = match columns[1].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let memory: f64 = match columns[2].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let alert = columns[3];

        if !alert.contains("HIGH_CPU") && !alert.contains("HIGH_MEMORY") {
            continue;
        }

        let severity = if cpu >= 95.0 || memory >= 95.0 {
            "CRITICAL"
        } else if cpu >= 85.0 || memory >= 85.0 {
            "WARNING"
        } else {
            "NOTICE"
        };

        println!("\n--- SIMULATED NOTIFICATION ---");
        println!("Severity: {}", severity);
        println!("Timestamp: {}", timestamp);
        println!("CPU Usage: {:.2}%", cpu);
        println!("Memory Usage: {:.2}%", memory);
        println!("Alert Type: {}", alert);
        println!("Status: Notification simulated");

        notifications += 1;
    }

    println!("\n====================================");
    println!("Notifications simulated: {}", notifications);
    println!("No actual messages were sent.");

    Ok(())
}
