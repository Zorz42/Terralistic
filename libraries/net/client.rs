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

/// What went wrong, in terms the transport can tell apart.
///
/// A kind rather than a string, because only the connection's owner can word what it means: this
/// layer knows the socket shut, and they know a close mid-handshake means they were turned away.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    /// Never established. `connect` does not block, so both a dead port and a refusing peer
    /// surface here rather than as an error from `connect`.
    NotAccepted,
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

/// A packet, and whether it arrived before the handshake finished. Handshake traffic is
/// usually setup to apply first, so the phase is recorded on arrival rather than guessed at.
pub struct Received {
    pub packet: Packet,
    pub during_handshake: bool,
}

/// Connects to a `PacketServer` and carries `Packet`s to and from it. Like the server, the
/// socket lives on its own thread and speaks to the owner through channels.
///
/// The handshake is three things the caller supplies: what to send on connecting (`greeting`),
/// which received packet ends it (`handshake_done`), and a pause afterwards - delivery stops
/// until `resume_receiving`, so the owner can drain what the handshake produced before the
/// first ordinary packet lands behind it. What those packets *mean* is not in scope, and
/// neither is reconnecting or deciding when to give up.
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

    /// Connects, sends `greeting` in order, and runs until stopped. `handshake_done` is asked
    /// about each received packet, on the networking thread, until it answers true once.
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

        // The signal chain starts here, not when the handshake ends: everything else in that
        // phase reacts to the peer, so with nothing ticking a caller that has given up -
        // closing the window mid-join - is not noticed until the peer says something.
        handler.signals().send_with_timer((), std::time::Duration::from_millis(1));

        let record_error = |kind: ClientError| {
            let mut slot = error.lock().unwrap_or_else(PoisonError::into_inner);
            if slot.is_none() {
                log(LogLevel::Error, &kind.to_string());
                *slot = Some(kind);
            }
        };

        listener.for_each(move |event| {
            // Reporting anything after the first failure only buries its cause.
            if error.lock().unwrap_or_else(PoisonError::into_inner).is_some() {
                return;
            }

            match event {
                NodeEvent::Signal(()) => {
                    if !is_running.load(Ordering::Relaxed) {
                        handler.stop();
                        return;
                    }

                    // Nothing is sent during the handshake: the greeting went out on
                    // connecting, and anything queued behind it would arrive out of order.
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
                    // `connect` does not block, so a peer that is down or refusing arrives
                    // here. Ignoring it left callers spinning on `is_handshaking`.
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
                            // Wait for the owner to drain the handshake. `is_running` too:
                            // a caller that gives up here would never say it was ready. No
                            // signal is sent - the timer armed before the loop is still in
                            // flight, and a second one is two chains ticking forever.
                            while !may_resume.load(Ordering::Relaxed) && is_running.load(Ordering::Relaxed) {
                                std::thread::sleep(std::time::Duration::from_millis(1));
                            }
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
                // `message_io` drops the resource when a pending connect is refused, so this
                // is also what a dead port looks like - and the greeting finds it first.
                SendStatus::ResourceNotFound => bail!("no connection to {endpoint}: nothing is listening there, or it closed the connection"),
                SendStatus::ResourceNotAvailable => std::thread::sleep(std::time::Duration::from_millis(1)),
            }
        }
        Ok(())
    }

    /// Everything that arrived since the last call, plus whatever the networking thread died
    /// of. What the *receive loop* noticed comes out of `take_error` instead, as a kind the
    /// owner can word. Drain first and ask after: a failed handshake usually delivered
    /// something worth having before it failed.
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

    /// Whether the handshake is still going. Callers spin on this waiting to be let in, so it
    /// goes false on failure too - otherwise the wait never ends. The failure is the next
    /// `poll`'s to report.
    #[must_use]
    pub fn is_handshaking(&self) -> bool {
        self.is_handshaking.load(Ordering::Relaxed) && self.error.lock().unwrap_or_else(PoisonError::into_inner).is_none()
    }

    /// Lets the networking thread go on delivering after the handshake.
    pub fn resume_receiving(&self) {
        self.may_resume.store(true, Ordering::Relaxed);
    }

    /// Disconnects and joins the networking thread. Safe mid-handshake: both the loop's timer
    /// and the wait to resume watch the running flag, so there is always something to join.
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
