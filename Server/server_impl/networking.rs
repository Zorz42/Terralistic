use std::any::Any;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use bincode;
use shared::packet::Packet;
use events::EventManager;

/**
This handles all the networking for one client.
 */
pub struct Connection {
    stream: TcpStream,
    id: u32,
}

impl Connection {
    pub fn send_packet(&mut self, packet: Packet) {
        let serialized_packet = bincode::serialize(&packet).unwrap();
        self.stream.write(&serialized_packet).unwrap();
    }
}

pub struct PacketFromClientEvent {
    pub packet: Packet,
    pub connection_id: u32,
}

pub struct NewConnectionEvent {
    pub connection_id: u32,
}

/**
This handles all the networking for the server.
Server listens for connections and sends and receives packets
for each client.
 */
pub struct ServerNetworking {
    server_port: u16,
    tcp_listener: Option<TcpListener>,
    connections: Vec<Connection>,
    current_id: u32,
}

impl ServerNetworking {
    pub fn new(server_port: u16) -> Self {
        Self {
            server_port,
            tcp_listener: None,
            connections: Vec::new(),
            current_id: 0,
        }
    }

    pub fn init(&mut self) {
        // start listening for connections
        self.tcp_listener = Some(TcpListener::bind(format!("127.0.0.1:{}", self.server_port)).unwrap());
        // set the listener to non-blocking
        self.tcp_listener.as_ref().unwrap().set_nonblocking(true).unwrap();
    }

    pub fn on_event(&mut self, event: &Box<dyn Any>) {

    }

    pub fn update(&mut self, events: &mut EventManager) {
        // check for new connections
        loop {
            let tcp_stream = self.tcp_listener.as_ref().unwrap().accept();
            if tcp_stream.is_err() {
                break;
            }
            let tcp_stream = tcp_stream.unwrap().0;
            // set the stream to non-blocking
            tcp_stream.set_nonblocking(true).unwrap();
            // add the connection to the list
            self.connections.push(Connection {
                stream: tcp_stream,
                id: self.current_id,
            });

            events.push_event(Box::new(NewConnectionEvent {
                connection_id: self.current_id,
            }));

            self.current_id += 1;

            let connection_ip = self.connections.last().unwrap().stream.peer_addr().unwrap().ip().to_string();
            println!("New connection: {}", connection_ip);
        }

        // flush the streams
        for connection in &mut self.connections {
            connection.stream.flush().unwrap();
        }

        // get all packets from the clients one by one
        // if a client disconnects, remove it from the list
        for i in 0..self.connections.len() {
            let mut buffer = [0; 1024];
            let bytes_read = self.connections[i].stream.read(&mut buffer);
            if bytes_read.is_err() {
                continue;
            }
            let bytes_read = bytes_read.unwrap();
            if bytes_read == 0 {
                // client disconnected
                let connection_ip = self.connections[i].stream.peer_addr().unwrap().ip().to_string();
                println!("Connection closed: {}", connection_ip);
                self.connections.remove(i);
                continue;
            }
            // deserialize the packet with bincode
            let packet: Packet = bincode::deserialize(&buffer[..bytes_read]).unwrap();

            events.push_event(Box::new(PacketFromClientEvent {
                packet,
                connection_id: self.connections[i].id,
            }));
        }
    }

    pub fn stop(&mut self) {
        // close all connections
        for connection in &mut self.connections {
            connection.stream.shutdown(std::net::Shutdown::Both).unwrap_or_default();
        }
    }

    pub fn get_connection_by_id(&mut self, id: u32) -> Option<&mut Connection> {
        for connection in &mut self.connections {
            if connection.id == id {
                return Some(connection);
            }
        }
        None
    }
}