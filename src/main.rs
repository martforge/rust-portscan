use std::net::TcpStream;
use std::time::Duration;
use std::env;

fn main() {
    // Title
    println!("=== Rust Port Scanner ===\n");

    // Get command-line arguments or use defaults
    let args: Vec<String> = env::args().collect();
    
    let target = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("127.0.0.1")
    };

    let start_port: u16 = if args.len() > 2 {
        args[2].parse().unwrap_or(8000)
    } else {
        8000
    };

    let end_port: u16 = if args.len() > 3 {
        args[3].parse().unwrap_or(8010)
    } else {
        8010
    };

    // Print scan info
    println!("Target: {}", target);
    println!("Port Range: {} - {}", start_port, end_port);
    println!("Scanning...\n");

    let mut open_ports = Vec::new();

    for port in start_port..=end_port {
        let address = format!("{}:{}", target, port);
        
        // Set timeout to 1 second for faster scanning
        match TcpStream::connect_timeout(
            &address.parse().unwrap(),
            Duration::from_secs(1),
        ) {
            Ok(_) => {
                println!("[+] {} OPEN", address);
                open_ports.push(port);
            }
            Err(_) => {
                println!("[-] {} CLOSED", address);
            }
        }
    }

    // Summary
    println!("\n=== Scan Complete ===");
    println!("Open ports: {}", if open_ports.is_empty() {
        "None found".to_string()
    } else {
        open_ports.iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    });
}
