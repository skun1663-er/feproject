
use std::fs::OpenOptions;
use std::io::Write;
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

    let file_exists = std::path::Path::new("system_health_history.csv").exists();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("system_health_history.csv")?;

    if !file_exists || file.metadata()?.len() == 0 {
        writeln!(
            file,
            "Timestamp,CPU_Percent,Memory_Used_GiB,Memory_Total_GiB,Memory_Percent,Process_Count"
        )?;
    }

    writeln!(
        file,
        "{},{:.2},{:.2},{:.2},{:.2},{}",
        timestamp,
        cpu,
        used_gib,
        total_gib,
        memory_percent,
        process_count
    )?;

    println!("=== System Health Snapshot ===");
    println!("CPU Usage: {:.2}%", cpu);
    println!("Memory Usage: {:.2}%", memory_percent);
    println!("Memory Used: {:.2} / {:.2} GiB", used_gib, total_gib);
    println!("Running Processes: {}", process_count);
    println!("\nSnapshot appended to system_health_history.csv");

    Ok(())
}
