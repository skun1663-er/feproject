
use std::fs;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://easyhealths.com";

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; RustChangeDetector/1.0)")
        .build()?;

    println!("Checking website: {}", url);

    let response = client.get(url).send()?;
    let status = response.status();

    if !status.is_success() {
        println!("Website returned HTTP status: {}", status);
        return Ok(());
    }

    let current_content = response.text()?;
    let snapshot_file = "website_snapshot.html";

    if !std::path::Path::new(snapshot_file).exists() {
        fs::write(snapshot_file, &current_content)?;
        println!("First snapshot saved. Run again to detect changes.");
        return Ok(());
    }

    let previous_content = fs::read_to_string(snapshot_file)?;

    if current_content == previous_content {
        println!("No changes detected.");
    } else {
        println!("Website content has changed!");
        fs::write(snapshot_file, &current_content)?;
        println!("Updated snapshot saved.");
    }

    Ok(())
}
