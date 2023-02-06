use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;

use crate::libraries::events::{Event, EventManager};
use crate::shared::enet_global::ENET_GLOBAL;
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use enet::{Address, BandwidthLimit, ChannelLimit, Host, PacketMode};

/**
This struct holds the address of a connection.
 */
#[derive(Clone, Eq)]
pub struct Connection {
    pub(super) address: Address,
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.ip().hash(state);
        self.address.port().hash(state);
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.address.ip() == other.address.ip() && self.address.port() == other.address.port()
    }
}

/**
This handles all the networking for the server.
server listens for connections and sends and receives packets
for each client.
 */
pub struct ServerNetworking {
    server_port: u16,
    net_server: Option<Host<()>>,
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
        //self.net_server = Some(server::bind(format!("127.0.0.1:{}", self.server_port), Default::default()).unwrap());
        let local_addr = Address::new(Ipv4Addr::LOCALHOST, self.server_port);

        self.net_server = Some(
            ENET_GLOBAL
                .create_host::<()>(
                    Some(&local_addr),
                    100,
                    ChannelLimit::Maximum,
                    BandwidthLimit::Unlimited,
                    BandwidthLimit::Unlimited,
                )
                .unwrap(),
        );
    }

    pub fn on_event(&mut self, event: &Event) {
        // handle new connection event
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            self.send_packet(&Packet::new(WelcomeCompletePacket {}), &event.conn);
            self.connections.push(event.conn.clone());
        }
    }

    pub fn update(&mut self, events: &mut EventManager) {
        while let Some(event) = self.net_server.as_mut().unwrap().service(0).unwrap() {
            match event {
                enet::Event::Connect(ref peer) => {
                    println!("[{:?}] connected", peer.address());
                    events.push_event(Event::new(NewConnectionEvent {
                        conn: Connection {
                            address: peer.address(),
                        },
                    }));
                }
                enet::Event::Disconnect(ref peer, ..) => {
                    println!("[{:?}] disconnected", peer.address());
                    self.connections.retain(|x| x.address != peer.address());
                }
                enet::Event::Receive {
                    ref packet,
                    ref sender,
                    ..
                } => {
                    let packet: Packet = bincode::deserialize(packet.data()).unwrap();
                    events.push_event(Event::new(PacketFromClientEvent {
                        packet,
                        conn: Connection {
                            address: sender.address(),
                        },
                    }));
                }
            }
        }
    }

    pub fn send_packet(&mut self, packet: &Packet, conn: &Connection) {
        let packet_data = bincode::serialize(packet).unwrap();
        let mut client = self
            .net_server
            .as_mut()
            .unwrap()
            .peers()
            .find(|x| x.address() == conn.address)
            .unwrap();
        client
            .send_packet(
                enet::Packet::new(&packet_data, PacketMode::ReliableSequenced).unwrap(),
                0,
            )
            .unwrap();
    }

    pub fn send_packet_to_all(&mut self, packet: &Packet) {
        let packet_data = bincode::serialize(packet).unwrap();
        for conn in &self.connections {
            let mut client = self
                .net_server
                .as_mut()
                .unwrap()
                .peers()
                .find(|x| x.address() == conn.address)
                .unwrap();
            client
                .send_packet(
                    enet::Packet::new(&packet_data, PacketMode::ReliableSequenced).unwrap(),
                    0,
                )
                .unwrap();
        }
    }

    pub fn stop(&mut self) {
        // close all connections
        for conn in self.net_server.as_mut().unwrap().peers() {
            conn.disconnect_now(0);
        }
    }
}

pub struct PacketFromClientEvent {
    pub packet: Packet,
    pub conn: Connection,
}

pub struct NewConnectionEvent {
    pub conn: Connection,
}
