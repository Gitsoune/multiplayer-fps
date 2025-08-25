pub mod player_movement;
pub mod prompts;
pub mod structs;
use bevy::prelude::*;

mod client;
mod server;
mod show_fps;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        server::run();
    } else {
        client::run();
    }
}
