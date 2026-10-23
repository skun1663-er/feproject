
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("system_health_history.csv")?;

    let mut count = 0_u64;
    let mut cpu_sum = 0.0;
    let mut memory_sum = 0.0;
    let mut peak_cpu: f64 = 0.0;
    let mut peak_memory: f64 = 0.0;

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 6 {
            continue;
        }

        let cpu: f64 = match columns[1].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let memory: f64 = match columns[4].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        cpu_sum += cpu;
        memory_sum += memory;
        peak_cpu = peak_cpu.max(cpu);
        peak_memory = peak_memory.max(memory);
        count += 1;
    }

    if count == 0 {
        println!("No valid history records found.");
        return Ok(());
    }

    println!("=== System Health Trend Report ===");
    println!("Snapshots analyzed: {}", count);
    println!("Average CPU usage: {:.2}%", cpu_sum / count as f64);
    println!(
        "Average memory usage: {:.2}%",
        memory_sum / count as f64
    );
    println!("Peak CPU usage: {:.2}%", peak_cpu);
    println!("Peak memory usage: {:.2}%", peak_memory);

    Ok(())
}
