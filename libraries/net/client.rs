use std::net::ToSocketAddrs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;

use anyhow::{anyhow, bail, Result};
use message_io::network::{Endpoint, NetEvent, SendStatus, ToRemoteAddr, Transport};
use message_io::node::{self, NodeEvent, NodeHandler};

use crate::libraries::net::{LogLevel, Logger, Packet};
use crate::libraries::serialization;

/// What went wrong, in terms the transport can actually tell apart.
///
/// A kind rather than a string, because the two interesting cases mean something specific to
/// whoever owns the connection and only they can word it: a caller waiting to be let into
/// something knows that a close mid-handshake means it was turned away, and this layer only
/// knows that the socket shut.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    /// The connection was never established. `connect` does not block, so a port with
    /// nothing behind it and a peer that refuses both surface here rather than as an error
    /// from `connect` itself.
    NotAccepted,
    /// The peer closed the connection while the handshake was still going.
    ClosedDuringHandshake,
    /// Anything else, already described.
    Failed(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAccepted => write!(formatter, "could not connect: the connection was not accepted"),
            Self::ClosedDuringHandshake => write!(formatter, "the connection was closed during the handshake"),
            Self::Failed(message) => write!(formatter, "{message}"),
        }
    }
}

/// A packet that arrived, and whether it did so before the handshake finished.
///
/// The two are different kinds of traffic to most callers - what arrives during a handshake
/// is usually setup that has to be applied before anything else is looked at - so the phase
/// is recorded when the packet arrives rather than guessed at afterwards.
pub struct Received {
    pub packet: Packet,
    pub during_handshake: bool,
}

/// Connects to a `PacketServer` and carries `Packet`s to and from it.
///
/// Like the server, the socket lives on its own thread and speaks to the owner through
/// channels.
///
/// # The handshake phase
///
/// A connection usually starts with an exchange that has to finish before ordinary traffic
/// means anything. This layer knows only three things about it: what to send on connecting
/// (`greeting`), which received packet ends it (`handshake_done`), and that the owner may
/// need a moment afterwards. When the ending packet arrives the thread stops delivering
/// until `resume_receiving` is called, so the owner can drain everything the handshake
/// produced before the first ordinary packet lands behind it.
///
/// What any of those packets *mean* is not in scope.
///
/// # Not in scope
///
/// Reconnecting, and deciding when to give up. `is_handshaking` goes false on failure so a
/// caller spinning on it is not left with nothing to wait for, and the failure itself comes
/// out of the next `poll`.
pub struct PacketClient {
    address: String,
    port: u16,
    log: Logger,
    is_running: Arc<AtomicBool>,
    is_handshaking: Arc<AtomicBool>,
    may_resume: Arc<AtomicBool>,
    net_loop_thread: Option<JoinHandle<Result<()>>>,
    packet_receiver: Option<Receiver<Received>>,
    packet_sender: Option<Sender<Packet>>,
    error: Arc<Mutex<Option<ClientError>>>,
}

impl PacketClient {
    #[must_use]
    pub fn new(address: String, port: u16, log: Logger) -> Self {
        Self {
            address,
            port,
            log,
            is_running: Arc::new(AtomicBool::new(true)),
            is_handshaking: Arc::new(AtomicBool::new(true)),
            may_resume: Arc::new(AtomicBool::new(false)),
            net_loop_thread: None,
            packet_receiver: None,
            packet_sender: None,
            error: Arc::new(Mutex::new(None)),
        }
    }

    /// Connects, sends `greeting` in order, and runs until stopped.
    ///
    /// `handshake_done` is asked about each received packet, on the networking thread, until
    /// it answers true once.
    pub fn connect<HandshakeDone: Fn(&Packet) -> bool + Send + 'static>(&mut self, greeting: Vec<Packet>, handshake_done: HandshakeDone) -> Result<()> {
        let (packet_sender_in, packet_receiver_in) = mpsc::channel();
        let (packet_sender_out, packet_receiver_out) = mpsc::channel();
        self.packet_receiver = Some(packet_receiver_in);
        self.packet_sender = Some(packet_sender_out);

        let is_running = self.is_running.clone();
        let is_handshaking = self.is_handshaking.clone();
        let may_resume = self.may_resume.clone();
        let error = self.error.clone();
        let log = self.log.clone();
        let address = format!("{}:{}", self.address, self.port);

