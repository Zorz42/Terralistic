use std::any::Any;
use std::i32;

pub struct ClientNetworking {
    server_port: u16,
    server_address: String,
}

impl ClientNetworking {
    pub fn new(server_port: u16, server_address: String) -> ClientNetworking {
        ClientNetworking {
            server_port,
            server_address,
        }
    }
}