
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("website_history.csv")?;

    let mut stats: HashMap<String, (u32, u32)> = HashMap::new();

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 5 {
            continue;
        }

        let url = columns[1].trim_matches('"').to_string();
        let result = columns[4].trim_matches('"');

        let entry = stats.entry(url).or_insert((0, 0));
        entry.1 += 1;

        if result == "SUCCESS" {
            entry.0 += 1;
        }
    }

    println!("\n=== Website Uptime Summary ===");

    for (url, (successes, total)) in &stats {
        let uptime = (*successes as f64 / *total as f64) * 100.0;

        println!(
            "{}\nSuccessful checks: {}/{}\nUptime rate: {:.2}%\n",
            url, successes, total, uptime
        );
    }

    Ok(())
}
