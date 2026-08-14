use anyhow::{anyhow, Result};

use crate::libraries::events;
use crate::libraries::events::EventManager;
use crate::libraries::net::{no_logging, ClientError, PacketClient};
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use crate::shared::players::NamePacket;
use crate::shared::versions::VersionPacket;

/// This event is called, when the client has received a welcome packet.
pub struct WelcomePacketEvent {
    pub packet: Packet,
}

/// The game's half of the client protocol.
///
/// The socket, the thread and the handshake *phase* are `libraries::net::PacketClient`.
/// What is left here is what the game says and means: the version and the name go out as the
/// greeting, `WelcomeCompletePacket` is what ends the welcome, and everything that arrives
/// before it becomes a `WelcomePacketEvent` rather than an ordinary packet.
pub struct ClientNetworking {
    client: PacketClient,
}

impl ClientNetworking {
    #[must_use]
    pub fn new(server_port: u16, server_address: String) -> Self {
        // the client has no console of its own to write a transport's commentary to
        Self {
            client: PacketClient::new(server_address, server_port, no_logging()),
        }
    }

    pub fn init(&mut self, name: String) -> Result<()> {
        // The version goes first, so a mismatched server can say so instead of silently
        // failing to understand everything that follows.
        let greeting = vec![Packet::new(VersionPacket::current())?, Packet::new(NamePacket { name })?];

        self.client.connect(greeting, Packet::is::<WelcomeCompletePacket>)
    }

    pub fn update(&mut self, events: &mut EventManager) -> Result<()> {
        for received in self.client.poll()? {
            // A welcome packet is the world arriving, and the game drains those into
            // `pre_events` before it starts playing. Everything after is ordinary traffic and
            // goes to whichever subsystem recognises it.
            if received.during_handshake {
                events.push_event(events::Event::new(WelcomePacketEvent { packet: received.packet }));
            } else {
                events.push_event(events::Event::new(received.packet));
            }
        }

        if let Some(error) = self.client.take_error() {
            return Err(anyhow!(Self::explain(&error)));
        }

        Ok(())
    }

    /// Says what a transport failure means for somebody trying to join a world.
    ///
    /// The transport knows the socket shut; only this side knows that a server which drops a
    /// connection mid-handshake is a server that has refused this client, which is the whole
    /// case the version check exists for.
    fn explain(error: &ClientError) -> String {
        match error {
            ClientError::NotAccepted => "could not connect to the server: it did not accept the connection".to_owned(),
            ClientError::ClosedDuringHandshake => "the server closed the connection during the handshake, so it refused to let this client join".to_owned(),
            ClientError::Failed(message) => message.clone(),
        }
    }

    pub fn send_packet(&mut self, packet: Packet) -> Result<()> {
        self.client.send(packet)
    }

    pub fn check_thread_for_errors(&mut self) -> Result<()> {
        self.client.check_thread_for_errors()
    }

    /// Whether the handshake is still in progress.
    ///
    /// Callers spin on this waiting for the world to arrive, so it goes false when the
    /// connection fails too - otherwise there is nothing left to wait for and the wait never
    /// ends. The failure itself is reported by the next `update`.
    #[must_use]
    pub fn is_welcoming(&self) -> bool {
        self.client.is_handshaking()
    }

    pub fn start_receiving(&self) {
        self.client.resume_receiving();
    }

    /// Disconnects and joins the networking thread. Safe to call at any point, including
    /// while the welcome is still going - somebody closing the window while a world loads.
    pub fn stop(&mut self) -> Result<()> {
        self.client.stop()
    }
}
