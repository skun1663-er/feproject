
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;

fn main() -> std::io::Result<()> {
    let mut system = System::new_all();

    let cpu_limit = 80.0;
    let memory_limit = 80.0;

    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("system_health_alerts.csv")?;

    if log.metadata()?.len() == 0 {
        writeln!(log, "Timestamp,CPU_Percent,Memory_Percent,Alert")?;
    }

    println!("System Health Alert Dashboard");
    println!("CPU threshold: {}%", cpu_limit);
    println!("RAM threshold: {}%", memory_limit);
    println!("Press Ctrl+C to stop.");

    thread::sleep(Duration::from_secs(1));

    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();

        thread::sleep(Duration::from_secs(1));

        system.refresh_cpu_usage();
        system.refresh_memory();

        let cpu = system.global_cpu_usage();
        let total = system.total_memory() as f64;
        let used = system.used_memory() as f64;

        let memory_percent = if total > 0.0 {
            used / total * 100.0
        } else {
            0.0
        };

        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_secs(),
            Err(error) => {
                eprintln!("Clock error: {}", error);
                continue;
            }
        };

        let mut alerts = Vec::new();

        if cpu >= cpu_limit {
            alerts.push("HIGH_CPU");
        }

        if memory_percent >= memory_limit {
            alerts.push("HIGH_MEMORY");
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("====================================");
        println!("      SYSTEM HEALTH ALERTS          ");
        println!("====================================");
        println!("CPU Usage:    {:.2}%", cpu);
        println!("Memory Usage: {:.2}%", memory_percent);

        if alerts.is_empty() {
            println!("\nStatus: NORMAL");
        } else {
            let alert_text = alerts.join("|");

            println!("\nStatus: WARNING");
            println!("Alert: {}", alert_text);

            writeln!(
                log,
                "{},{:.2},{:.2},{}",
                timestamp, cpu, memory_percent, alert_text
            )?;

            log.flush()?;
            println!("Alert saved to system_health_alerts.csv");
        }

        println!("\nRefreshing every second. Press Ctrl+C to stop.");
    }
}
