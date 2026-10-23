
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut total_alerts = 0;
    let mut high_cpu_count = 0;
    let mut high_memory_count = 0;
    let mut max_cpu: f64 = 0.0;
    let mut max_memory: f64 = 0.0;

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let cpu: f64 = match columns[1].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let memory: f64 = match columns[2].parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let alert = columns[3];

        total_alerts += 1;

        if alert.contains("HIGH_CPU") {
            high_cpu_count += 1;
        }

        if alert.contains("HIGH_MEMORY") {
            high_memory_count += 1;
        }

        if cpu > max_cpu {
            max_cpu = cpu;
        }

        if memory > max_memory {
            max_memory = memory;
        }
    }

    let report = format!(
        "SYSTEM HEALTH ALERT STATISTICS\n\
         =============================\n\
         Total Alert Records: {}\n\
         High CPU Alerts: {}\n\
         High Memory Alerts: {}\n\
         Maximum CPU Recorded: {:.2}%\n\
         Maximum Memory Recorded: {:.2}%\n",
        total_alerts,
        high_cpu_count,
        high_memory_count,
        max_cpu,
        max_memory
    );

    println!("{}", report);

    fs::write("system_health_alert_stats.txt", &report)?;

    println!("Report saved to system_health_alert_stats.txt");

    Ok(())
}
