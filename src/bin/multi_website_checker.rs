
use std::time::{Duration, Instant};

fn main() {
    let urls = vec![
        "https://example.com",
        "https://www.rust-lang.org",
        "https://www.wikipedia.org",
    ];

    let client = match reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(10))
    .user_agent(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
         AppleWebKit/537.36 (KHTML, like Gecko) \
         Chrome/130.0.0.0 Safari/537.36"
    )
    .build()
{
    Ok(client) => client,
    Err(error) => {
        println!("Could not create HTTP client: {}", error);
        return;
    }
};

    println!("=== Multi-Website Report ===");

    let mut successful = 0;
    let mut failed = 0;

    for url in &urls {
        let start = Instant::now();

        match client.get(*url).send() {
            Ok(response) => {
                let status = response.status();
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;

                println!("\nWebsite: {}", url);
                println!("HTTP status: {}", status);
                println!("Response time: {:.2} ms", elapsed);

                if status.is_success() {
                    println!("Result: SUCCESS");
                    successful += 1;
                } else {
                    println!("Result: HTTP ERROR");
                    failed += 1;
                }
            }
            Err(error) => {
                println!("\nWebsite: {}", url);
                println!("Result: REQUEST FAILED");
                println!("Reason: {}", error);
                failed += 1;
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total websites: {}", urls.len());
    println!("Successful: {}", successful);
    println!("Failed: {}", failed);
}
