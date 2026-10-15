
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    let cpu_limit = 80.0;
    let memory_limit = 80.0;

    println!("=== System Alert Monitor ===");
    println!("CPU alert threshold: {}%", cpu_limit);
    println!("Memory alert threshold: {}%", memory_limit);
    println!("Press Ctrl+C to stop.\n");

    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();
        thread::sleep(Duration::from_secs(1));
        system.refresh_cpu_usage();
        system.refresh_memory();

        let cpu = system.global_cpu_usage();

        let total_memory = system.total_memory() as f64;
        let used_memory = system.used_memory() as f64;

        let memory_percent = if total_memory > 0.0 {
            used_memory / total_memory * 100.0
        } else {
            0.0
        };

        println!(
            "CPU: {:.1}% | Memory: {:.1}%",
            cpu, memory_percent
        );

        if cpu >= cpu_limit {
            println!("[CPU ALERT] Usage is above the threshold!");
        }

        if memory_percent >= memory_limit {
            println!("[MEMORY ALERT] Usage is above the threshold!");
        }

        println!("------------------------------");
    }
}
