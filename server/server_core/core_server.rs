use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};

use crate::libraries::events::EventManager;
use crate::server::server_core::chat::server_chat_on_event;
use crate::server::server_core::entities::ServerEntities;
use crate::server::server_core::items::ServerItems;
use crate::server::server_core::networking::{DisconnectEvent, NewConnectionEvent};
use crate::server::server_core::players::ServerPlayers;
use crate::server::server_ui::{ConsoleMessageType, PlayerEventType, ServerState, UiMessageType};
use crate::shared::versions::{WORLD_SAVE_HEADER_LEN, WORLD_SAVE_MAGIC, WORLD_SAVE_VERSION};

use super::blocks::ServerBlocks;
use super::commands::CommandManager;
use super::mod_manager::ServerModManager;
use super::networking::{BindAddress, ServerNetworking};
use super::walls::ServerWalls;
use super::world_generator::WorldGenerator;
use crate::libraries::serialization;

pub const SINGLEPLAYER_PORT: u16 = 49152;
pub const MULTIPLAYER_PORT: u16 = 49153;

/// The world a server generates when it is started without a save to load.
///
/// `min_width` is a floor rather than the exact width: the biome walk decides where the
/// world actually ends, so the generated world is at least this wide.
pub const DEFAULT_WORLD_MIN_WIDTH: i32 = 4400;
pub const DEFAULT_WORLD_HEIGHT: i32 = 1200;
pub const DEFAULT_WORLD_SEED: u64 = 423_657;

pub struct Server {
    pub tps_limit: f32,
    /// Size of the world to generate, as (`min_width`, height). Only read when there is
    /// no world to load. A field rather than a constant in `start` so a test can generate
    /// a world small enough to assert about all of.
    pub world_size: (i32, i32),
    pub world_seed: u64,
    pub state: Arc<Mutex<ServerState>>,
    events: EventManager,
    networking: ServerNetworking,
    mods: ServerModManager,
    blocks: ServerBlocks,
    walls: ServerWalls,
    entities: ServerEntities,
    items: ServerItems,
    players: ServerPlayers,
    ui_event_receiver: Option<Receiver<UiMessageType>>,
    commands: CommandManager,
    /// Simulated milliseconds already stepped, used to catch the fixed 5ms tick up to real time.
    ms_counter: i32,
    /// Whole seconds already stepped, used to rate limit entity syncing.
    seconds_counter: i32,
    /// Set on the first update, then used as the origin that `ms_counter` counts from.
    ms_timer: Option<std::time::Instant>,
    /// Time of the previous update, used to measure delta time.
    last_time: Option<std::time::Instant>,
}

impl Server {
    #[must_use]
    pub fn new(port: u16, bind_address: BindAddress, ui_event_receiver: Option<Receiver<UiMessageType>>, ui_event_sender: Option<Sender<UiMessageType>>) -> Self {
        send_to_ui(UiMessageType::ServerState(ServerState::Nothing), ui_event_sender); //this is useless but sets the ui event sender
        let blocks = ServerBlocks::new();
        let walls = ServerWalls::new(&mut blocks.get_blocks());
        let commands = CommandManager::new();
        Self {
            tps_limit: 20.0,
            world_size: (DEFAULT_WORLD_MIN_WIDTH, DEFAULT_WORLD_HEIGHT),
            world_seed: DEFAULT_WORLD_SEED,
            state: Arc::new(Mutex::new(ServerState::Nothing)),
            events: EventManager::new(),
            networking: ServerNetworking::new(port, bind_address),
            mods: ServerModManager::new(Vec::new()),
            blocks,
            walls,
            entities: ServerEntities::new(),
            items: ServerItems::new(),
            players: ServerPlayers::new(),
            ui_event_receiver,
            commands,
            ms_counter: 0,
            seconds_counter: 0,
            ms_timer: None,
            last_time: None,
        }
    }

    pub fn set_state(&self, server_state: ServerState) {
        *self.state.lock().unwrap_or_else(PoisonError::into_inner) = server_state;
        send_to_ui(UiMessageType::ServerState(server_state), None);
    }

