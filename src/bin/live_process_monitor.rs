
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system = System::new_all();

    println!("=== Live Process Monitor ===");
    println!("Press Ctrl+C to stop.\n");

    loop {
        system.refresh_all();

        let mut processes: Vec<_> = system.processes().values().collect();

        processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

        print!("\x1B[2J\x1B[1;1H");
        println!("=== Live Process Monitor ===");
        println!(
            "{:<32} {:>10} {:>12}",
            "PROCESS", "CPU (%)", "MEMORY (MB)"
        );
        println!("{}", "-".repeat(58));

        for process in processes.iter().take(15) {
            let name = process.name().to_string_lossy();
            let name = if name.chars().count() > 32 {
                format!("{}...", name.chars().take(29).collect::<String>())
            } else {
                name.to_string()
            };

            println!(
                "{:<32} {:>10.1} {:>12.1}",
                name,
                process.cpu_usage(),
                process.memory() as f64 / 1_048_576.0
            );
        }

        println!("\nRefreshing every 2 seconds. Press Ctrl+C to stop.");

        thread::sleep(Duration::from_secs(2));
    }
}
