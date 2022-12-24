use std::io::{Read, Write};
use std::net::TcpStream;
use bincode;
use shared::packet::Packet;

/**
 This handles all the networking for the client
 */
pub struct ClientNetworking {
    server_port: u16,
    server_address: String,
    tcp_stream: Option<TcpStream>,
}

impl ClientNetworking {
    pub fn new(server_port: u16, server_address: String) -> ClientNetworking {
        ClientNetworking {
            server_port,
            server_address,
            tcp_stream: None,
        }
    }

    pub fn init(&mut self) {
        // connect to the server
        self.tcp_stream = Some(TcpStream::connect(format!("{}:{}", self.server_address, self.server_port)).unwrap());
        // set the stream to non-blocking
        self.tcp_stream.as_ref().unwrap().set_nonblocking(true).unwrap();
    }

    pub fn update(&mut self) {
        // flush the stream
        self.tcp_stream.as_ref().unwrap().flush().unwrap();
        // get all packets from the server one by one
        loop {
            let mut buffer = [0; 1024];
            let bytes_read = self.tcp_stream.as_ref().unwrap().read(&mut buffer);
            if bytes_read.is_err() {
                break;
            }
            let bytes_read = bytes_read.unwrap();
            if bytes_read == 0 {
                break;
            }
            // deserialize the packet with bincode
            let packet: Packet = bincode::deserialize(&buffer[..bytes_read]).unwrap();
        }
    }

    pub fn stop(&mut self) {
        // disconnect the socket
        self.tcp_stream.as_ref().unwrap().shutdown(std::net::Shutdown::Both).unwrap();
    }
}