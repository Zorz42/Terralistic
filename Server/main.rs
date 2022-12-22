#![allow(non_snake_case)]

use crate::server::Server;

mod server;
mod server_module;

fn main() {
    let mut server = Server::new();
    server.start();
}
