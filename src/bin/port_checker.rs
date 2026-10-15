
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    let host = "127.0.0.1";

    let ports = vec![
        80, 443, 3000, 5000, 8000, 8080,
    ];

    let timeout = Duration::from_secs(2);

    println!("Checking TCP ports on {}\n", host);

    for port in ports {
        let address = format!("{}:{}", host, port);

        let result = address.parse::<SocketAddr>();

        match result {
            Ok(socket_address) => {
                match TcpStream::connect_timeout(
                    &socket_address,
                    timeout,
                ) {
                    Ok(_) => {
                        println!("[OPEN] Port {}", port);
                    }
                    Err(_) => {
                        println!("[CLOSED or FILTERED] Port {}", port);
                    }
                }
            }
            Err(error) => {
                println!("[ERROR] Port {}: {}", port, error);
            }
        }
    }

    println!("\nPort check completed.");
}
