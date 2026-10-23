
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut cpu_alerts = 0;
    let mut memory_alerts = 0;
    let mut both_alerts = 0;

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let alert = columns[3];
        let has_cpu = alert.contains("HIGH_CPU");
        let has_memory = alert.contains("HIGH_MEMORY");

        if has_cpu {
            cpu_alerts += 1;
        }

        if has_memory {
            memory_alerts += 1;
        }

        if has_cpu && has_memory {
            both_alerts += 1;
        }
    }

    println!("\nSYSTEM HEALTH ALERT VISUALIZER");
    println!("==============================");

    println!("\nCPU Alerts:    {} ", cpu_alerts);
    println!("{}", "#".repeat(cpu_alerts.min(100)));

    println!("\nMemory Alerts: {} ", memory_alerts);
    println!("{}", "#".repeat(memory_alerts.min(100)));

    println!("\nRecords with both alerts: {}", both_alerts);

    println!("\nLegend: Each # represents one alert.");
    println!("Bars are capped at 100 characters.");

    Ok(())
}
