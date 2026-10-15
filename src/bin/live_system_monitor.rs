
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    println!("Live System Monitor");
    println!("Press Ctrl+C to stop.\n");

    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();

        thread::sleep(Duration::from_secs(1));

        system.refresh_cpu_usage();
        system.refresh_memory();

        let cpu = system.global_cpu_usage();
        let used_memory = system.used_memory() as f64 / 1_073_741_824.0;
        let total_memory = system.total_memory() as f64 / 1_073_741_824.0;

        let memory_percent = if total_memory > 0.0 {
            used_memory / total_memory * 100.0
        } else {
            0.0
        };

        print!("\x1B[2J\x1B[1;1H");
        println!("=== Live System Monitor ===");
        println!("CPU Usage:    {:.2}%", cpu);
        println!(
            "Memory Usage: {:.2} / {:.2} GiB ({:.2}%)",
            used_memory, total_memory, memory_percent
        );
        println!("\nPress Ctrl+C to stop.");
    }
}
