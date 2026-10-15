
use std::io::{self, Write};
use sysinfo::System;

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    print!("Enter a process name to search for: ");
    io::stdout().flush().expect("Failed to flush output");

    let mut search = String::new();

    if io::stdin().read_line(&mut search).is_err() {
        println!("Failed to read input.");
        return;
    }

    let search = search.trim().to_lowercase();

    if search.is_empty() {
        println!("Please enter a process name.");
        return;
    }

    println!("\n=== Process Search Results ===");

    let mut found = false;

    for (pid, process) in system.processes() {
        let name = process.name().to_string_lossy();

        if name.to_lowercase().contains(&search) {
            found = true;

            println!("Process: {}", name);
            println!("PID: {}", pid);
            println!("CPU Usage: {:.2}%", process.cpu_usage());
            println!(
                "Memory: {:.2} MB",
                process.memory() as f64 / 1_048_576.0
            );
            println!("------------------------------");
        }
    }

    if !found {
        println!("No matching processes found.");
    }
}
