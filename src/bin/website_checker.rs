
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = match args.get(1) {
        Some(url) => url,
        None => {
            println!(
                "Usage: cargo run --bin website_checker -- <https_url>"
            );
            return;
        }
    };

    if !url.starts_with("https://") {
        println!("Please provide an HTTPS URL.");
        return;
    }

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            println!("Could not create HTTP client: {}", error);
            return;
        }
    };

    println!("Checking: {}", url);

    match client.get(url).send() {
        Ok(response) => {
            let status = response.status();

            println!("HTTP status: {}", status);

            if status.is_success() {
                println!("Website responded successfully.");
            } else {
                println!("Website responded, but returned an error status.");
            }
        }
        Err(error) => {
            println!("Website request failed: {}", error);
        }
    }
}
