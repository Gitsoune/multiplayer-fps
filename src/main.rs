use bevy::prelude::*;

mod client;
mod server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        server::run();
    } else {
        client::run();
    }
}
