use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use message_io::network::{NetEvent, SendStatus, Transport};
use message_io::node::{self, NodeEvent, NodeHandler};

use crate::libraries::net::{BindAddress, Connection, LogLevel, Logger, Packet};
use crate::libraries::serialization;

/// Something the transport noticed, in the order it happened.
pub enum ServerEvent {
    /// A peer completed a TCP connection. Nothing has been said by either side yet.
    Connected(Connection),
    Disconnected(Connection),
    Received {
        conn: Connection,
        packet: Packet,
    },
}

/// What the networking thread is asked to do between polls.
enum Command {
    Send { data: Vec<u8>, conn: Connection },
    Disconnect(Connection),
}

/// Accepts connections and carries `Packet`s to and from them.
///
/// The socket lives on a thread of its own and talks to the owner through a channel pair, so
/// the owner's loop only ever touches the channels. `poll` is where everything surfaces:
/// the events that arrived, and any error the thread died of.
///
/// # Not in scope
///
/// Who is allowed to connect, what has to be said first, and what counts as a peer being
/// ready. This layer will hand over every packet from every peer that completed a TCP
/// connection, including one it is about to refuse - deciding that is the owner's, and
/// `disconnect` is how it acts on the decision.
///
/// `send` takes the connections to send to rather than having a notion of "everyone",
/// because which peers count as everyone is exactly that same policy.
pub struct PacketServer {
    port: u16,
    bind_address: BindAddress,
    log: Logger,
    event_receiver: Option<Receiver<ServerEvent>>,
    command_sender: Option<Sender<Command>>,
    is_running: Arc<AtomicBool>,
    is_listening: Arc<AtomicBool>,
    net_loop_thread: Option<std::thread::JoinHandle<Result<()>>>,
}

impl PacketServer {
    #[must_use]
    pub fn new(port: u16, bind_address: BindAddress, log: Logger) -> Self {
        Self {
            port,
            bind_address,
            log,
            event_receiver: None,
            command_sender: None,
            is_running: Arc::new(AtomicBool::new(true)),
            is_listening: Arc::new(AtomicBool::new(false)),
            net_loop_thread: None,
        }
    }

