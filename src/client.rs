use std::io::{self, Write};
use std::net::UdpSocket;

pub fn run() {
    // Prompt for server IP and username
    print!("Enter IP Address: ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();

    print!("Enter Name: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("Starting...");

    // Connect to server via UDP
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind client socket");
    socket.connect(ip).expect("Could not connect to server");

    // Send username to server
    socket
        .send(username.as_bytes())
        .expect("Failed to send username");

    // Listen for server updates
    let mut buf = [0u8; 1024];
    let mut first = true;
    loop {
        match socket.recv(&mut buf) {
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buf[..size]);
                if first {
                    println!("Server: {}", msg);
                    first = false;
                }
            }
            Err(e) => {
                eprintln!("Error receiving from server: {}", e);
                break;
            }
        }
    }
}
