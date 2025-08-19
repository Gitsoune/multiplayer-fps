use std::io::{self, Write};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

// Minimal terminal-only UDP client. Keeps networking working while you iterate
// on the Bevy UI separately. It connects to the server, sends a username and
// prints server messages without attempting to use Bevy APIs.

pub fn run() {
    // Prompt for server IP and username (minimal)
    print!("Enter IP Address (e.g. 127.0.0.1:8080): ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim().to_string();

    print!("Enter Name: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    println!("Starting...");

    // create and connect UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind client socket");
    socket
        .connect(&ip)
        .unwrap_or_else(|e| panic!("Could not connect to server {}: {}", ip, e));

    socket
        .send(username.as_bytes())
        .expect("Failed to send username");

    let sock_bg = socket.try_clone().expect("failed to clone socket");
    thread::spawn(move || {
        let mut buf = [0u8; 2048];
        let mut last_printed = String::new();
        loop {
            match sock_bg.recv(&mut buf) {
                Ok(size) => {
                    let msg = String::from_utf8_lossy(&buf[..size]).to_string();
                    // Only print when the server message changes (suppress repeated STATE spam)
                    if !msg.is_empty() && msg != last_printed {
                        println!("Server: {}", msg);
                        last_printed = msg;
                    }
                }
                Err(_) => {
                    // continue
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Main thread can accept simple user commands to send to server
    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_ok() {
            let line = line.trim();
            if line == "quit" || line == "exit" {
                println!("Exiting client");
                break;
            }
            if !line.is_empty() {
                let _ = socket.send(line.as_bytes());
            }
        }
    }
}
