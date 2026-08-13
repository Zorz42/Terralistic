//! Shared machinery for the integration tests: temporary worlds, free ports, and small
//! drivers that step a real server and a real client by hand instead of running their
//! main loops.
//!
//! Everything here is `#[cfg(test)]` by virtue of the parent module, so none of it is
//! compiled into the shipped binary.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::let_underscore_must_use)] // a failing helper should fail the test loudly

use std::collections::HashMap;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::Result;
use message_io::network::{Endpoint, NetEvent, SendStatus, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler};

use crate::client::game::{ClientNetworking, WelcomePacketEvent};
use crate::libraries::events::EventManager;
use crate::libraries::serialization;
use crate::server::server_core::world_save_header;
use crate::server::server_core::{BindAddress, Connection, DisconnectEvent, NewConnectionEvent, PacketFromClientEvent, SavedPlayerData, Server, ServerNetworking};
use crate::shared::blocks::Blocks;
use crate::shared::packet::{Packet, WelcomeCompletePacket};
use crate::shared::walls::Walls;

/// The same mod bytes the real game ships, so these tests exercise the lua that players
/// actually run rather than a stub.
pub const BASE_GAME_MOD: &[u8] = include_bytes!("../base_game/base_game.mod");

/// How long a test waits for the network before giving up. Generous, because it only
/// matters when something is broken - a passing test reaches its condition in
/// milliseconds.
const TIMEOUT: Duration = Duration::from_secs(30);

/// A port no other test in this process is using, and that nothing else holds right now.
///
/// Tests run in parallel threads of one process, so they cannot share the game's fixed
/// ports. Asking the OS for port 0 is not enough on its own: two tests that probe one
/// after the other can be handed the same port, and then one test's client connects to
/// the other test's server. The counter is what makes the numbers distinct; the bind is
/// only there to skip ports something outside this process already holds.
///
/// The range sits below the ephemeral range so it does not fight the OS for numbers, and
/// clear of the game's own 49152/49153.
pub fn free_port() -> u16 {
    static NEXT_PORT: AtomicU64 = AtomicU64::new(0);
    const FIRST_PORT: u64 = 41_000;
    const PORT_COUNT: u64 = 4_000;

    for _ in 0..PORT_COUNT {
        let port = (FIRST_PORT + NEXT_PORT.fetch_add(1, Ordering::Relaxed) % PORT_COUNT) as u16;
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
            drop(listener);
            return port;
        }
    }
    panic!("no free port in the test range");
}

