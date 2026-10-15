
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() -> io::Result<()> {
    let urls = vec![
        "https://example.com",
        "https://www.rust-lang.org",
        "https://www.wikipedia.org",
    ];

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; RustWebsiteMonitor/1.0)")
        .build()
        .map_err(io::Error::other)?;

    let file_exists = std::path::Path::new("website_history.csv").exists();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("website_history.csv")?;

    if !file_exists {
        writeln!(
            file,
            "Timestamp,URL,HTTP Status,Response Time (ms),Result"
        )?;
    }

    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(error) => {
            eprintln!("System clock error: {}", error);
            return Ok(());
        }
    };

    for url in &urls {
        let start = Instant::now();

        let (status, result) = match client.get(*url).send() {
            Ok(response) => {
                let status = response.status().to_string();
                let result = if response.status().is_success() {
                    "SUCCESS"
                } else {
                    "HTTP ERROR"
                };
                (status, result)
            }
            Err(error) => {
                eprintln!("Request failed for {}: {}", url, error);
                ("REQUEST FAILED".to_string(), "FAILED")
            }
        };

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        println!(
            "{} | {} | {} | {:.2} ms",
            url, status, result, elapsed
        );

        writeln!(
            file,
            "{},\"{}\",\"{}\",{:.2},\"{}\"",
            timestamp, url, status, elapsed, result
        )?;
    }

    println!("Historical results appended to website_history.csv");

    Ok(())
}
