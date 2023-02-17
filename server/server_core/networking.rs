use core::hash::{Hash, Hasher};
use std::net::Ipv4Addr;

use crate::libraries::events::{Event, EventManager};
use crate::shared::enet_global::ENET_GLOBAL;
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use anyhow::{anyhow, Result};
use enet::{Address, BandwidthLimit, ChannelLimit, Host, PacketMode};

/// This struct holds the address of a connection.
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

/// This handles all the networking for the server.
/// server listens for connections and sends and receives packets
/// for each client.
pub struct ServerNetworking {
    server_port: u16,
    net_server: Option<Host<()>>,
    connections: Vec<Connection>,
}

impl ServerNetworking {
    pub const fn new(server_port: u16) -> Self {
        Self {
            server_port,
            net_server: None,
            connections: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        // start listening for connections
        let local_addr = Address::new(Ipv4Addr::LOCALHOST, self.server_port);

        self.net_server = Some(ENET_GLOBAL.create_host::<()>(
            Some(&local_addr),
            100,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )?);

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event) -> Result<()> {
        // handle new connection event
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            self.send_packet(&Packet::new(WelcomeCompletePacket {})?, &event.conn)?;
            self.connections.push(event.conn.clone());
        }
        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager) -> Result<()> {
        while let Some(event) = self
            .net_server
            .as_mut()
            .ok_or_else(|| anyhow!("Enet server not initialized yet"))?
            .service(0)?
        {
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
                    let packet: Packet = bincode::deserialize(packet.data())?;
                    events.push_event(Event::new(PacketFromClientEvent {
                        packet,
                        conn: Connection {
                            address: sender.address(),
                        },
                    }));
                }
            }
        }
        Ok(())
    }

    pub fn send_packet(&mut self, packet: &Packet, conn: &Connection) -> Result<()> {
        let packet_data = bincode::serialize(packet)?;
        let mut client = self
            .net_server
            .as_mut()
            .ok_or_else(|| anyhow!("Enet server not initialized yet"))?
            .peers()
            .find(|x| x.address() == conn.address)
            .ok_or_else(|| anyhow::anyhow!("Client not found"))?;
        client.send_packet(
            enet::Packet::new(&packet_data, PacketMode::ReliableSequenced)?,
            0,
        )?;
        Ok(())
    }

    pub fn send_packet_to_all(&mut self, packet: &Packet) -> Result<()> {
        let packet_data = bincode::serialize(packet)?;
        for conn in &self.connections {
            let mut client = self
                .net_server
                .as_mut()
                .ok_or_else(|| anyhow!("Enet server not initialized yet"))?
                .peers()
                .find(|x| x.address() == conn.address)
                .ok_or_else(|| anyhow!("Client not found on send_packet_to_all"))?;
            client.send_packet(
                enet::Packet::new(&packet_data, PacketMode::ReliableSequenced)?,
                0,
            )?;
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        // close all connections
        for conn in self
            .net_server
            .as_mut()
            .ok_or_else(|| anyhow!("Enet server not initialized yet"))?
            .peers()
        {
            conn.disconnect_now(0);
        }
        Ok(())
    }
}

pub struct PacketFromClientEvent {
    pub packet: Packet,
    pub conn: Connection,
}

pub struct NewConnectionEvent {
    pub conn: Connection,
}