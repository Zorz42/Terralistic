use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::libraries::net::{LogLevel, Logger, PacketServer, ServerEvent};
use crate::server::server_core::print_to_console;
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use crate::shared::players::NamePacket;
use crate::shared::versions::{VersionPacket, VERSION};

pub use crate::libraries::net::{BindAddress, Connection};

pub enum SendTarget {
    All,
    Connection(Connection),
    AllExcept(Connection),
}

/// The game's half of the server protocol: who is let in, and on what terms.
///
/// The socket, the thread and the packets themselves are `libraries::net::PacketServer`.
/// What is left here is the handshake - a version that has to match, then a name - and the
/// notion of a connection being *welcomed*, which is what `SendTarget::All` means and is not
/// the same as a peer having completed a TCP connection.
pub struct ServerNetworking {
    server: PacketServer,
    /// Peers that have been welcomed. A peer mid-handshake is connected but not one of these.
    connections: Vec<Connection>,
    connection_names: HashMap<Connection, String>,
    /// Peers that sent a version this build accepts. A `NamePacket` from anyone else means a
    /// client too old to send a version at all.
    accepted_peers: HashSet<Connection>,
    /// The simulation tick the server is on, so a client joining mid-game is told where the
    /// clock has got to rather than starting its own from zero.
    current_tick: u64,
}

impl ServerNetworking {
    #[must_use]
    pub fn new(server_port: u16, bind_address: BindAddress) -> Self {
        Self {
            server: PacketServer::new(server_port, bind_address, server_console_logger()),
            current_tick: 0,
            connections: Vec::new(),
            connection_names: HashMap::new(),
            accepted_peers: HashSet::new(),
        }
    }

    /// True once the networking thread has actually bound the port and is accepting.
    ///
    /// Only the tests need to know: the game's own server has a world to load before anyone
    /// can connect, which is a far longer wait than the bind.
    #[cfg(test)]
    #[must_use]
    pub fn is_listening(&self) -> bool {
        self.server.is_listening()
    }

    #[must_use]
    pub fn get_connection_name(&self, conn: &Connection) -> String {
        let unknown = "Unknown".to_owned();
        self.connection_names.get(conn).unwrap_or(&unknown).clone()
    }

    pub const fn set_current_tick(&mut self, tick: u64) {
        self.current_tick = tick;
    }

    pub fn init(&mut self) {
        self.server.listen();

        if self.server.bind_address() == BindAddress::AllInterfaces {
            print_to_console(
                "this server is reachable from the network and has no authentication, so anyone who can reach this port can join and change the world",
                1,
            );
        }
    }

    pub fn on_event(&mut self, event: &Event, events: &mut EventManager) -> Result<()> {
        // handle new connection event
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            self.connection_names.insert(event.conn.clone(), event.name.clone());

            self.send_packet(&Packet::new(WelcomeCompletePacket { server_tick: self.current_tick })?, SendTarget::Connection(event.conn.clone()))?;
            self.connections.push(event.conn.clone());
            events.push_event(Event::new(NewConnectionWelcomedEvent { conn: event.conn.clone() }));
        }

        if let Some(event) = event.downcast::<DisconnectEvent>() {
            self.connections.retain(|x| x != &event.conn);
        }

        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager) -> Result<()> {
        for net_event in self.server.poll()? {
            match net_event {
                // a peer that has connected has said nothing yet, so there is nothing to
                // tell the rest of the server about until the handshake gets somewhere
                ServerEvent::Connected(_conn) => {}
                ServerEvent::Disconnected(conn) => {
                    self.accepted_peers.remove(&conn);
                    events.push_event(Event::new(DisconnectEvent { conn }));
                }
                ServerEvent::Received { conn, packet } => self.handle_packet(conn, packet, events)?,
            }
        }

        Ok(())
    }

    /// The handshake, one packet at a time.
    ///
    /// The version goes first and is checked before anything else is believed, because packet
    /// ids are a hash of the rust type: a client built against a different protocol does not
    /// fail to understand this server so much as fail to recognise it at all, and would
    /// otherwise hang with no explanation.
    fn handle_packet(&mut self, conn: Connection, packet: Packet, events: &mut EventManager) -> Result<()> {
        if let Some(version) = packet.try_deserialize::<VersionPacket>() {
            if version.version == VERSION {
                self.accepted_peers.insert(conn);
            } else {
                print_to_console(&format!("[{conn}] refused: it is version {}, this server is {VERSION}", version.version), 1);
                self.server.disconnect(&conn)?;
            }
            return Ok(());
        }

        if let Some(name) = packet.try_deserialize::<NamePacket>() {
            if !self.accepted_peers.contains(&conn) {
                print_to_console(&format!("[{conn}] refused: it did not send a version, so it is older than {VERSION}"), 1);
                self.server.disconnect(&conn)?;
                return Ok(());
            }

            print_to_console(&format!("[{:?}] joined the game", name.name), 0);
            events.push_event(Event::new(NewConnectionEvent { conn, name: name.name }));
            return Ok(());
        }

        events.push_event(Event::new(PacketFromClientEvent { packet, conn }));
        Ok(())
    }

    pub fn send_packet(&mut self, packet: &Packet, target: SendTarget) -> Result<()> {
        match target {
            SendTarget::All => {
                let connections = self.connections.clone();
                self.server.send(packet, &connections)
            }
            SendTarget::Connection(conn) => self.server.send(packet, &[conn]),
            SendTarget::AllExcept(conn) => {
                let connections: Vec<Connection> = self.connections.iter().filter(|c| **c != conn).cloned().collect();
                self.server.send(packet, &connections)
            }
        }
    }

    pub fn stop(&mut self, events: &mut EventManager) -> Result<()> {
        self.server.stop()?;

        for conn in &self.connections {
            events.push_event(Event::new(DisconnectEvent { conn: conn.clone() }));
        }

        Ok(())
    }
}

/// Sends the transport's commentary to the server console, which is where the rest of the
/// server's output goes.
fn server_console_logger() -> Logger {
    std::sync::Arc::new(|level, message| {
        print_to_console(
            message,
            match level {
                LogLevel::Info => 0,
                LogLevel::Warning => 1,
                LogLevel::Error => 2,
            },
        );
    })
}

pub struct PacketFromClientEvent {
    pub packet: Packet,
    pub conn: Connection,
}

pub struct NewConnectionEvent {
    pub conn: Connection,
    pub name: String,
}

pub struct DisconnectEvent {
    pub conn: Connection,
}

pub struct NewConnectionWelcomedEvent {
    pub conn: Connection,
}
