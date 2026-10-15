
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    let cpu_limit = 50.0;
    let memory_limit_mb = 500.0;

    println!("=== Process Resource Alert ===");
    println!("CPU threshold: {}%", cpu_limit);
    println!("Memory threshold: {} MB", memory_limit_mb);
    println!("Press Ctrl+C to stop.\n");

    loop {
        system.refresh_all();

        thread::sleep(Duration::from_secs(2));

        system.refresh_all();

        let mut alert_count = 0;

        for (pid, process) in system.processes() {
            let name = process.name().to_string_lossy();
            let cpu = process.cpu_usage();
            let memory_mb = process.memory() as f64 / 1_048_576.0;

            if cpu >= cpu_limit || memory_mb >= memory_limit_mb {
                alert_count += 1;

                println!(
                    "[ALERT] {} | PID: {} | CPU: {:.1}% | Memory: {:.1} MB",
                    name, pid, cpu, memory_mb
                );
            }
        }

        if alert_count == 0 {
            println!("No processes exceeded the configured thresholds.");
        }

        println!("--- Next check in 2 seconds ---\n");
    }
}