    pub fn get_state(&self) -> ServerState {
        *self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn init_mod_interface(&mut self) -> Result<()> {
        let state = self.state.clone();
        self.mods.mod_manager.add_global_function("stop_server", move |_, ()| -> Result<_, rlua::Error> {
            *state.lock().unwrap_or_else(PoisonError::into_inner) = ServerState::Stopping;
            Ok(())
        })
    }

    /// Starts the server - manual way. It only inits the server but doesn't run a loop
    #[allow(clippy::too_many_lines)]
    pub fn start(&mut self, status_text: &Mutex<String>, mods_serialized: Vec<Vec<u8>>, world_path: &Path) -> Result<()> {
        print_to_console("Starting server...", 0);
        let timer = std::time::Instant::now();
        "Starting server".clone_into(&mut status_text.lock().unwrap_or_else(PoisonError::into_inner));
        self.set_state(ServerState::Starting);

        let mut mods = Vec::new();
        for game_mod in mods_serialized {
            // decompress mod with snap
            let game_mod = snap::raw::Decoder::new().decompress_vec(&game_mod)?;
            mods.push(serialization::deserialize(&game_mod)?);
        }
        self.mods = ServerModManager::new(mods);

        self.init_mod_interface()?;

        // init modules
        self.networking.init();
        self.blocks.init(&mut self.mods.mod_manager)?;
        self.walls.init(&mut self.mods.mod_manager)?;
        self.items.init(&mut self.mods.mod_manager, &self.entities.get_entities_arc())?;

        let generator = WorldGenerator::new();
        generator.init(&mut self.mods.mod_manager)?;

        self.set_state(ServerState::InitMods);
        print_to_console("initializing mods", 0);
        "Initializing mods".clone_into(&mut status_text.lock().unwrap_or_else(PoisonError::into_inner));
        self.mods.init()?;

        self.commands.init(&mut self.mods.mod_manager);

        if world_path.exists() {
            self.set_state(ServerState::LoadingWorld);
            print_to_console("loading world", 0);
            "Loading world".clone_into(&mut status_text.lock().unwrap_or_else(PoisonError::into_inner));
            self.load_world(world_path)?;
        } else {
            self.set_state(ServerState::GeneratingWorld);
            generator.generate(
                (&mut *self.blocks.get_blocks(), &mut self.walls.get_walls()),
                &mut self.mods.mod_manager,
                self.world_size.0,
                self.world_size.1,
                self.world_seed,
                status_text,
            )?;

            let width = self.blocks.get_blocks().get_size().0;
            let height = self.blocks.get_blocks().get_size().1;
            for x in 0..width as i32 {
                for y in 0..height as i32 {
                    self.blocks.update_block(x, y, &mut self.events)?;
                }
            }
        }

        self.set_state(ServerState::Running);

        print_to_console(&format!("server started in {}ms", timer.elapsed().as_millis()), 0);
        status_text.lock().unwrap_or_else(PoisonError::into_inner).clear();
        Ok(())
    }

    /// Runs the server - automated way. It starts (initializes) the server, runs it until it has top be stopped, then stops it and returns
    pub fn run(&mut self, is_running: &AtomicBool, status_text: &Mutex<String>, mods_serialized: Vec<Vec<u8>>, world_path: &Path) -> Result<()> {
        let mut last_time;

        self.start(status_text, mods_serialized, world_path)?;

        loop {
            last_time = std::time::Instant::now();

            self.update()?;

            // sleep
            let sleep_time = 1000.0 / self.tps_limit - last_time.elapsed().as_secs_f32() * 1000.0;
            if sleep_time > 0.0 {
                sleep(Duration::from_secs_f32(sleep_time / 1000.0));
            }

            if !is_running.load(Ordering::Relaxed) || self.get_state() == ServerState::Stopping {
                //state is there so outside events can stop it
                break;
            }
        }

        self.stop(status_text, world_path)?;

        Ok(())
    }

    /// Advances the frame timers and returns the tick origin and the previous frame time.
    /// Returns `None` on the first update, when there is not yet a previous frame to
    /// measure a delta against, so that update is skipped.
    ///
    /// The second guard is kept from the original implementation for safety, but is
    /// unreachable: the first branch always sets `last_time` alongside `ms_timer`.
    fn advance_timers(&mut self) -> Option<(std::time::Instant, std::time::Instant)> {
        let Some(ms_timer) = self.ms_timer else {
            self.ms_timer = Some(std::time::Instant::now());
            self.last_time = self.ms_timer;
            return None; //we skip this time
        };

        let Some(last_time) = self.last_time else {
            self.last_time = Some(std::time::Instant::now());
            return None; //we skip this time
        };

        self.last_time = Some(std::time::Instant::now());

        Some((ms_timer, last_time))
    }

    /// Updates the server - manual way. It updates the server once and returns
    pub fn update(&mut self) -> Result<()> {
        // the counters are private fields so outside functions cannot mismanage them
        let Some((ms_timer, last_time)) = self.advance_timers() else {
            return Ok(()); //we return early this time
        };

        let delta_time = last_time.elapsed().as_secs_f32() * 1000.0;

        // update modules
        self.networking.update(&mut self.events)?;
        self.mods.update()?;
        self.blocks.update(&mut self.events, delta_time)?;
        self.walls.update(delta_time, &mut self.events)?;
        self.items.update(&mut self.events);

        // handle events
        self.handle_events()?;

        while self.ms_counter < ms_timer.elapsed().as_millis() as i32 {
            self.players.update(
                &mut self.entities.get_entities(),
                &self.blocks.get_blocks(),
                &mut self.events,
                &self.items.get_items(),
                &mut self.networking,
            )?;
            self.entities.get_entities().update_entities_ms(&self.blocks.get_blocks(), &mut self.events)?;
            self.ms_counter += 5;
        }

        if self.seconds_counter < self.ms_counter / 1000 {
            self.entities.sync_entities(&mut self.networking)?;
            self.seconds_counter = self.ms_counter / 1000;
        }

        Ok(())
    }

    /// Stops the server - manual way. It stops the server and returns
    pub fn stop(&mut self, status_text: &Mutex<String>, world_path: &Path) -> Result<()> {
        if self.get_state() == ServerState::Stopped {
            //so we don't stop it twice
            return Ok(());
        }
        // stop modules
        self.networking.stop(&mut self.events)?;
        self.mods.stop()?;
        self.handle_events()?;

        self.set_state(ServerState::Stopping);
        print_to_console("saving world", 0);
        "Saving world".clone_into(&mut status_text.lock().unwrap_or_else(PoisonError::into_inner));

        self.save_world(world_path)?;

        print_to_console("stopping server", 0);
        "Stopping server".clone_into(&mut status_text.lock().unwrap_or_else(PoisonError::into_inner));

        self.set_state(ServerState::Stopped);
        status_text.lock().unwrap_or_else(PoisonError::into_inner).clear();
        print_to_console("server stopped.", 0);

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(receiver) = &self.ui_event_receiver {
            //goes through the messages received from the server
            while let Ok(UiMessageType::UiToSrvConsoleMessage(message)) = receiver.try_recv() {
                let feedback = self.commands.execute_command(&message, &mut self.mods.mod_manager, None);
                match feedback {
                    Ok(feedback) => print_to_console(&feedback, 0),
                    Err(val) => print_to_console(&val.to_string(), 1),
                }
            }
        }

        while let Some(event) = self.events.pop_event() {
            if let Some(disconnect) = event.downcast::<DisconnectEvent>() {
                send_to_ui(UiMessageType::PlayerEvent(PlayerEventType::Leave(disconnect.conn.address.addr())), None);
            }
            if let Some(connect) = event.downcast::<NewConnectionEvent>() {
                send_to_ui(UiMessageType::PlayerEvent(PlayerEventType::Join((connect.name.clone(), connect.conn.address.addr()))), None);
            }
            if let Some(event) = event.downcast::<UiMessageType>() {
                send_to_ui(event.clone(), None);
            }

            self.commands.on_event(&event, &self.players, &self.entities, &mut self.networking, &mut self.mods.mod_manager)?;

            self.mods.on_event(&event, &mut self.networking)?;
            self.blocks.on_event(
                &event,
                &mut self.events,
                &mut self.networking,
                &self.entities.get_entities(),
                &self.players,
                &self.items.get_items(),
                &mut self.mods.mod_manager,
            )?;
            self.walls.on_event(&event, &mut self.networking)?;
            self.items.on_event(&event, &mut self.entities.get_entities(), &mut self.events, &mut self.networking)?;
            self.players
                .on_event(&event, &mut self.entities.get_entities(), &self.blocks, &mut self.networking, &mut self.events, &self.items.get_items())?;
            ServerEntities::on_event(&event, &mut self.networking)?;
            self.networking.on_event(&event, &mut self.events)?;
            server_chat_on_event(&event, &mut self.networking)?;
        }

        Ok(())
    }

    /// Read only views of the world, for the integration tests.
    ///
    /// The game itself never reaches into a running server from outside - everything goes
    /// through events and packets - so these exist purely so a test can check what the
    /// simulation actually did. Each one takes the same lock the server does, so a test
    /// must drop the guard before stepping the server again.
    /// True once the networking thread has bound the port, so a test knows when it is safe
    /// to connect without probing the port and racing that bind.
    #[cfg(test)]
    #[must_use]
    pub fn is_listening(&self) -> bool {
        self.networking.is_listening()
    }

    #[cfg(test)]
    pub fn get_blocks(&self) -> std::sync::MutexGuard<'_, crate::shared::blocks::Blocks> {
        self.blocks.get_blocks()
    }

