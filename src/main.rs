use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    println!("=== Rust Port Scanner ===\n");

    let args: Vec<String> = env::args().collect();

    let target = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let start_port = match parse_port(args.get(2), 8000) {
        Ok(port) => port,
        Err(message) => exit_with_error(&message),
    };

    let end_port = match parse_port(args.get(3), 8010) {
        Ok(port) => port,
        Err(message) => exit_with_error(&message),
    };

    if start_port > end_port {
        exit_with_error("ang start port ay dapat mas maliit o kapantay ng end port");
    }

    println!("Target: {target}");
    println!("Port Range: {start_port} - {end_port}");
    println!("Scanning...\n");

    let mut open_ports = Vec::new();

    for port in start_port..=end_port {
        let address = format!("{target}:{port}");

        let is_open = match address.to_socket_addrs() {
            Ok(addresses) => addresses.into_iter().any(|socket_address| {
                TcpStream::connect_timeout(&socket_address, Duration::from_secs(1)).is_ok()
            }),
            Err(error) => {
                eprintln!("[!] Hindi ma-resolve ang {target}: {error}");
                false
            }
        };

        if is_open {
            println!("[+] {address} OPEN");
            open_ports.push(port);
        } else {
            println!("[-] {address} CLOSED");
        }
    }

    println!("\n=== Scan Complete ===");

    if open_ports.is_empty() {
        println!("Open ports: None found");
    } else {
        let ports = open_ports
            .iter()
            .map(u16::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        println!("Open ports: {ports}");
    }
}

fn parse_port(value: Option<&String>, default: u16) -> Result<u16, String> {
    match value {
        None => Ok(default),
        Some(value) => value
            .parse::<u16>()
            .map_err(|_| format!("invalid port '{value}'; gumamit ng number 0–65535")),
    }
}

fn exit_with_error(message: &str) -> ! {
    eprintln!("Error: {message}");
    eprintln!("Usage: rust-portscan [TARGET] [START_PORT] [END_PORT]");
    std::process::exit(2);
}
