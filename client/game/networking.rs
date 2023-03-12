use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
extern crate alloc;
use alloc::sync::Arc;
use std::thread::JoinHandle;

use anyhow::{anyhow, bail, Result};
use enet::{Address, BandwidthLimit, ChannelLimit, Event, Host, PacketMode};

use crate::libraries::events;
use crate::libraries::events::EventManager;
use crate::shared::enet_global::ENET_GLOBAL;
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use crate::shared::players::NamePacket;

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
    is_running: Arc<AtomicBool>,
    is_welcoming: Arc<AtomicBool>,
    should_start_receiving: Arc<AtomicBool>,
    net_loop_thread: Option<JoinHandle<Result<()>>>,
    event_receiver: Option<Receiver<events::Event>>,
    packet_sender: Option<Sender<Packet>>,
}

impl ClientNetworking {
    pub fn new(server_port: u16, server_address: String) -> Self {
        Self {
            server_port,
            server_address,
            is_running: Arc::new(AtomicBool::new(true)),
            is_welcoming: Arc::new(AtomicBool::new(true)),
            should_start_receiving: Arc::new(AtomicBool::new(false)),
            net_loop_thread: None,
            event_receiver: None,
            packet_sender: None,
        }
    }

    pub fn init(&mut self, name: String) -> Result<()> {
        // connect to the server

        let (event_sender, event_receiver) = mpsc::channel();
        let (packet_sender, packet_receiver) = mpsc::channel();
        self.event_receiver = Some(event_receiver);
        self.packet_sender = Some(packet_sender);

        let is_running = self.is_running.clone();
        let is_welcoming = self.is_welcoming.clone();
        let should_start_receiving = self.should_start_receiving.clone();
        let server_address = self.server_address.clone();
        let server_port = self.server_port;

        let net_loop_thread = std::thread::spawn(move || {
            Self::net_receive_loop(
                &event_sender,
                &packet_receiver,
                &is_running,
                &is_welcoming,
                &should_start_receiving,
                &server_address,
                server_port,
                &name,
            )
        });
        if net_loop_thread.is_finished() {
            bail!("net loop thread failed");
        }

        self.net_loop_thread = Some(net_loop_thread);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn net_receive_loop(
        event_sender: &Sender<events::Event>,
        packet_receiver: &Receiver<Packet>,
        is_running: &Arc<AtomicBool>,
        is_welcoming: &Arc<AtomicBool>,
        should_start_receiving: &Arc<AtomicBool>,
        server_address: &str,
        server_port: u16,
        player_name: &str,
    ) -> Result<()> {
        let mut net_client = ENET_GLOBAL.create_host::<()>(
            None,
            10,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )?;

        net_client.connect(&Address::new(server_address.parse()?, server_port), 10, 0)?;

        loop {
            if let Some(event) = net_client.service(0)? {
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

        Self::send_packet_internal(
            &mut net_client,
            &Packet::new(NamePacket {
                name: player_name.to_owned(),
            })?,
        )?;

        'welcome_loop: loop {
            if let Some(event) = net_client.service(0)? {
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
                        match event_sender.send(events::Event::new(WelcomePacketEvent { packet })) {
                            Ok(_) => {}
                            Err(_) => {
                                bail!("event sender disconnected");
                            }
                        }
                    }
                };
            }
        }
        is_welcoming.store(false, Ordering::Relaxed);

        while !should_start_receiving.load(Ordering::Relaxed) {
            // wait 1 ms
            std::thread::sleep(core::time::Duration::from_millis(1));
        }

        while is_running.load(Ordering::Relaxed) {
            while let Ok(packet) = packet_receiver.try_recv() {
                Self::send_packet_internal(&mut net_client, &packet)?;
            }

            if let Some(event) = net_client.service(0)? {
                match event {
                    Event::Connect(_) => {
                        bail!("unexpected connect");
                    }
                    Event::Disconnect { .. } => {
                        bail!("disconnected from server");
                    }
                    Event::Receive { ref packet, .. } => {
                        let packet = bincode::deserialize::<Packet>(packet.data())?;

                        match event_sender.send(events::Event::new(packet)) {
                            Ok(_) => {}
                            Err(_) => {
                                bail!("event sender disconnected");
                            }
                        }
                    }
                };
            }
        }

        for ref mut server in net_client.peers() {
            server.disconnect(0);
        }
        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager) -> Result<()> {
        if let Some(ref receiver) = self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                events.push_event(event);
            }
        }
        if let Some(net_loop_thread) = &self.net_loop_thread {
            if net_loop_thread.is_finished() {
                bail!("net loop thread failed");
            }
        }
        Ok(())
    }

    fn send_packet_internal(net_client: &mut Host<()>, packet: &Packet) -> Result<()> {
        let packet_data = bincode::serialize(packet)?;
        let mut server = net_client
            .peers()
            .next()
            .ok_or_else(|| anyhow!("no server peer"))?;
        server.send_packet(
            enet::Packet::new(&packet_data, PacketMode::ReliableSequenced)?,
            0,
        )?;
        Ok(())
    }

    pub fn send_packet(&mut self, packet: Packet) -> Result<()> {
        self.packet_sender
            .as_mut()
            .ok_or_else(|| anyhow!("packet sender not constructed yet"))?
            .send(packet)?;
        Ok(())
    }

    pub fn is_welcoming(&self) -> bool {
        self.is_welcoming.load(Ordering::Relaxed)
    }

    pub fn start_receiving(&mut self) {
        self.should_start_receiving.store(true, Ordering::Relaxed);
    }

    pub fn stop(&mut self) -> Result<()> {
        // disconnect the socket
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(net_loop_thread) = self.net_loop_thread.take() {
            match net_loop_thread.join() {
                Ok(_) => {}
                Err(_) => {
                    bail!("net loop thread panicked");
                }
            }
        }
        Ok(())
    }
}
