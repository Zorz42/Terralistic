use crate::libraries::events;
use crate::libraries::events::EventManager;
use crate::shared::enet_global::ENET_GLOBAL;
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use crate::shared::players::NamePacket;
use anyhow::{anyhow, bail, Result};
use enet::{Address, BandwidthLimit, ChannelLimit, Event, Host, PacketMode};

/**
This event is called, when the client has received a welcome packet.
 */
pub struct WelcomePacketEvent {
    pub packet: Packet,
}

/**
This handles all the networking for the client.
client connects to a server and sends and receives packets.
 */
pub struct ClientNetworking {
    server_port: u16,
    server_address: String,
    net_client: Option<Host<()>>,
}

impl ClientNetworking {
    pub const fn new(server_port: u16, server_address: String) -> Self {
        Self {
            server_port,
            server_address,
            net_client: None,
        }
    }

    pub fn init(&mut self, events: &mut EventManager, name: &str) -> Result<()> {
        // connect to the server
        self.net_client = Some(ENET_GLOBAL.create_host::<()>(
            None,
            10,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )?);

        self.net_client
            .as_mut()
            .ok_or_else(|| anyhow!("client enet not constructed yet"))?
            .connect(
                &Address::new(self.server_address.parse()?, self.server_port),
                10,
                0,
            )?;

        loop {
            if let Some(event) = self
                .net_client
                .as_mut()
                .ok_or_else(|| anyhow!("client enet not constructed yet"))?
                .service(0)?
            {
                match event {
                    Event::Connect(ref _peer) => {
                        break;
                    }
                    Event::Disconnect { .. } => {
                        bail!("disconnected from server");
                    }
                    Event::Receive { .. } => {
                        bail!("unexpected receive");
                    }
                };
            }
        }

        self.send_packet(&Packet::new(NamePacket {
            name: name.to_owned(),
        })?)?;

        'welcome_loop: loop {
            if let Some(event) = self
                .net_client
                .as_mut()
                .ok_or_else(|| anyhow!("client enet not constructed yet"))?
                .service(0)?
            {
                match event {
                    Event::Connect(_) => {
                        bail!("unexpected connect");
                    }
                    Event::Disconnect { .. } => {
                        bail!("disconnected from server");
                    }
                    Event::Receive { ref packet, .. } => {
                        let packet = bincode::deserialize::<Packet>(packet.data())?;

                        if packet.try_deserialize::<WelcomeCompletePacket>().is_some() {
                            break 'welcome_loop;
                        }

                        // send welcome packet event
                        events.push_event(events::Event::new(WelcomePacketEvent { packet }));
                    }
                };
            }
        }

        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager) -> Result<()> {
        self.net_client
            .as_mut()
            .ok_or_else(|| anyhow!("client enet not constructed yet"))?
            .flush();

        while let Some(event) = self
            .net_client
            .as_mut()
            .ok_or_else(|| anyhow!("client enet not constructed yet"))?
            .service(0)?
        {
            match event {
                Event::Connect(_) => {
                    bail!("connected to server at incorrect time");
                }
                Event::Disconnect { .. } => {
                    bail!("disconnected from server");
                }
                Event::Receive { ref packet, .. } => {
                    let packet = bincode::deserialize::<Packet>(packet.data())?;

                    // send welcome packet event
                    events.push_event(events::Event::new(packet));
                }
            };
        }
        Ok(())
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        let packet_data = bincode::serialize(packet)?;
        let mut server = self
            .net_client
            .as_mut()
            .ok_or_else(|| anyhow!("client enet not constructed yet"))?
            .peers()
            .next()
            .ok_or_else(|| anyhow!("no server peer"))?;
        server.send_packet(
            enet::Packet::new(&packet_data, PacketMode::ReliableSequenced)?,
            0,
        )?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        // disconnect the socket
        for ref mut server in self
            .net_client
            .as_mut()
            .ok_or_else(|| anyhow!("client enet not constructed yet"))?
            .peers()
        {
            server.disconnect(0);
        }
        Ok(())
    }
}
