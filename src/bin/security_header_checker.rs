
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://easyhealths.com";

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; RustSecurityChecker/1.0)")
        .build()?;

    println!("Checking security headers: {}\n", url);

    let response = client.get(url).send()?;

    println!("HTTP Status: {}\n", response.status());

    let headers = response.headers();

    let security_headers = [
        (
            "Strict-Transport-Security",
            "Enforces HTTPS in supported browsers",
        ),
        (
            "Content-Security-Policy",
            "Helps restrict allowed content sources",
        ),
        (
            "X-Content-Type-Options",
            "Helps prevent MIME-type sniffing",
        ),
        (
            "X-Frame-Options",
            "Helps protect against clickjacking",
        ),
        (
            "Referrer-Policy",
            "Controls referrer information",
        ),
        (
            "Permissions-Policy",
            "Controls access to selected browser features",
        ),
    ];

    let mut present = 0;
    let total = security_headers.len();

    for (name, purpose) in security_headers {
        match headers.get(name) {
            Some(value) => {
                present += 1;
                println!("[PRESENT] {}", name);
                println!("  Value: {:?}", value);
                println!("  Purpose: {}\n", purpose);
            }
            None => {
                println!("[MISSING] {}", name);
                println!("  Purpose: {}\n", purpose);
            }
        }
    }

    println!("Headers present: {}/{}", present, total);

    Ok(())
}
