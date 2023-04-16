use core::hash::{Hash, Hasher};
use core::sync::atomic::AtomicBool;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
extern crate alloc;
use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::libraries::events::{Event, EventManager};
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use crate::shared::players::NamePacket;
use anyhow::{anyhow, bail, Result};
use message_io::network::{Endpoint, NetEvent, SendStatus, Transport};

use crate::server::server_ui::{ConsoleMessageType, UiMessageType};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler};

//TODO: add more errors to this file

/// This struct holds the address of a connection.
#[derive(Clone, Eq)]
pub struct Connection {
    pub(super) address: Endpoint,
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.addr().hash(state);
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.address.addr() == other.address.addr()
    }
}

pub enum SendTarget {
    All,
    Connection(Connection),
    AllExcept(Connection),
}

/// This handles all the networking for the server.
/// server listens for connections and sends and receives packets
/// for each client.
pub struct ServerNetworking {
    server_port: u16,
    connections: Vec<Connection>,
    connection_names: HashMap<Connection, String>,
    event_receiver: Option<Receiver<Event>>,
    packet_sender: Option<Sender<(Vec<u8>, Connection)>>,
    is_running: Arc<AtomicBool>,
    net_loop_thread: Option<std::thread::JoinHandle<Result<()>>>,
}

impl ServerNetworking {
    pub fn new(server_port: u16) -> Self {
        Self {
            server_port,
            connections: Vec::new(),
            connection_names: HashMap::new(),
            event_receiver: None,
            packet_sender: None,
            is_running: Arc::new(AtomicBool::new(true)),
            net_loop_thread: None,
        }
    }

    pub fn get_connection_name(&self, conn: &Connection) -> String {
        let unknown = "Unknown".to_owned();
        self.connection_names.get(conn).unwrap_or(&unknown).clone()
    }

    pub fn init(&mut self) {
        // start listening for connections
        let (event_sender, event_receiver) = mpsc::channel();
        let (packet_sender, packet_receiver) = mpsc::channel();
        self.event_receiver = Some(event_receiver);
        self.packet_sender = Some(packet_sender);

        let is_running = self.is_running.clone();
        let server_port = self.server_port;

        self.net_loop_thread = Some(std::thread::spawn(move || {
            Self::net_receive_loop(&event_sender, &packet_receiver, &is_running, server_port)
        }));
    }