        self.net_loop_thread = Some(std::thread::Builder::new().name("Packet client".to_owned()).spawn(move || {
            Self::net_receive_loop(
                &packet_sender_in,
                &packet_receiver_out,
                &is_running,
                &is_handshaking,
                &may_resume,
                &error,
                &log,
                &address,
                greeting,
                handshake_done,
            )
        })?);

        Ok(())
    }

    #[allow(clippy::too_many_arguments, reason = "everything the loop needs has to cross the thread boundary by value")]
    fn net_receive_loop(
        packet_sender: &Sender<Received>,
        packet_receiver: &Receiver<Packet>,
        is_running: &Arc<AtomicBool>,
        is_handshaking: &Arc<AtomicBool>,
        may_resume: &Arc<AtomicBool>,
        error: &Arc<Mutex<Option<ClientError>>>,
        log: &Logger,
        address: &str,
        greeting: Vec<Packet>,
        handshake_done: impl Fn(&Packet) -> bool,
    ) -> Result<()> {
        let (handler, listener) = node::split();

        let server_addr = address.to_remote_addr()?.to_socket_addrs()?.next().ok_or_else(|| anyhow!("address {address} not found"))?;
        let (server_endpoint, _) = handler.network().connect(Transport::FramedTcp, server_addr)?;

        for packet in greeting {
            Self::send_now(&handler, &packet, server_endpoint)?;
        }

        // The signal chain starts here rather than when the handshake finishes, because the
        // handshake is otherwise driven entirely by what the peer sends: with nothing
        // ticking, a caller that has given up - somebody closing the window while a world was
        // still on its way - could not be noticed until the peer said something.
        handler.signals().send_with_timer((), std::time::Duration::from_millis(1));

        let record_error = |kind: ClientError| {
            let mut slot = error.lock().unwrap_or_else(PoisonError::into_inner);
            if slot.is_none() {
                log(LogLevel::Error, &kind.to_string());
                *slot = Some(kind);
            }
        };

        listener.for_each(move |event| {
            if error.lock().unwrap_or_else(PoisonError::into_inner).is_some() {
                // once something has gone wrong there is nothing useful left to do with
                // anything that arrives, and reporting it again only buries the first cause
                return;
            }

            match event {
                NodeEvent::Signal(()) => {
                    if !is_running.load(Ordering::Relaxed) {
                        handler.stop();
                        return;
                    }

                    // nothing is sent during the handshake: the greeting went out on
                    // connecting, and anything the owner queues before it is welcomed would
                    // arrive out of order behind the exchange still in flight
                    if !is_handshaking.load(Ordering::Relaxed) {
                        while let Ok(packet) = packet_receiver.try_recv() {
                            if let Err(err) = Self::send_now(&handler, &packet, server_endpoint) {
                                record_error(ClientError::Failed(err.to_string()));
                                return;
                            }
                        }
                    }

                    handler.signals().send_with_timer((), std::time::Duration::from_millis(1));
                }
                NodeEvent::Network(event) => match event {
                    NetEvent::Accepted(..) => {}
                    // `connect` is not blocking, so a peer that is down or refusing is
                    // reported here rather than by an error from `connect` itself. Ignoring
                    // it left the caller spinning on `is_handshaking` for as long as it was
                    // willing to wait.
                    NetEvent::Connected(_peer, established) => {
                        if !established {
                            record_error(ClientError::NotAccepted);
                            handler.stop();
                        }
                    }
                    NetEvent::Disconnected(..) => {
                        if is_handshaking.load(Ordering::Relaxed) {
                            record_error(ClientError::ClosedDuringHandshake);
                            handler.stop();
                        }
                    }
                    NetEvent::Message(_peer, data) => {
                        let packet: Packet = match serialization::deserialize(data) {
                            Ok(packet) => packet,
                            Err(err) => {
                                record_error(ClientError::Failed(err.to_string()));
                                return;
                            }
                        };

                        let during_handshake = is_handshaking.load(Ordering::Relaxed);
                        if during_handshake && handshake_done(&packet) {
                            is_handshaking.store(false, Ordering::Relaxed);
                            // The owner drains what the handshake produced and then says it
                            // is ready, so nothing that arrives next is read before it has.
                            // `is_running` is watched as well because a caller can also give
                            // up here, and then nothing would ever say it was ready.
                            while !may_resume.load(Ordering::Relaxed) && is_running.load(Ordering::Relaxed) {
                                std::thread::sleep(std::time::Duration::from_millis(1));
                            }
                            // no signal is sent here: the timer armed before this loop is
                            // still in flight, and a second one would mean two chains ticking
                            // against each other forever
                        }

                        if let Err(err) = packet_sender.send(Received { packet, during_handshake }) {
                            record_error(ClientError::Failed(err.to_string()));
                        }
                    }
                },
            }
        });

        Ok(())
    }

    fn send_now(handler: &NodeHandler<()>, packet: &Packet, endpoint: Endpoint) -> Result<()> {
        let data = serialization::serialize(packet)?;

        loop {
            match handler.network().send(endpoint, &data) {
                SendStatus::Sent => break,
                SendStatus::MaxPacketSizeExceeded => bail!("Max packet size exceeded"),
                // `message_io` drops the connection's resource when a pending connect is
                // refused, so this is also what a port with nothing behind it looks like from
                // here - and it is the first thing a handshake finds out, since the greeting
                // goes before anything else.
                SendStatus::ResourceNotFound => {
                    bail!("no connection to {endpoint}: nothing is listening there, or it closed the connection");
                }
                SendStatus::ResourceNotAvailable => {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    // just try again
                }
            }
        }
        Ok(())
    }

    /// Everything that arrived since the last call, plus whatever the networking thread
    /// died of if it has ended.
    ///
    /// What the *receive loop* noticed is deliberately not reported here: it comes out of
    /// `take_error` as a kind, so the owner can say what it means. Drain the packets first
    /// and ask afterwards - a handshake that failed usually delivered something before it
    /// did, and that is worth having.
    pub fn poll(&mut self) -> Result<Vec<Received>> {
        let mut received = Vec::new();
        if let Some(receiver) = &self.packet_receiver {
            while let Ok(packet) = receiver.try_recv() {
                received.push(packet);
            }
        }

        self.check_thread_for_errors()?;

        Ok(received)
    }

    /// The failure kind, if there is one, clearing it so it is only reported once.
    #[must_use]
    pub fn take_error(&self) -> Option<ClientError> {
        self.error.lock().unwrap_or_else(PoisonError::into_inner).take()
    }

    pub fn send(&mut self, packet: Packet) -> Result<()> {
        self.packet_sender.as_mut().ok_or_else(|| anyhow!("not connected yet"))?.send(packet)?;
        Ok(())
    }

    /// Surfaces the networking thread's own error, if it has ended.
    pub fn check_thread_for_errors(&mut self) -> Result<()> {
        if self.net_loop_thread.as_ref().is_some_and(JoinHandle::is_finished) {
            let thread = self.net_loop_thread.take().ok_or_else(|| anyhow!("thread not found"))?;
            let joined = thread.join().ok().ok_or_else(|| anyhow!("thread returned an error"))?;
            return joined;
        }

        Ok(())
    }

    /// Whether the handshake is still in progress.
    ///
    /// Callers spin on this waiting to be let in, so it has to go false when the connection
    /// fails too - otherwise there is nothing left to wait for and the wait never ends. The
    /// failure itself is reported by the next `poll`.
    #[must_use]
    pub fn is_handshaking(&self) -> bool {
        self.is_handshaking.load(Ordering::Relaxed) && self.error.lock().unwrap_or_else(PoisonError::into_inner).is_none()
    }

    /// Lets the networking thread go on delivering after the handshake.
    pub fn resume_receiving(&self) {
        self.may_resume.store(true, Ordering::Relaxed);
    }

    /// Disconnects and joins the networking thread.
    ///
    /// Safe to call at any point, including while the handshake is still going: the loop has
    /// a timer of its own that watches the running flag, and the wait for the owner to resume
    /// watches it too. Without those, a caller that gave up mid-handshake had nothing to join.
    pub fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(net_loop_thread) = self.net_loop_thread.take() {
            if net_loop_thread.join().is_err() {
                bail!("net loop thread returned an error");
            }
        }
        Ok(())
    }
}
