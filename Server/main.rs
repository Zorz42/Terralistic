#![allow(non_snake_case)]

use terralistic_server::Server;

fn main() {
    let mut server = Server::new();
    server.start();
}
