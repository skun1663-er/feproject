
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::time::Duration;
use reqwest::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let website = "https://easyhealths.com";
    let base_url = Url::parse(website)?;

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; RustLinkChecker/1.0)")
        .build()?;

    println!("Scanning links on: {}", website);

    let response = client.get(website).send()?;

    if !response.status().is_success() {
        println!("Could not scan page: HTTP {}", response.status());
        return Ok(());
    }

    let html = response.text()?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a[href]")?;
    let mut links = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = base_url.join(href) {
                if url.scheme() == "http" || url.scheme() == "https" {
                    links.insert(url.to_string());
                }
            }
        }
    }

    println!("Found {} unique links.\n", links.len());

    for link in links {
        match client.get(&link).send() {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    println!("[OK] {} - {}", status, link);
                } else {
                    println!("[HTTP ERROR] {} - {}", status, link);
                }
            }
            Err(error) => {
                println!("[REQUEST FAILED] {} - {}", link, error);
            }
        }
    }

    Ok(())
}
