use std::io::{self, Write};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub fn prompt_server_addr() -> SocketAddr {
    loop {
        print!("Enter server IP:PORT (127.0.0.1:8080): ");
        io::stdout().flush().ok();
        let mut ip_input = String::new();
        if io::stdin().read_line(&mut ip_input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }
        let ip_input = ip_input.trim();
        if let Ok(addr) = ip_input
            .to_socket_addrs()
            .and_then(|mut iter| iter.next().ok_or(std::io::ErrorKind::InvalidInput.into()))
        {
            match UdpSocket::bind("0.0.0.0:0") {
                Ok(sock) => {
                    sock.set_read_timeout(Some(Duration::from_millis(500))).ok();
                    let _ = sock.send_to(b"ping", addr);
                    println!("Server address accepted: {}", addr);
                    return addr;
                }
                Err(e) => {
                    println!("Failed to bind UDP socket: {e}");
                    continue;
                }
            }
        } else {
            println!("Invalid IP:PORT. Please try again.");
        }
    }
}

pub fn prompt_username() -> String {
    loop {
        print!("Enter Name: ");
        io::stdout().flush().ok();
        let mut name = String::new();
        if io::stdin().read_line(&mut name).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }
        let name = name.trim();
        if name.is_empty() {
            println!("Name cannot be empty.");
            continue;
        }
        return name.to_string();
    }
}