    #[cfg(test)]
    pub fn get_walls(&self) -> std::sync::MutexGuard<'_, crate::shared::walls::Walls> {
        self.walls.get_walls()
    }

    #[cfg(test)]
    pub fn get_items(&self) -> std::sync::MutexGuard<'_, crate::shared::items::Items> {
        self.items.get_items()
    }

    #[cfg(test)]
    pub fn get_entities(&self) -> std::sync::MutexGuard<'_, crate::shared::entities::Entities> {
        self.entities.get_entities()
    }

    /// The mods as the server loaded them, so a test can call into their lua.
    #[cfg(test)]
    pub const fn get_mods(&mut self) -> &mut crate::shared::mod_manager::ModManager {
        &mut self.mods.mod_manager
    }

    /// Runs a command the way a player typing it in chat would.
    #[cfg(test)]
    pub fn execute_command(&mut self, command: &str) -> Result<String> {
        self.commands.execute_command(command, &mut self.mods.mod_manager, None)
    }

    fn load_world(&mut self, world_path: &Path) -> Result<()> {
        let world_file = std::fs::read(world_path)?;
        // The header is read before the body on purpose, so that a world this build cannot
        // read is *named* rather than handed to a decoder that will make nonsense of it.
        let body = read_world_header(&world_file)?;

        let world: HashMap<String, Vec<u8>> = serialization::deserialize(body)?;
        self.blocks.get_blocks().deserialize(world.get("blocks").unwrap_or(&Vec::new()))?;
        self.walls.get_walls().deserialize(world.get("walls").unwrap_or(&Vec::new()))?;
        self.players.deserialize(world.get("players").unwrap_or(&Vec::new()))?;
        Ok(())
    }

    fn save_world(&self, world_path: &Path) -> Result<()> {
        let mut world = HashMap::new();
        world.insert("blocks".to_owned(), self.blocks.get_blocks().serialize()?);
        world.insert("walls".to_owned(), self.walls.get_walls().serialize()?);
        world.insert("players".to_owned(), self.players.serialize()?);

        let mut world_file = world_save_header();
        serialization::serialize_into(&mut world_file, &world)?;
        if !world_path.exists() {
            std::fs::create_dir_all(world_path.parent().ok_or_else(|| anyhow!("could not get parent folder"))?)?;
        }
        std::fs::write(world_path, world_file)?;
        Ok(())
    }
}

