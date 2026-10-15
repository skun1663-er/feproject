
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = System::new_all();

    std::thread::sleep(Duration::from_secs(1));
    system.refresh_all();

    let cpu = system.global_cpu_usage();
    let total_memory = system.total_memory() as f64;
    let used_memory = system.used_memory() as f64;

    let total_gib = total_memory / 1_073_741_824.0;
    let used_gib = used_memory / 1_073_741_824.0;

    let memory_percent = if total_memory > 0.0 {
        used_memory / total_memory * 100.0
    } else {
        0.0
    };

    let process_count = system.processes().len();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    let health = if cpu >= 90.0 || memory_percent >= 90.0 {
        "WARNING"
    } else {
        "NORMAL"
    };

    let report = format!(
        "=== SYSTEM HEALTH REPORT ===\n\
         Timestamp (Unix): {}\n\
         Overall Status: {}\n\n\
         CPU Usage: {:.2}%\n\
         Memory Used: {:.2} GiB\n\
         Total Memory: {:.2} GiB\n\
         Memory Usage: {:.2}%\n\
         Running Processes: {}\n",
        timestamp,
        health,
        cpu,
        used_gib,
        total_gib,
        memory_percent,
        process_count
    );

    fs::write("system_health_report.txt", &report)?;

    println!("{}", report);
    println!("Report saved to system_health_report.txt");

    Ok(())
}