    /// The address this server binds, for whoever wants to say so.
    #[must_use]
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.bind_address.as_ip(), self.port)
    }

    #[must_use]
    pub const fn bind_address(&self) -> BindAddress {
        self.bind_address
    }

    /// Starts the networking thread.
    ///
    /// **The bind happens on that thread, so its failure arrives late.** This returns before
    /// the port has even been attempted, and a port that is already taken surfaces through a
    /// later `poll`, when the finished thread is joined. A server whose port is taken would
    /// otherwise look like one that started and then accepted nobody forever, so the error
    /// names the address it could not bind rather than being a bare `AddrInUse`.
    pub fn listen(&mut self) {
        let (event_sender, event_receiver) = mpsc::channel();
        let (command_sender, command_receiver) = mpsc::channel();
        self.event_receiver = Some(event_receiver);
        self.command_sender = Some(command_sender);

        let is_running = self.is_running.clone();
        let is_listening = self.is_listening.clone();
        let log = self.log.clone();
        let listen_addr = self.listen_address();

        self.net_loop_thread = Some(
            // this panics with normal thread creation anyway
            #[allow(clippy::unwrap_used)]
            std::thread::Builder::new()
                .name("Packet server".to_owned())
                .spawn(move || Self::net_receive_loop(&event_sender, &command_receiver, &is_running, &is_listening, &log, &listen_addr))
                .unwrap(),
        );
    }

    /// True once the networking thread has actually bound the port and is accepting.
    ///
    /// `listen` only spawns that thread, so there is a window where the server exists and
    /// nothing is listening yet. Anyone who needs to know the difference has to be told by
    /// the thread that binds - probing the port from outside means *binding* it, which races
    /// the bind being waited for and can lose it the port entirely.
    #[must_use]
    pub fn is_listening(&self) -> bool {
        self.is_listening.load(Ordering::Relaxed)
    }

    fn net_receive_loop(
        event_sender: &Sender<ServerEvent>,
        command_receiver: &Receiver<Command>,
        is_running: &Arc<AtomicBool>,
        is_listening: &Arc<AtomicBool>,
        log: &Logger,
        listen_addr: &str,
    ) -> Result<()> {
        let (handler, listener) = node::split::<()>();

        // the error is worth naming: a port already in use is the common way this fails, and
        // it otherwise surfaces only as a server that silently never accepts anyone
        handler
            .network()
            .listen(Transport::FramedTcp, listen_addr)
            .map_err(|e| anyhow!("could not listen on {listen_addr}: {e}"))?;
        is_listening.store(true, Ordering::Relaxed);
        log(LogLevel::Info, &format!("listening on {listen_addr}"));

        handler.signals().send(());

        listener.for_each(|event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(..) => {}
                NetEvent::Accepted(peer, _) => {
                    log(LogLevel::Info, &format!("[{peer}] connected"));
                    if let Err(e) = event_sender.send(ServerEvent::Connected(Connection::new(peer))) {
                        log(LogLevel::Error, &format!("Failed to report a connection: {e}"));
                    }
                }
                NetEvent::Disconnected(peer) => {
                    log(LogLevel::Info, &format!("[{peer}] disconnected"));
                    if let Err(e) = event_sender.send(ServerEvent::Disconnected(Connection::new(peer))) {
                        log(LogLevel::Error, &format!("Failed to report a disconnection: {e}"));
                    }
                }
                NetEvent::Message(peer, data) => {
                    // a malformed frame from one peer must not take down networking for
                    // everyone, so drop the packet and keep serving the other connections
                    let packet: Packet = match serialization::deserialize(data) {
                        Ok(packet) => packet,
                        Err(e) => {
                            log(LogLevel::Warning, &format!("[{peer}] sent a packet that could not be deserialized, ignoring it: {e}"));
                            return;
                        }
                    };

                    if let Err(e) = event_sender.send(ServerEvent::Received { conn: Connection::new(peer), packet }) {
                        log(LogLevel::Error, &format!("Failed to report a packet: {e}"));
                    }
                }
            },
            NodeEvent::Signal(()) => {
                if !is_running.load(Ordering::Relaxed) {
                    handler.stop();
                }

                while let Ok(command) = command_receiver.try_recv() {
                    match command {
                        Command::Send { data, conn } => {
                            // sending routinely fails when the peer has gone away between
                            // queueing and sending, which is normal and must not kill the
                            // networking thread
                            if let Err(e) = Self::send_now(&handler, &data, &conn) {
                                log(LogLevel::Warning, &format!("Failed to send a packet to [{conn}]: {e}"));
                            }
                        }
                        Command::Disconnect(conn) => {
                            handler.network().remove(conn.endpoint.resource_id());
                        }
                    }
                }

                handler.signals().send_with_timer((), std::time::Duration::from_millis(1));
            }
        });

        Ok(())
    }

    fn send_now(handler: &NodeHandler<()>, data: &[u8], conn: &Connection) -> Result<()> {
        loop {
            match handler.network().send(conn.endpoint, data) {
                SendStatus::Sent => break,
                SendStatus::MaxPacketSizeExceeded => bail!("Max packet size exceeded"),
                SendStatus::ResourceNotFound => bail!("Resource not found"),
                SendStatus::ResourceNotAvailable => {
                    // wait a bit and try again
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }

        Ok(())
    }

    /// Everything that arrived since the last call.
    ///
    /// Also where the networking thread's own failure is reported: it is joined once it has
    /// finished, and whatever it returned is returned from here.
    pub fn poll(&mut self) -> Result<Vec<ServerEvent>> {
        let mut events = Vec::new();
        if let Some(event_receiver) = &self.event_receiver {
            while let Ok(event) = event_receiver.try_recv() {
                events.push(event);
            }
        }

        if self.net_loop_thread.as_ref().is_some_and(std::thread::JoinHandle::is_finished) {
            if let Some(thread_handle) = self.net_loop_thread.take() {
                match thread_handle.join() {
                    Ok(result) => result?,
                    Err(_e) => bail!("Failed to join net loop thread"),
                }
            }
        }

        Ok(events)
    }

    /// Queues `packet` for each of `connections`. Serialized once, however many there are.
    pub fn send(&mut self, packet: &Packet, connections: &[Connection]) -> Result<()> {
        if connections.is_empty() {
            return Ok(());
        }

        let data = serialization::serialize(packet)?;
        let sender = self.command_sender.as_mut().ok_or_else(|| anyhow!("the server is not listening yet"))?;

        for conn in connections {
            sender.send(Command::Send {
                data: data.clone(),
                conn: conn.clone(),
            })?;
        }
        Ok(())
    }

    /// Drops a peer. The owner's way of refusing a connection it has decided against.
    pub fn disconnect(&mut self, conn: &Connection) -> Result<()> {
        self.command_sender
            .as_mut()
            .ok_or_else(|| anyhow!("the server is not listening yet"))?
            .send(Command::Disconnect(conn.clone()))?;
        Ok(())
    }

    /// Stops the networking thread and waits for it to end.
    pub fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(thread_handle) = self.net_loop_thread.take() {
            if thread_handle.join().is_err() {
                bail!("Failed to join net loop thread");
            }
        }
        Ok(())
    }
}
