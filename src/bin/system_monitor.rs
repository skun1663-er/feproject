
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    // Wait between CPU measurements for a more useful reading.
    thread::sleep(Duration::from_millis(500));
    system.refresh_all();

    let total_memory = system.total_memory() as f64 / 1_073_741_824.0;
    let used_memory = system.used_memory() as f64 / 1_073_741_824.0;
    let cpu_usage = system.global_cpu_usage();

    println!("=== System Resource Monitor ===");
    println!("CPU Usage: {:.2}%", cpu_usage);
    println!("Memory Used: {:.2} GB", used_memory);
    println!("Total Memory: {:.2} GB", total_memory);

    if total_memory > 0.0 {
        let memory_percentage = (used_memory / total_memory) * 100.0;
        println!("Memory Usage: {:.2}%", memory_percentage);
    }
}
