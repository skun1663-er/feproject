
use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};

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

    let mut file = File::create("website_report.csv")?;

    writeln!(file, "URL,HTTP Status,Response Time (ms),Result")?;

    for url in &urls {
        let start = Instant::now();

        match client.get(*url).send() {
            Ok(response) => {
                let status = response.status();
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;

                let result = if status.is_success() {
                    "SUCCESS"
                } else {
                    "HTTP ERROR"
                };

                println!(
                    "{} | {} | {:.2} ms | {}",
                    url, status, elapsed, result
                );

                writeln!(
                    file,
                    "\"{}\",\"{}\",{:.2},\"{}\"",
                    url, status, elapsed, result
                )?;
            }
            Err(error) => {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;

                println!("{} | REQUEST FAILED | {}", url, error);

                writeln!(
                    file,
                    "\"{}\",\"REQUEST FAILED\",{:.2},\"{}\"",
                    url, elapsed, error
                )?;
            }
        }
    }

    println!("\nCSV report saved as website_report.csv");

    Ok(())
}
