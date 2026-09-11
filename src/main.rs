use std::net::TcpStream;


fn main() {

    // Title
    println!("Rust Port Scanner");
    
    // Variables
    let target = "127.0.0.1";
    let port: u16 = 8080;
    
    //Print for readability
    println!("Target: {target}");
    println!("Port: {port}");
    

    let address = format!("{target}:{port}");
    println!("Address: {address}");
    let result = TcpStream::connect(address);
    match result {
        Ok(_) =>{
            println!("OPEN");
        }
        Err(_) =>{
            println!("CLOSED");
        }
        
    }
}

