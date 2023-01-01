use std::any::Any;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use bincode;
use shared::packet::Packet;
use uflow::server::Server;
use events::EventManager;

pub type Connection = SocketAddr;

pub struct PacketFromClientEvent {
    pub packet: Packet,
    pub conn: Connection,
}

pub struct NewConnectionEvent {
    pub conn: Connection,
}

/**
This handles all the networking for the server.
Server listens for connections and sends and receives packets
for each client.
 */
pub struct ServerNetworking {
    server_port: u16,
    net_server: Option<Server>,
    connections: Vec<Connection>,
}

impl ServerNetworking {
    pub fn new(server_port: u16) -> Self {
        Self {
            server_port,
            net_server: None,
            connections: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        // start listening for connections
        self.net_server = Some(Server::bind(format!("127.0.0.1:{}", self.server_port), Default::default()).unwrap());
    }

    pub fn on_event(&mut self, event: &Box<dyn Any>) {

    }

    pub fn update(&mut self, events: &mut EventManager) {
        /*// check for new connections
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
        }*/
        for event in self.net_server.as_mut().unwrap().step() {
            match event {
                uflow::server::Event::Connect(client_address) => {
                    println!("[{:?}] connected", client_address);
                    self.connections.push(client_address);
                }
                uflow::server::Event::Disconnect(client_address) => {
                    println!("[{:?}] disconnected", client_address);
                    self.connections.retain(|&x| x != client_address);
                }
                uflow::server::Event::Error(client_address, err) => {
                    panic!("[{:?}] error: {:?}", client_address, err);
                }
                uflow::server::Event::Receive(client_address, packet_data) => {
                    let packet: Packet = bincode::deserialize(&packet_data).unwrap();
                    events.push_event(Box::new(PacketFromClientEvent {
                        packet,
                        conn: client_address,
                    }));
                }
            }
        }

        self.net_server.as_mut().unwrap().flush();
    }

    pub fn send_packet(&mut self, packet: &Packet, conn: Connection) {
        let packet_data = bincode::serialize(packet).unwrap().into_boxed_slice();
        let mut client = self.net_server.as_mut().unwrap().client(&conn).unwrap().borrow_mut();

        client.send(packet_data, 0, uflow::SendMode::Reliable);

    }

    pub fn stop(&mut self) {
        // close all connections
        for connection in &mut self.connections {
            self.net_server.as_mut().unwrap().client(&connection).unwrap().borrow_mut().disconnect();
        }
    }
}