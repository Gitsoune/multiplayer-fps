use crate::client::start_client;
use crate::server::start_server;
use std::env;

mod client;
mod maze;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        // Run only the server
        start_server("0.0.0.0:8080");
    } else {
        // Run only the client
        start_client();
    }
}
