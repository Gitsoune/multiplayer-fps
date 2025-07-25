use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

const MAX_CLIENTS: usize = 32;
const TICK_RATE: Duration = Duration::from_millis(20); // ~50 FPS

#[derive(Debug, Clone)]
struct Player {
    username: String,
    position: (f32, f32),
    addr: SocketAddr,
    // Add more fields as needed (direction, health, etc.)
}

#[derive(Debug, Clone)]
struct Maze {
    // Define your maze structure here
    // For example: walls, size, level number, etc.
}

pub fn run() {
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("Could not bind server socket");
    socket
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    println!("Server running on 0.0.0.0:8080");

    let mut clients: HashMap<SocketAddr, Player> = HashMap::new();
    let mut maze = Maze {
        // Initialize your maze here (level 1)
    };

    let mut buf = [0u8; 1024];
    let mut last_tick = Instant::now();

    loop {
        // Receive messages from clients
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                // Parse message, handle new connections, updates, etc.
                // Example: handle join, movement, etc.
                // If new client:
                if !clients.contains_key(&addr) && clients.len() < MAX_CLIENTS {
                    // Parse username from message
                    let username = String::from_utf8_lossy(&buf[..size]).to_string();
                    clients.insert(
                        addr,
                        Player {
                            username: username.clone(),
                            position: (0.0, 0.0), // spawn position
                            addr,
                        },
                    );
                    println!("New client: {} @ {}", username, addr);
                } else {
                    // Handle movement or other updates
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data received, continue
            }
            Err(e) => {
                eprintln!("Error receiving from socket: {}", e);
            }
        }

        // Game tick: update state and broadcast to clients
        if last_tick.elapsed() >= TICK_RATE {
            // Serialize game state (players, maze, etc.)
            let state = format!("STATE: {:?}", clients); // Replace with proper serialization

            for player in clients.values() {
                let _ = socket.send_to(state.as_bytes(), player.addr);
            }

            last_tick = Instant::now();
        }

        // TODO: Handle level switching, client disconnects, etc.
    }
}
