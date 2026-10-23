//Project 45: System Health Alert Acknowledgment Tracker

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut output = OpenOptions::new()
        .create(true)
        .append(true)
        .open("system_health_acknowledgments.csv")?;

    if output.metadata()?.len() == 0 {
        writeln!(output, "Timestamp,CPU_Percent,Memory_Percent,Alert,Status")?;
    }

    let mut lines = content.lines().skip(1);
    let mut alerts = Vec::new();

    for line in lines.by_ref() {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        alerts.push((
            columns[0].to_string(),
            columns[1].to_string(),
            columns[2].to_string(),
            columns[3].to_string(),
        ));
    }

    if alerts.is_empty() {
        println!("No alert records found.");
        return Ok(());
    }

    println!("\nSYSTEM HEALTH ACKNOWLEDGMENT TRACKER");
    println!("====================================");

    for (index, alert) in alerts.iter().enumerate() {
        println!(
            "{}. Timestamp: {} | CPU: {}% | RAM: {}% | {}",
            index + 1,
            alert.0,
            alert.1,
            alert.2,
            alert.3
        );
    }

    println!("\nEnter alert number to acknowledge (0 to exit):");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice: usize = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input.");
            return Ok(());
        }
    };

    if choice == 0 {
        println!("No acknowledgment recorded.");
        return Ok(());
    }

    if choice > alerts.len() {
        println!("Alert number out of range.");
        return Ok(());
    }

    let selected = &alerts[choice - 1];

    writeln!(
        output,
        "{},{},{},{},ACKNOWLEDGED",
        selected.0, selected.1, selected.2, selected.3
    )?;

    output.flush()?;

    println!("Alert acknowledged and saved.");
    println!("File: system_health_acknowledgments.csv");

    Ok(())
}
