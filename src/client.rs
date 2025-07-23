use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::io::{self, Write};
use std::net::UdpSocket;
use std::time::Duration;

use crate::maze::Maze;

const BUFFER_SIZE: usize = 1024;

pub fn start_client() {
    // Get server address and username from user
    print!("Enter server IP (127.0.0.1:9000): ");
    io::stdout().flush().unwrap();
    let mut server_addr = String::new();
    io::stdin().read_line(&mut server_addr).unwrap();
    let server_addr = server_addr.trim();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    // Create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind client UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    // Send JOIN message
    let join_msg = format!("JOIN:{}", username);
    socket
        .send_to(join_msg.as_bytes(), server_addr)
        .expect("Failed to send JOIN");

    // Wait for response
    let mut buf = [0u8; BUFFER_SIZE];
    match socket.recv_from(&mut buf) {
        Ok((size, _)) => {
            let resp = String::from_utf8_lossy(&buf[..size]);
            if resp == "WELCOME" {
                println!("Connected to server!");
            } else {
                println!("Server response: {}", resp);
                return;
            }
        }
        Err(e) => {
            println!("Failed to receive response: {}", e);
            return;
        }
    }

    // Initialize SDL2 window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Maze Wars Client", 1200, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let maze = Maze::level(3); // Change to 2 or 3 for harder levels

    let window_width = 1200;
    let window_height = 800;
    let cell_size = 10;

    let maze_pixel_width = maze.width * cell_size;
    let maze_pixel_height = maze.height * cell_size;

    let offset_x = ((window_width as isize - maze_pixel_width as isize) / 2) as i32;
    let offset_y = ((window_height as isize - maze_pixel_height as isize) / 2) as i32;

    // Main loop (just clears screen for now)
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw maze centered
        for y in 0..maze.height {
            for x in 0..maze.width {
                if maze.grid[y][x] == 1 {
                    canvas.set_draw_color(Color::RGB(100, 100, 100));
                    let _ = canvas.fill_rect(sdl2::rect::Rect::new(
                        offset_x + (x as i32) * (cell_size as i32),
                        offset_y + (y as i32) * (cell_size as i32),
                        cell_size as u32,
                        cell_size as u32,
                    ));
                }
            }
        }

        canvas.present();
    }
}
