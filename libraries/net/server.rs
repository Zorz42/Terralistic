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
/// The socket lives on its own thread and talks to the owner through a channel pair, so the owner's
/// loop only touches the channels; `poll` surfaces both the events and any error the thread died
/// of.
///
/// **Not in scope**: who may connect and what has to be said first. This hands over every
/// packet from every peer that completed a TCP connection, including one it is about to
/// refuse, and `disconnect` is how the owner acts. `send` takes the connections to send to
/// rather than knowing about "everyone", which is the same policy.
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

    /// Starts the networking thread. **The bind happens there, so its failure arrives late**:
    /// this returns before the port is attempted, and a taken one surfaces through a later
    /// `poll` that joins the finished thread. Since the alternative reads as a server that
    /// started and accepts nobody, the error names the address rather than being `AddrInUse`.
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

    /// True once the thread has bound the port. `listen` only spawns it, so there is a window
    /// where the server exists and nothing listens - and this is the only way to tell, since
    /// probing the port means *binding* it, which races the bind being waited for.
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

        // Named, because a port in use is the common failure and otherwise surfaces only as a
        // server that silently never accepts anyone.
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
                    // One peer's malformed frame must not take networking down for everyone.
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
                            // A peer that went away between queueing and sending is normal,
                            // and must not kill the networking thread.
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
                SendStatus::ResourceNotAvailable => std::thread::sleep(std::time::Duration::from_millis(1)),
            }
        }

        Ok(())
    }

    /// Everything that arrived since the last call, and where the networking thread's own
    /// failure surfaces: a finished thread is joined and whatever it returned comes back here.
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
