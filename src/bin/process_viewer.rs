
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    thread::sleep(Duration::from_secs(1));
    system.refresh_all();

    println!("=== Running Process Viewer ===\n");
    println!(
        "{:<35} {:>10} {:>12}",
        "PROCESS", "CPU (%)", "MEMORY (MB)"
    );
    println!("{}", "-".repeat(60));

    let mut processes: Vec<_> = system.processes().values().collect();

    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

    for process in processes.iter().take(20) {
        let name = process.name().to_string_lossy();
        let name = if name.chars().count() > 35 {
            format!("{}...", name.chars().take(32).collect::<String>())
        } else {
            name.to_string()
        };

        let memory_mb = process.memory() as f64 / 1_048_576.0;

        println!(
            "{:<35} {:>10.1} {:>12.1}",
            name,
            process.cpu_usage(),
            memory_mb
        );
    }

    println!("\nShowing the 20 processes using the most memory.");
}