/// The bytes every world file starts with: the magic, then the save version.
///
/// Both are fixed width and little endian, deliberately not touched by the serializer - see
/// `WORLD_SAVE_MAGIC`.
#[must_use]
pub fn world_save_header() -> Vec<u8> {
    let mut header = Vec::with_capacity(WORLD_SAVE_HEADER_LEN);
    header.extend_from_slice(WORLD_SAVE_MAGIC);
    header.extend_from_slice(&WORLD_SAVE_VERSION.to_le_bytes());
    header
}

/// Checks a world file's header and returns the body after it.
///
/// Every rejection here names what is wrong, which is the entire reason the header exists.
fn read_world_header(file: &[u8]) -> Result<&[u8]> {
    let Some((header, body)) = file.split_at_checked(WORLD_SAVE_HEADER_LEN) else {
        bail!("this world file is too short to be a world - it is {} bytes", file.len());
    };
    let (magic, version) = header.split_at(WORLD_SAVE_MAGIC.len());

    if magic != WORLD_SAVE_MAGIC {
        bail!("this world was saved by a build older than the versioned save format (save version 2 or earlier) and cannot be read");
    }

    let version = u32::from_le_bytes(version.try_into().unwrap_or([0; 4]));
    if version != WORLD_SAVE_VERSION {
        bail!("this world is save version {version}, but this build reads version {WORLD_SAVE_VERSION}");
    }
    Ok(body)
}

