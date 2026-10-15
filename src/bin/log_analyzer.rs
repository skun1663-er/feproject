
use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    let log_path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Usage: cargo run --bin log_analyzer -- <log_file>");
            return;
        }
    };

    match analyze_log(log_path) {
        Ok(()) => println!("Log analysis completed."),
        Err(error) => println!("Error reading log: {}", error),
    }
}

fn analyze_log(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;

    let mut info_count = 0;
    let mut warning_count = 0;
    let mut error_count = 0;

    for line in contents.lines() {
        if line.contains("INFO:") {
            info_count += 1;
        }
        if line.contains("WARNING:") {
            warning_count += 1;
        }
        if line.contains("ERROR:") {
            error_count += 1;
        }
    }

    println!("--- Log Analysis Report ---");
    println!("INFO entries: {}", info_count);
    println!("WARNING entries: {}", warning_count);
    println!("ERROR entries: {}", error_count);
    println!(
        "Total entries: {}",
        info_count + warning_count + error_count
    );

    Ok(())
}
