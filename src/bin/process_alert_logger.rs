
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::System;

fn main() -> std::io::Result<()> {
    let mut system = System::new_all();

    let cpu_limit = 50.0;
    let memory_limit_mb = 500.0;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("process_alerts.csv")?;

    if file.metadata()?.len() == 0 {
        writeln!(
            file,
            "Timestamp,PID,Process,CPU_Percent,Memory_MB,Alert"
        )?;
    }

    println!("=== Process Alert Logger ===");
    println!("Logging alerts to process_alerts.csv");
    println!("Press Ctrl+C to stop.\n");

    loop {
        system.refresh_all();
        thread::sleep(Duration::from_secs(2));
        system.refresh_all();

        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_secs(),
            Err(error) => {
                eprintln!("Clock error: {}", error);
                continue;
            }
        };

        let mut alerts_found = 0;

        for (pid, process) in system.processes() {
            let name = process.name().to_string_lossy().to_string();
            let cpu = process.cpu_usage();
            let memory_mb = process.memory() as f64 / 1_048_576.0;

            let mut reasons = Vec::new();

            if cpu >= cpu_limit {
                reasons.push("HIGH_CPU");
            }

            if memory_mb >= memory_limit_mb {
                reasons.push("HIGH_MEMORY");
            }

            if !reasons.is_empty() {
                let alert = reasons.join("|");

                println!(
                    "[ALERT] {} | PID: {} | CPU: {:.1}% | Memory: {:.1} MB",
                    name, pid, cpu, memory_mb
                );

                let safe_name = name.replace('"', "\"\"");

                writeln!(
                    file,
                    "{},\"{}\",\"{}\",{:.2},{:.2},\"{}\"",
                    timestamp, pid, safe_name, cpu, memory_mb, alert
                )?;

                alerts_found += 1;
            }
        }

        file.flush()?;

        if alerts_found == 0 {
            println!("No process exceeded the configured thresholds.");
        }

        println!("Recorded {} alerts.\n", alerts_found);
    }
}
