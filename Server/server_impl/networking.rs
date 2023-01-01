use std::any::Any;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use bincode;
use shared::packet::{Packet, WelcomeCompletePacket};
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
        // handle new connection event
        if let Some(event) = event.downcast_ref::<NewConnectionEvent>() {
            self.send_packet(&Packet::new(WelcomeCompletePacket {}), event.conn);
            self.connections.push(event.conn);
        }
    }

    pub fn update(&mut self, events: &mut EventManager) {
        for event in self.net_server.as_mut().unwrap().step() {
            match event {
                uflow::server::Event::Connect(client_address) => {
                    println!("[{:?}] connected", client_address);
                    events.push_event(Box::new(NewConnectionEvent {
                        conn: client_address,
                    }));
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

        println!("sending packet with size {}", packet_data.len());
        client.send(packet_data, 0, uflow::SendMode::TimeSensitive);
    }

    pub fn stop(&mut self) {
        // close all connections
        for connection in &mut self.connections {
            self.net_server.as_mut().unwrap().client(&connection).unwrap().borrow_mut().disconnect();
        }
    }
}