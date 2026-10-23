
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let content = fs::read_to_string("system_health_alerts.csv")?;

    let mut search = String::new();

    println!("SYSTEM HEALTH ALERT SEARCH");
    println!("1. HIGH_CPU");
    println!("2. HIGH_MEMORY");
    println!("3. BOTH");
    print!("Choose an option: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut search)?;

    let keyword = match search.trim() {
        "1" => "HIGH_CPU",
        "2" => "HIGH_MEMORY",
        "3" => "BOTH",
        _ => {
            println!("Invalid option.");
            return Ok(());
        }
    };

    let mut found = 0;

    println!("\nMatching alert records:");
    println!("-----------------------");

    for line in content.lines().skip(1) {
        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() < 4 {
            continue;
        }

        let alert = columns[3];

        let matches = match keyword {
            "HIGH_CPU" => alert.contains("HIGH_CPU"),
            "HIGH_MEMORY" => alert.contains("HIGH_MEMORY"),
            "BOTH" => {
                alert.contains("HIGH_CPU") && alert.contains("HIGH_MEMORY")
            }
            _ => false,
        };

        if matches {
            println!("{}", line);
            found += 1;
        }
    }

    println!("\nTotal matching records: {}", found);

    Ok(())
}
