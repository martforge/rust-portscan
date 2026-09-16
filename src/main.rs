use std::net::TcpStream;

fn main() {

    // Title
    println!("=== Rust Port Scanner ===");

    // Variables
    let target = "127.0.0.1";

    // Print for readability
    println!("Target: {target}");

    for port in 8000..8010 {
        let address = format!("{target}:{port}");
        println!("Checking port: {address}");

        let result = TcpStream::connect(&address);

        match result {
            Ok(_) => {
                println!("[+] {address} OPEN");
            }
            Err(_) => {
                println!("[-] {address} CLOSED");
            }
        }
    }
}