    #[allow(clippy::unwrap_used)]
    fn net_receive_loop(
        event_sender: &Sender<Event>,
        packet_receiver: &Receiver<(Vec<u8>, Connection)>,
        is_running: &Arc<AtomicBool>,
        server_port: u16,
    ) -> Result<()> {
        let (mut handler, listener) = node::split::<()>();

        let listen_addr = format!("127.0.0.1:{server_port}");
        handler
            .network()
            .listen(Transport::FramedTcp, listen_addr)?;

        handler.signals().send(());

        listener.for_each(|event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(..) => {}
                NetEvent::Accepted(peer, _) => {
                    println!("[{peer}] connected");
                    send_console_message(
                        event_sender,
                        ConsoleMessageType::Info(format!("[INFO] [{peer}] connected")),
                    );
                }
                NetEvent::Disconnected(peer) => {
                    println!("[{peer}] disconnected");
                    send_console_message(
                        event_sender,
                        ConsoleMessageType::Info(format!("[INFO] [{peer}] disconnected")),
                    );
                    match event_sender.send(Event::new(DisconnectEvent {
                        conn: Connection { address: peer },
                    })) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Failed to send disconnect event: {e}");
                        }
                    }
                }
                NetEvent::Message(peer, packet) => {
                    let packet: Packet = bincode::deserialize(packet).unwrap();
                    if let Some(packet) = packet.try_deserialize::<NamePacket>() {
                        println!("[{:?}] joined", packet.name);
                        send_console_message(
                            event_sender,
                            ConsoleMessageType::Info(format!("[{:?}] joined", packet.name)),
                        );
                        match event_sender.send(Event::new(NewConnectionEvent {
                            conn: Connection { address: peer },
                            name: packet.name,
                        })) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to send new connection event: {e}");
                                send_console_message(
                                    event_sender,
                                    ConsoleMessageType::Error(format!(
                                        "[ERROR] Failed to send new connection event: {e}"
                                    )),
                                );
                            }
                        }
                    } else {
                        match event_sender.send(Event::new(PacketFromClientEvent {
                            packet,
                            conn: Connection { address: peer },
                        })) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to send packet from client event: {e}");
                                send_console_message(
                                    event_sender,
                                    ConsoleMessageType::Error(format!(
                                        "[ERROR] Failed to send packet from client event: {e}"
                                    )),
                                );
                            }
                        }
                    }
                }
            },
            NodeEvent::Signal(_) => {
                if !is_running.load(Ordering::Relaxed) {
                    handler.stop();
                }

                while let Ok((packet_data, conn)) = packet_receiver.try_recv() {
                    Self::send_packet_internal(&mut handler, &packet_data, &conn).unwrap();
                }

                handler
                    .signals()
                    .send_with_timer((), core::time::Duration::from_millis(1));
            }
        });

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, events: &mut EventManager) -> Result<()> {
        // handle new connection event
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            self.connection_names
                .insert(event.conn.clone(), event.name.clone());

            self.send_packet(
                &Packet::new(WelcomeCompletePacket {})?,
                SendTarget::Connection(event.conn.clone()),
            )?;
            self.connections.push(event.conn.clone());
            events.push_event(Event::new(NewConnectionWelcomedEvent {
                conn: event.conn.clone(),
            }));
        }

        if let Some(event) = event.downcast::<DisconnectEvent>() {
            self.connections.retain(|x| x != &event.conn);
        }

        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager) -> Result<()> {
        if let Some(event_receiver) = &self.event_receiver {
            while let Ok(event) = event_receiver.try_recv() {
                events.push_event(event);
            }
        }

        let mut net_loop_finished = false;
        if let Some(thread_handle) = &self.net_loop_thread {
            if thread_handle.is_finished() {
                net_loop_finished = true;
            }
        }

        if net_loop_finished {
            if let Some(thread_handle) = self.net_loop_thread.take() {
                let result = match thread_handle.join() {
                    Ok(result) => result,
                    Err(_e) => {
                        bail!("Failed to join net loop thread");
                    }
                };
                return result;
            }
        }

        Ok(())
    }

    fn send_packet_internal(
        net_server: &mut NodeHandler<()>,
        packet_data: &[u8],
        conn: &Connection,
    ) -> Result<()> {
        loop {
            let status = net_server.network().send(conn.address, packet_data);
            match status {
                SendStatus::Sent => break,
                SendStatus::MaxPacketSizeExceeded => bail!("Max packet size exceeded"),
                SendStatus::ResourceNotFound => bail!("Resource not found"),
                SendStatus::ResourceNotAvailable => {
                    // wait a bit and try again
                    std::thread::sleep(core::time::Duration::from_millis(1));
                }
            };
        }

        Ok(())
    }

    pub fn send_packet(&mut self, packet: &Packet, target: SendTarget) -> Result<()> {
        let packet_data = bincode::serialize(&packet)?;

        match target {
            SendTarget::All => {
                for conn in &self.connections {
                    self.packet_sender
                        .as_mut()
                        .ok_or_else(|| anyhow!("packet sender not constructed yet"))?
                        .send((packet_data.clone(), conn.clone()))?;
                }
            }
            SendTarget::Connection(conn) => {
                self.packet_sender
                    .as_mut()
                    .ok_or_else(|| anyhow!("packet sender not constructed yet"))?
                    .send((packet_data, conn))?;
            }
            SendTarget::AllExcept(conn) => {
                for c in &self.connections {
                    if c != &conn {
                        self.packet_sender
                            .as_mut()
                            .ok_or_else(|| anyhow!("packet sender not constructed yet"))?
                            .send((packet_data.clone(), c.clone()))?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn stop(&mut self, events: &mut EventManager) -> Result<()> {
        // close all connections

        self.is_running.store(false, Ordering::Relaxed);
        if let Some(thread_handle) = self.net_loop_thread.take() {
            match thread_handle.join() {
                Ok(_) => {}
                Err(_) => {
                    bail!("Failed to join net loop thread");
                }
            }
        }

        for conn in &self.connections {
            events.push_event(Event::new(DisconnectEvent { conn: conn.clone() }));
        }

        Ok(())
    }
}

//sends a console message to UI
fn send_console_message(event_sender: &Sender<Event>, message: ConsoleMessageType) {
    if let Err(e) = event_sender.send(Event::new(UiMessageType::SrvToUiConsoleMessage(message))) {
        println!("Failed to send console message: {e}");
    }
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
