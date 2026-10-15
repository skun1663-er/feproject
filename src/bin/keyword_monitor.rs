
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://easyhealths.com";

    let keywords = vec![
        "example",
        "domain",
        "website",
        "rust",
        "programming",
        "tutorial",
        "doctor",
        "health",
        "booking",
        "appointment",
    ];

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; RustKeywordMonitor/1.0)")
        .build()?;

    println!("Checking keywords on: {}", url);

    let response = client.get(url).send()?;

    if !response.status().is_success() {
        println!("HTTP error: {}", response.status());
        return Ok(());
    }

    let content = response.text()?.to_lowercase();

    for keyword in &keywords {
        if content.contains(&keyword.to_lowercase()) {
            println!("[FOUND] {}", keyword);
        } else {
            println!("[MISSING] {}", keyword);
        }
    }

    Ok(())
}
