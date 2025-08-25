use bevy::prelude::*;
use std::io::{self, Write};

// Minimal Bevy client with an FPS overlay (works with Bevy 0.16.1).

#[derive(Component)]
struct FpsText;

// Small resource that stores the last-displayed integer FPS to avoid
// reformatting and updating the UI every frame when the value hasn't changed.
#[derive(Resource)]
struct FpsCache {
    last: i32,
}

pub fn run() {
    use std::net::ToSocketAddrs;
    use std::net::UdpSocket;
    use std::time::Duration;

    // Prompt for server IP:PORT and validate
    let server_addr = loop {
        print!("Enter server IP:PORT (e.g. 127.0.0.1:8080): ");
        io::stdout().flush().ok();
        let mut ip_input = String::new();
        if io::stdin().read_line(&mut ip_input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }
        let ip_input = ip_input.trim();
        // Try to parse as SocketAddr
        if let Ok(addr) = ip_input
            .to_socket_addrs()
            .and_then(|mut iter| iter.next().ok_or(std::io::ErrorKind::InvalidInput.into()))
        {
            // Try to send a test UDP packet
            match UdpSocket::bind("0.0.0.0:0") {
                Ok(sock) => {
                    sock.set_read_timeout(Some(Duration::from_millis(500))).ok();
                    let _ = sock.send_to(b"ping", addr);
                    // Optionally, wait for a response (not required for now)
                    // let mut buf = [0u8; 16];
                    // if sock.recv_from(&mut buf).is_ok() { ... }
                    println!("Server address accepted: {}", addr);
                    break addr;
                }
                Err(e) => {
                    println!("Failed to bind UDP socket: {e}");
                    continue;
                }
            }
        } else {
            println!("Invalid IP:PORT. Please try again.");
        }
    };

    // Prompt for username only after IP is validated
    let username = loop {
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
        break name.to_string();
    };

    println!(
        "Starting Bevy UI Client... (server: {}, username: {})",
        server_addr, username
    );

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FpsCache { last: -1 })
        .add_systems(Startup, setup)
        .add_systems(Update, show_fps)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera (example uses the unit `Camera2d` marker)
    commands.spawn(Camera2d);

    // Spawn a parent Text with one section "FPS: " and a child TextSpan which
    // we will update each frame. This mirrors the Bevy example structure.
    commands
        .spawn((
            // Parent text with the label
            Text::new("FPS: "),
            // Styling for the parent text
            TextFont {
                font: default(),
                font_size: 42.0,
                ..default()
            },
        ))
        .with_child((
            // Child span that will contain the numeric FPS value.
            TextSpan::default(),
            TextFont {
                font: default(),
                font_size: 33.0,
                ..default()
            },
            // Marker so we can query the span later
            FpsText,
        ));
}

fn show_fps(
    time: Res<Time>,
    mut cache: ResMut<FpsCache>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    // Compute integer FPS and only update UI when it changes.
    let fps_i = (1.0 / dt).round() as i32;
    if fps_i == cache.last {
        return;
    }
    cache.last = fps_i;

    let display = fps_i.to_string();
    for mut span in &mut query {
        **span = display.clone();
    }
}