/// The channel back to the server ui. It is process global because `print_to_console` and
/// `send_to_ui` are free functions called from all over the server, which have no `Server`
/// to reach through. The first non-`None` sender handed in wins; later ones are ignored.
///
/// A `Sender` is `Send` but not `Sync`, so this cannot be a `OnceLock` - it needs the `Mutex`.
static UI_EVENT_SENDER: Mutex<Option<Sender<UiMessageType>>> = Mutex::new(None);

/// sends any data to the ui if the server was started without nogui flag
pub fn send_to_ui(data: UiMessageType, ui_event_sender: Option<Sender<UiMessageType>>) {
    let mut sender = UI_EVENT_SENDER.lock().unwrap_or_else(PoisonError::into_inner);

    if sender.is_none() {
        *sender = ui_event_sender;
    }

    if let Some(sender) = sender.as_ref() {
        if sender.send(data).is_err() {
            println!("error sending data to ui");
        }
    }
}

/// prints to the terminal the server was started in and sends it to the ui
pub fn print_to_console(text: &str, warn_level: u8) {
    if text.is_empty() {
        return;
    }

    if text.contains('\n') {
        for line in text.split('\n') {
            print_to_console(line, warn_level);
        }
        return;
    }

    let mut formatted_text;
    if warn_level == 0 {
        formatted_text = format!("[INFO] {text}");
    } else if warn_level == 1 {
        formatted_text = format!("[WARNING] {text}");
    } else {
        formatted_text = format!("[ERROR] {text}");
    }
    formatted_text = format_timestamp(&formatted_text);
    println!("{formatted_text}");
    let text_with_type = match warn_level {
        0 => ConsoleMessageType::Info(formatted_text),
        1 => ConsoleMessageType::Warning(formatted_text),
        _ => ConsoleMessageType::Error(formatted_text),
    };
    send_to_ui(UiMessageType::SrvToUiConsoleMessage(text_with_type), None);
}

/// This function formats the string to add the timestamp
fn format_timestamp(message: &String) -> String {
    let timestamp = chrono::Local::now().naive_local().and_utc().timestamp();
    let timestamp = chrono::DateTime::from_timestamp(timestamp, 0);
    format!("[{}] {}", timestamp.map_or_else(|| "???".to_owned(), |time| time.format("%m-%d %H:%M:%S").to_string(),), message)
}
