
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    // Allow time for the first CPU measurement.
    thread::sleep(Duration::from_secs(1));

    println!("Starting System Health Dashboard...");
    println!("Press Ctrl+C to stop.");

    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();
        system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let cpu = system.global_cpu_usage();
        let used = system.used_memory() as f64;
        let total = system.total_memory() as f64;

        let used_gib = used / 1_073_741_824.0;
        let total_gib = total / 1_073_741_824.0;

        let memory_percent = if total > 0.0 {
            used / total * 100.0
        } else {
            0.0
        };

        let process_count = system.processes().len();

        let cpu_bars = (cpu / 5.0).round().clamp(0.0, 20.0) as usize;
        let memory_bars = (memory_percent / 5.0)
            .round()
            .clamp(0.0, 20.0) as usize;

        print!("\x1B[2J\x1B[1;1H");

        println!("========================================");
        println!("         SYSTEM HEALTH DASHBOARD        ");
        println!("========================================");

        println!(
            "\nCPU Usage:    {:>6.2}% [{}{}]",
            cpu,
            "#".repeat(cpu_bars),
            " ".repeat(20 - cpu_bars)
        );

        println!(
            "RAM Usage:    {:>6.2}% [{}{}]",
            memory_percent,
            "#".repeat(memory_bars),
            " ".repeat(20 - memory_bars)
        );

        println!(
            "\nMemory: {:.2} / {:.2} GiB",
            used_gib, total_gib
        );

        println!("Running Processes: {}", process_count);

        let health = if cpu >= 90.0 || memory_percent >= 90.0 {
            "WARNING"
        } else {
            "NORMAL"
        };

        println!("Health Status: {}", health);
        println!("\nRefreshing every 2 seconds...");
        println!("Press Ctrl+C to exit.");

        thread::sleep(Duration::from_secs(2));
        system.refresh_cpu_usage();
    }
}