/// Runs `step` until it returns true, or panics with `what` when the timeout runs out.
pub fn wait_until(what: &str, mut step: impl FnMut() -> Result<bool>) {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        if step().unwrap() {
            return;
        }
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// A directory under the system temp dir that deletes itself when it goes out of scope.
///
/// The crate has no dev-dependencies and this is all the tests need from one, so it is
/// spelled out here rather than pulling in `tempfile`.
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new(tag: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let path = std::env::temp_dir().join(format!("terralistic-test-{tag}-{}-{}", std::process::id(), COUNTER.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir_all(&path).expect("could not create a temp dir for the test");
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The path a `Server` should save to, matching the layout the game uses.
    pub fn world_path(&self) -> PathBuf {
        self.path.join("server.world")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // a failed cleanup must not turn a passing test red, so this is best effort
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Writes a world save of `size` full of air, so a `Server` started against it takes the
/// load path instead of generating the stock 4400x1200 world.
///
/// The ids written here line up with the ones the server will have after it loads mods:
/// `Blocks::new` registers air first and `Walls::new` registers clear first, so both are
/// id 0 on either side. That is the same assumption the real save format makes - see the
/// note about registries not being saved in `CLAUDE.md`.
pub fn write_world_save(path: &Path, size: (u32, u32)) {
    let mut blocks = Blocks::new();
    blocks.create(size);

    let mut walls = Walls::new(&mut blocks);
    walls.create_from_wall_ids(&vec![vec![walls.clear; size.1 as usize]; size.0 as usize]).unwrap();

    let mut world = HashMap::new();
    world.insert("blocks".to_owned(), blocks.serialize().unwrap());
    world.insert("walls".to_owned(), walls.serialize().unwrap());
    world.insert("players".to_owned(), serialization::serialize(&HashMap::<String, SavedPlayerData>::new()).unwrap());

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut file = world_save_header();
    serialization::serialize_into(&mut file, &world).unwrap();
    std::fs::write(path, file).unwrap();
}

/// A full `Server` on a temp world, stepped by hand.
///
/// `Server::run` owns its own loop and only returns once the server stops, which is no
/// use to a test that wants to look at the world halfway through. This drives `start`,
/// `update` and `stop` directly, which is the documented "manual way".
pub struct TestServer {
    pub server: Server,
    status: Mutex<String>,
    dir: TempDir,
    pub port: u16,
}

impl TestServer {
    /// Starts a server on a small, freshly written world. Fast, because world generation
    /// is skipped entirely - most tests only need somewhere for blocks to live.
    pub fn start_on_small_world(tag: &str, size: (u32, u32)) -> Result<Self> {
        let dir = TempDir::new(tag);
        write_world_save(&dir.world_path(), size);
        Self::start_in(dir, tag)
    }

    /// Starts a server on an empty directory, so it generates a world instead of loading
    /// one. `size` is (`min_width`, height); the biome walk decides the real width.
    pub fn start_on_generated_world(tag: &str, size: (i32, i32), seed: u64) -> Result<Self> {
        let mut this = Self::new_in(TempDir::new(tag));
        this.server.world_size = size;
        this.server.world_seed = seed;
        this.start()?;
        Ok(this)
    }

    /// Starts a server on an existing directory, which is how a save is reloaded.
    pub fn start_in(dir: TempDir, _tag: &str) -> Result<Self> {
        let mut this = Self::new_in(dir);
        this.start()?;
        Ok(this)
    }

    fn new_in(dir: TempDir) -> Self {
        let port = free_port();
        Self {
            server: Server::new(port, BindAddress::Loopback, None, None),
            status: Mutex::new(String::new()),
            dir,
            port,
        }
    }

    fn start(&mut self) -> Result<()> {
        let world_path = self.dir.world_path();
        self.server.start(&self.status, vec![BASE_GAME_MOD.to_vec()], &world_path)?;
        wait_until_listening(|| self.server.is_listening());
        Ok(())
    }

    /// Steps the server, letting a little real time pass first.
    ///
    /// The server measures its own frame length and feeds it to anything that advances
    /// over time - block breaking, physics - as whole milliseconds. Stepped in a tight
    /// loop the delta truncates to zero and none of that ever moves, so a test that
    /// waits for something to finish breaking has to let the clock run.
    pub fn update_slowly(&mut self) -> Result<()> {
        std::thread::sleep(Duration::from_millis(2));
        self.server.update()
    }

    pub fn world_path(&self) -> PathBuf {
        self.dir.world_path()
    }

    /// Steps the server the given number of times, at no particular rate.
    ///
    /// The first update is always a no-op: `advance_timers` uses it to set the origin the
    /// tick counter measures from.
    pub fn update_times(&mut self, times: u32) -> Result<()> {
        for _ in 0..times {
            self.server.update()?;
        }
        Ok(())
    }

    /// Stops the server and saves the world, returning the directory so a second server
    /// can be started on it.
    pub fn stop(mut self) -> Result<TempDir> {
        let world_path = self.dir.world_path();
        self.server.stop(&self.status, &world_path)?;
        Ok(self.dir)
    }
}

/// Starts a server that is expected to refuse the world it is given, and returns the
/// message it refused with.
///
/// `unwrap_err` would need `TestServer` to be `Debug`, which would mean deriving it
/// through a `Server` and everything it owns for no other reason.
pub fn expect_start_error(dir: TempDir, tag: &str) -> String {
    match TestServer::start_in(dir, tag) {
        Ok(_) => panic!("the server started on a world it should have refused"),
        Err(error) => error.to_string(),
    }
}

/// The server's networking layer with the event plumbing a `Server` would give it,
/// and a record of what arrived.
pub struct NetServer {
    pub net: ServerNetworking,
    pub events: EventManager,
    /// Names from every `NewConnectionEvent`, in arrival order.
    pub joined: Vec<String>,
    /// Connections that produced a `DisconnectEvent`.
    pub disconnected: Vec<Connection>,
    /// Every packet that was not part of the handshake.
    pub packets: Vec<(Packet, Connection)>,
    /// Connections that completed the handshake, in arrival order.
    pub connections: Vec<Connection>,
}

impl NetServer {
    /// Starts the server and waits until it is actually listening.
    ///
    /// `init` only spawns the thread that binds the port, so a client that connects the
    /// instant this returns can be refused before the listener exists. The game never
    /// notices because its server has a world to generate first; a test has no such
    /// pause, so it waits for the networking layer to report that it has bound.
    pub fn start(port: u16) -> Self {
        let mut net = ServerNetworking::new(port, BindAddress::Loopback);
        net.init();
        wait_until_listening(|| net.is_listening());
        Self {
            net,
            events: EventManager::new(),
            joined: Vec::new(),
            disconnected: Vec::new(),
            packets: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// One turn of the server's event loop, cut down to the networking parts: drain the
    /// network thread into the event queue, then offer every event back to networking so
    /// it replies to handshakes. Handlers push new events onto the same queue, so those
    /// are drained in this pass too, exactly as `Server::handle_events` does.
    pub fn pump(&mut self) -> Result<()> {
        self.pump_inner(true)
    }

    /// The same, except events are recorded and then dropped instead of being answered.
    ///
    /// This models a server that accepts the socket and then never welcomes anyone,
    /// which is what a client sees when the server refuses it.
    pub fn pump_without_answering(&mut self) -> Result<()> {
        self.pump_inner(false)
    }

    fn pump_inner(&mut self, answer: bool) -> Result<()> {
        self.net.update(&mut self.events)?;

        while let Some(event) = self.events.pop_event() {
            if let Some(event) = event.downcast::<NewConnectionEvent>() {
                self.joined.push(event.name.clone());
                self.connections.push(event.conn.clone());
            }
            if let Some(event) = event.downcast::<DisconnectEvent>() {
                self.disconnected.push(event.conn.clone());
            }
            if let Some(event) = event.downcast::<PacketFromClientEvent>() {
                self.packets.push((clone_packet(&event.packet), event.conn.clone()));
            }

            if answer {
                self.net.on_event(&event, &mut self.events)?;
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.net.stop(&mut self.events)
    }
}

/// A real `ClientNetworking` plus a record of what it received.
pub struct TestClient {
    pub net: ClientNetworking,
    pub events: EventManager,
    /// Packets received while the client was still in the welcome phase.
    pub welcome_packets: Vec<Packet>,
    /// Packets received after the handshake finished.
    pub packets: Vec<Packet>,
}

impl TestClient {
    /// Connects to a server on loopback and sends the handshake, as `run_game` does.
    pub fn connect(port: u16, name: &str) -> Result<Self> {
        let mut net = ClientNetworking::new(port, "127.0.0.1".to_owned());
        net.init(name.to_owned())?;
        Ok(Self {
            net,
            events: EventManager::new(),
            welcome_packets: Vec::new(),
            packets: Vec::new(),
        })
    }

    /// Drains whatever the networking thread has produced.
    ///
    /// Releasing the thread once welcoming is over is the client's job, not the
    /// networking layer's: the thread parks after the welcome packet until the game says
    /// it is ready. `run_game` does this once; doing it on every pump is equivalent,
    /// since it only sets a flag.
    pub fn pump(&mut self) -> Result<()> {
        self.net.update(&mut self.events)?;
        if !self.net.is_welcoming() {
            self.net.start_receiving();
        }

        while let Some(event) = self.events.pop_event() {
            if let Some(event) = event.downcast::<WelcomePacketEvent>() {
                self.welcome_packets.push(clone_packet(&event.packet));
            } else if let Some(packet) = event.downcast::<Packet>() {
                self.packets.push(clone_packet(packet));
            }
        }

        Ok(())
    }

    /// True once any welcome or normal packet deserializes as `T`.
    pub fn received<T: serde::de::DeserializeOwned + 'static>(&self) -> bool {
        self.find::<T>().is_some()
    }

    /// The first packet of type `T`, from either phase.
    pub fn find<T: serde::de::DeserializeOwned + 'static>(&self) -> Option<T> {
        self.welcome_packets.iter().chain(self.packets.iter()).find_map(Packet::try_deserialize::<T>)
    }

    /// Disconnects and joins the networking thread.
    ///
    /// Only safe once the handshake has finished one way or the other: while welcoming,
    /// the thread never looks at the running flag, so joining it would block forever.
    /// `run_game` has the same constraint - it only stops a client it has already waited
    /// for - so tests use `complete_handshake` first rather than working around it.
    pub fn stop(&mut self) -> Result<()> {
        self.net.stop()
    }
}

/// Blocks until the networking layer says it has bound its port.
///
/// Both servers bind on a background thread, so a client that connects the instant the
/// constructor returns can be refused before the listener exists.
///
/// This asks the server rather than probing the port, and that is the whole point. The
/// obvious check - "has the port stopped being bindable" - has to *bind* the port to find
/// out, and a second listener on an address someone else is binding is exactly the
/// collision it is looking for. Polling every millisecond, it regularly won the race, and
/// the server's own bind then failed with `AddrInUse`, killed the networking thread, and
/// left the probe waiting the full 30 seconds for a listener that would never exist. That
/// was a 1-in-10 flake across the whole integration suite, landing on whichever test
/// happened to lose the coin toss.
pub fn wait_until_listening(is_listening: impl Fn() -> bool) {
    wait_until("the server to bind its port", || Ok(is_listening()));
}

/// Connects a client to a running `Server` and steps both until the world has arrived.
///
/// This is the whole join sequence: the client sends its version and name, the server
/// answers with the mods, the blocks, the walls and finally the welcome, and only then
/// does the client stop welcoming.
pub fn join(server: &mut TestServer, name: &str) -> Result<TestClient> {
    let mut client = TestClient::connect(server.port, name)?;
    wait_until("the client to join the server", || {
        server.server.update()?;
        client.pump()?;
        Ok(client.received::<WelcomeCompletePacket>())
    });
    Ok(client)
}

/// Steps both sides until the client has been welcomed.
pub fn complete_handshake(server: &mut NetServer, client: &mut TestClient) {
    wait_until("the handshake to complete", || {
        server.pump()?;
        client.pump()?;
        Ok(client.received::<WelcomeCompletePacket>())
    });
}

/// A client that speaks the wire protocol directly, with none of `ClientNetworking`'s
/// rules about what to send when.
///
/// `ClientNetworking` always sends a correct version first, so it cannot be used to check
/// what the server does with a client that does not - which is exactly the case the
/// version handshake exists for. This can send anything, including bytes that are not a
/// packet at all.
pub struct RawClient {
    handler: NodeHandler<()>,
    endpoint: Endpoint,
    received: Arc<Mutex<Vec<Vec<u8>>>>,
    disconnected: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl RawClient {
    pub fn connect(port: u16) -> Result<Self> {
        let (handler, listener) = node::split::<()>();
        let (endpoint, _) = handler.network().connect(Transport::FramedTcp, format!("127.0.0.1:{port}"))?;

        let received = Arc::new(Mutex::new(Vec::new()));
        let disconnected = Arc::new(AtomicBool::new(false));

        let thread_received = received.clone();
        let thread_disconnected = disconnected.clone();
        let thread = std::thread::Builder::new().name("Raw test client".to_owned()).spawn(move || {
            listener.for_each(move |event| {
                if let NodeEvent::Network(event) = event {
                    match event {
                        NetEvent::Message(_, data) => thread_received.lock().unwrap().push(data.to_vec()),
                        NetEvent::Disconnected(_) => thread_disconnected.store(true, Ordering::Relaxed),
                        NetEvent::Connected(..) | NetEvent::Accepted(..) => {}
                    }
                }
            });
        })?;

        Ok(Self {
            handler,
            endpoint,
            received,
            disconnected,
            thread: Some(thread),
        })
    }

    /// Sends bytes as one frame, whatever they are.
    ///
    /// Retries while the socket is still connecting, the same way both real networking
    /// layers do, and gives up once the server has dropped us - which is a normal outcome
    /// for the tests that send something the server rejects.
    pub fn send_bytes(&self, bytes: &[u8]) {
        let deadline = Instant::now() + TIMEOUT;
        while Instant::now() < deadline {
            match self.handler.network().send(self.endpoint, bytes) {
                SendStatus::Sent | SendStatus::ResourceNotFound => return,
                SendStatus::MaxPacketSizeExceeded => panic!("test tried to send an oversized frame"),
                SendStatus::ResourceNotAvailable => std::thread::sleep(Duration::from_millis(1)),
            }
        }
        panic!("raw client could not send within the timeout");
    }

    /// Sends a value the way the game does: wrapped in a `Packet`, then serialized.
    pub fn send<T: serde::Serialize + 'static>(&self, value: T) {
        self.send_bytes(&serialization::serialize(&Packet::new(value).unwrap()).unwrap());
    }

    /// Everything received so far that parses as a `Packet`.
    pub fn received_packets(&self) -> Vec<Packet> {
        self.received.lock().unwrap().iter().filter_map(|bytes| serialization::deserialize::<Packet>(bytes).ok()).collect()
    }

    pub fn is_disconnected(&self) -> bool {
        self.disconnected.load(Ordering::Relaxed)
    }
}

impl Drop for RawClient {
    fn drop(&mut self) {
        self.handler.stop();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// `Packet` is deliberately not `Clone` - it is a wire type, and copying one is normally
/// a mistake. The tests keep a log of what arrived, so they need a copy.
pub fn clone_packet(packet: &Packet) -> Packet {
    Packet {
        id: packet.id,
        data: packet.data.clone(),
    }
}
