use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

const MAX_CLIENTS: usize = 10;
const BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct Player {
    pub username: String,
    pub addr: SocketAddr,
}

pub struct Server {
    socket: UdpSocket,
    players: HashMap<SocketAddr, Player>,
}

impl Server {
    pub fn new(bind_addr: &str) -> Self {
        let socket = UdpSocket::bind(bind_addr).expect("Could not bind UDP socket");
        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        Server {
            socket,
            players: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Server running on {}", self.socket.local_addr().unwrap());
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    let msg = String::from_utf8_lossy(&buf[..size]);
                    self.handle_message(msg.to_string(), addr);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Timeout, continue
                }
                Err(e) => {
                    eprintln!("Error receiving UDP packet: {}", e);
                }
            }
        }
    }

    fn handle_message(&mut self, msg: String, addr: SocketAddr) {
        if msg.starts_with("JOIN:") {
            let username = msg[5..].trim().to_string();
            if self.players.len() < MAX_CLIENTS {
                self.players.insert(
                    addr,
                    Player {
                        username: username.clone(),
                        addr,
                    },
                );
                println!("Player joined: {} from {}", username, addr);
                let _ = self.socket.send_to(b"WELCOME", addr);
            } else {
                let _ = self.socket.send_to(b"SERVER FULL", addr);
            }
        }
        // Future: handle game state, movement, etc.
    }
}

// Entry point for running the server
pub fn start_server(bind_addr: &str) {
    let mut server = Server::new(bind_addr);
    server.run();
}
