use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
extern crate alloc;
use alloc::sync::Arc;
use std::thread::JoinHandle;

use anyhow::{anyhow, bail, Result};
use message_io::network::{Endpoint, NetEvent, SendStatus, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler};

use crate::libraries::events;
use crate::libraries::events::EventManager;
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
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
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
        let (mut handler, listener) = node::split();

        let server_addr = format!("{server_address}:{server_port}");
        let (server_endpoint, _) = handler
            .network()
            .connect(Transport::FramedTcp, server_addr)
            .unwrap();

        Self::send_packet_internal(
            &mut handler,
            &Packet::new(NamePacket {
                name: player_name.to_owned(),
            })?,
            server_endpoint,
        )?;

        handler.signals().send(());

        listener.for_each(move |event| {
            if is_welcoming.load(Ordering::Relaxed) {
                // welcoming loop
                if let NodeEvent::Network(event) = event {
                    match event {
                        NetEvent::Accepted(..)
                        | NetEvent::Connected(..)
                        | NetEvent::Disconnected(..) => {}
                        NetEvent::Message(_peer, packet) => {
                            let packet = bincode::deserialize::<Packet>(packet).unwrap();

                            if packet.try_deserialize::<WelcomeCompletePacket>().is_some() {
                                is_welcoming.store(false, Ordering::Relaxed);
                                while !should_start_receiving.load(Ordering::Relaxed) {
                                    // wait 1 ms
                                    std::thread::sleep(core::time::Duration::from_millis(1));
                                }
                            }

                            // send welcome packet event
                            event_sender
                                .send(events::Event::new(WelcomePacketEvent { packet }))
                                .ok();
                        }
                    }
                };
            } else {
                // normal loop
                match event {
                    NodeEvent::Network(event) => match event {
                        NetEvent::Connected(..)
                        | NetEvent::Accepted(..)
                        | NetEvent::Disconnected(..) => {}
                        NetEvent::Message(_peer, packet) => {
                            let packet = bincode::deserialize::<Packet>(packet).unwrap();

                            event_sender.send(events::Event::new(packet)).ok();
                        }
                    },
                    NodeEvent::Signal(_) => {
                        if !is_running.load(Ordering::Relaxed) {
                            handler.stop();
                        }

                        while let Ok(packet) = packet_receiver.try_recv() {
                            Self::send_packet_internal(&mut handler, &packet, server_endpoint)
                                .unwrap();
                        }

                        handler
                            .signals()
                            .send_with_timer((), core::time::Duration::from_millis(1));
                    }
                };
            }
        });

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

    fn send_packet_internal(
        net_client: &mut NodeHandler<()>,
        packet: &Packet,
        endpoint: Endpoint,
    ) -> Result<()> {
        let packet_data = bincode::serialize(packet)?;

        loop {
            let status = net_client.network().send(endpoint, &packet_data);
            match status {
                SendStatus::Sent => break,
                SendStatus::MaxPacketSizeExceeded => {
                    bail!("Max packet size exceeded");
                }
                SendStatus::ResourceNotFound => {
                    bail!("Resource not found");
                }
                SendStatus::ResourceNotAvailable => {
                    std::thread::sleep(core::time::Duration::from_millis(1));
                    // just try again
                }
            };
        }
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
