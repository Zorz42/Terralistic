use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Result};

use crate::libraries::events::EventManager;
use crate::libraries::log::{log, LogLevel};
use crate::libraries::timing::{DeltaTimer, FixedStep};
use crate::server::server_core::chat::server_chat_on_event;
use crate::server::server_core::entities::ServerEntities;
use crate::server::server_core::items::ServerItems;
use crate::server::server_core::networking::{DisconnectEvent, NewConnectionEvent};
use crate::server::server_core::players::ServerPlayers;
use crate::server::server_ui::{ConsoleMessageType, PlayerEventType, ServerState, UiMessageType};
use crate::shared::versions::WORLD_SAVE_FORMAT;
use crate::shared::TICK_MS;

use super::blocks::ServerBlocks;
use super::commands::CommandManager;
use super::liquids::ServerLiquids;
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
    liquids: ServerLiquids,
    entities: ServerEntities,
    items: ServerItems,
    players: ServerPlayers,
    ui_event_receiver: Option<Receiver<UiMessageType>>,
    commands: CommandManager,
    /// The fixed 5ms tick the player and entity physics run on, caught up to real time.
    simulation_tick: FixedStep,
    /// Ticks simulated since the server started. The origin every timestamp on the wire
    /// counts from, handed to each client in its `WelcomeCompletePacket`.
    current_tick: u64,
    /// Whole seconds already stepped, used to rate limit entity syncing.
    seconds_counter: i32,
    /// Measures how long the previous update took.
    delta_timer: DeltaTimer,
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
            liquids: ServerLiquids::new(),
            entities: ServerEntities::new(),
            items: ServerItems::new(),
            players: ServerPlayers::new(),
            ui_event_receiver,
            commands,
            simulation_tick: FixedStep::new(TICK_MS),
            current_tick: 0,
            seconds_counter: 0,
            delta_timer: DeltaTimer::new(),
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
        self.liquids.init(&mut self.mods.mod_manager)?;
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
                (&mut *self.blocks.get_blocks(), &mut self.walls.get_walls(), &mut self.liquids.get_liquids()),
                &mut self.mods.mod_manager,
                self.world_size.0,
                self.world_size.1,
                self.world_seed,
                status_text,
            )?;

            // Only the multiblocks need updating, and this grows them where they stand.
            //
            // This used to be `update_block` on every cell in the world, and the cost was not
            // the sweep itself: it pushed a `BlockUpdateEvent` per cell, 5.4 million of them for
            // the default world, which sat in the queue until the first `update()` and were then
            // offered to every subsystem and handed to lua's `on_block_update` one at a time.
            // That first update took 18 seconds in a debug build and held about a gigabyte of
            // queued events - all of it after the loading screen had closed and while the client
            // was already connected and waiting to be welcomed, so joining a world that had just
            // been generated looked exactly like a game that had hung.
            self.blocks.expand_big_blocks(&mut self.events)?;
        }

        // The liquid grid is the same size as the block grid, always. A save that disagrees
        // with the world it was saved next to is a save whose liquids cannot be trusted, so
        // it starts dry rather than reading a cell of a different world.
        let world_size = self.blocks.get_blocks().get_size();
        if self.liquids.get_liquids().get_size() == world_size {
            // a grid that came out of a save may have been mid-splash when it was written,
            // and the scheduled set is derived rather than saved, so it is rebuilt here
            self.liquids.get_liquids().schedule_all_unsettled(&self.blocks.get_blocks())?;
        } else {
            self.liquids.get_liquids().create(world_size);
        }

        self.set_state(ServerState::Running);

        print_to_console(&format!("server started in {}ms", timer.elapsed().as_millis()), 0);
        status_text.lock().unwrap_or_else(PoisonError::into_inner).clear();
        Ok(())
    }

    /// Runs the server - automated way. It starts (initializes) the server, runs it until it has top be stopped, then stops it and returns
    ///
    /// A failure anywhere inside still shuts the networking thread down, which is the whole
    /// reason this wraps `run_until_stopped` rather than being it. `start` binds the port early
    /// and every step after it can fail, so a `?` used to return while that thread was still
    /// running - and the `Server` that owns the receiving end of its channel was dropped
    /// immediately afterwards. The thread went on holding the port and accepting connections it
    /// could no longer tell anyone about: the client that connected to one was accepted and then
    /// never welcomed, the server printed `Failed to send NewConnectionEvent: sending on a closed
    /// channel`, and the player watched a loading screen that would never move. Worse, the port
    /// stayed taken for the rest of the process, so every world opened afterwards failed to bind
    /// and hung the same way.
    ///
    /// The world is deliberately not saved on that path. Whatever went wrong may well be about
    /// the world itself, and overwriting the save with it is not a favour to anyone.
    pub fn run(&mut self, is_running: &AtomicBool, status_text: &Mutex<String>, mods_serialized: Vec<Vec<u8>>, world_path: &Path) -> Result<()> {
        let result = self.run_until_stopped(is_running, status_text, mods_serialized, world_path);

        if result.is_err() {
            if let Err(error) = self.networking.stop(&mut self.events) {
                print_to_console(&format!("could not stop networking after the server failed: {error}"), 2);
            }
            // so that anything that stops this server afterwards does not try to save the world
            // it never finished starting
            self.set_state(ServerState::Stopped);
        }

        result
    }

    fn run_until_stopped(&mut self, is_running: &AtomicBool, status_text: &Mutex<String>, mods_serialized: Vec<Vec<u8>>, world_path: &Path) -> Result<()> {
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

    /// Updates the server - manual way. It updates the server once and returns
    pub fn update(&mut self) -> Result<()> {
        // The first update has no previous one to measure a delta against, so it is skipped
        // rather than handed however long starting the server took.
        let Some(delta_time) = self.delta_timer.tick() else {
            return Ok(());
        };

        // update modules
        self.networking.update(&mut self.events)?;
        self.mods.update()?;
        self.blocks.update(&mut self.events);
        self.walls.update(delta_time, &mut self.events)?;
        self.liquids.update(&mut self.events, &mut self.networking)?;
        self.items.update(&mut self.events);

        // handle events
        self.handle_events()?;

        while self.simulation_tick.step() {
            self.current_tick += 1;
            self.networking.set_current_tick(self.current_tick);
            // everything the simulation owns advances on this clock, so every event it
            // produces can be named by a tick number rather than by when a frame happened
            self.blocks.tick(&mut self.events)?;
            self.liquids.tick(&self.blocks.get_blocks(), &mut self.events)?;
            self.players.update(
                self.current_tick,
                &mut self.entities.get_entities(),
                &self.blocks.get_blocks(),
                &self.liquids.get_liquids(),
                &mut self.events,
                &self.items.get_items(),
                &mut self.networking,
            )?;
            self.entities
                .get_entities()
                .update_entities_ms(&self.blocks.get_blocks(), &self.liquids.get_liquids(), &mut self.events)?;
        }

        let simulated_seconds = (self.simulation_tick.stepped_ms() / 1000) as i32;
        if self.seconds_counter < simulated_seconds {
            self.entities.sync_entities(self.current_tick, &mut self.networking)?;
            self.seconds_counter = simulated_seconds;
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
                send_to_ui(UiMessageType::PlayerEvent(PlayerEventType::Leave(disconnect.conn.addr())), None);
            }
            if let Some(connect) = event.downcast::<NewConnectionEvent>() {
                send_to_ui(UiMessageType::PlayerEvent(PlayerEventType::Join((connect.name.clone(), connect.conn.addr()))), None);
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
            self.liquids.on_event(&event, &mut self.networking)?;
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

    /// How many events are waiting to be handled, for the test that checks generation does not
    /// leave one per block behind it.
    #[cfg(test)]
    #[must_use]
    pub const fn get_current_tick(&self) -> u64 {
        self.current_tick
    }

    #[cfg(test)]
    #[must_use]
    pub fn queued_event_count(&self) -> usize {
        self.events.queued_count()
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
    pub fn get_liquids(&self) -> std::sync::MutexGuard<'_, crate::shared::liquids::Liquids> {
        self.liquids.get_liquids()
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
    pub const fn get_mods(&mut self) -> &mut crate::libraries::scripting::ScriptHost {
        &mut self.mods.mod_manager
    }

    /// Runs a command the way a player typing it in chat would.
    #[cfg(test)]
    pub fn execute_command(&mut self, command: &str) -> Result<String> {
        self.commands.execute_command(command, &mut self.mods.mod_manager, None)
    }

    fn load_world(&mut self, world_path: &Path) -> Result<()> {
        // The header is checked before the body on purpose, so that a world this build
        // cannot read is *named* rather than handed to a decoder that makes nonsense of it.
        let world = WORLD_SAVE_FORMAT.read(&std::fs::read(world_path)?)?;
        self.blocks.get_blocks().deserialize(world.get("blocks").unwrap_or(&Vec::new()))?;
        self.walls.get_walls().deserialize(world.get("walls").unwrap_or(&Vec::new()))?;
        self.players.deserialize(world.get("players").unwrap_or(&Vec::new()))?;
        // Liquids arrived with save version 4, so every world this build will read has the
        // key. It is still handled rather than unwrapped into an empty vector, because a
        // missing grid means a dry world, not a decode error - `start` sizes it afterwards.
        if let Some(liquids) = world.get("liquids") {
            self.liquids.get_liquids().deserialize(liquids)?;
        }
        Ok(())
    }

    fn save_world(&self, world_path: &Path) -> Result<()> {
        let mut world = HashMap::new();
        world.insert("blocks".to_owned(), self.blocks.get_blocks().serialize()?);
        world.insert("walls".to_owned(), self.walls.get_walls().serialize()?);
        world.insert("liquids".to_owned(), self.liquids.get_liquids().serialize()?);
        world.insert("players".to_owned(), self.players.serialize()?);

        let world_file = WORLD_SAVE_FORMAT.write(&world)?;
        if !world_path.exists() {
            std::fs::create_dir_all(world_path.parent().ok_or_else(|| anyhow!("could not get parent folder"))?)?;
        }
        std::fs::write(world_path, world_file)?;
        Ok(())
    }
}

/// The bytes every world file starts with, for the tests that check them.
#[cfg(test)]
#[must_use]
pub fn world_save_header() -> Vec<u8> {
    WORLD_SAVE_FORMAT.header()
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

    if sender.is_none() && ui_event_sender.is_some() {
        *sender = ui_event_sender;
        drop(sender);
        install_console_sink();
        sender = UI_EVENT_SENDER.lock().unwrap_or_else(PoisonError::into_inner);
    }

    if let Some(sender) = sender.as_ref() {
        if sender.send(data).is_err() {
            println!("error sending data to ui");
        }
    }
}

/// Prints to the terminal the server was started in. The server ui, if there is one, is
/// reached through the log sink installed in `install_console_sink`.
pub fn print_to_console(text: &str, warn_level: u8) {
    log(
        match warn_level {
            0 => LogLevel::Info,
            1 => LogLevel::Warning,
            _ => LogLevel::Error,
        },
        text,
    );
}

/// Points the log library at the server ui, so anything logged from anywhere reaches the
/// console panel as well as the terminal.
///
/// Installed once, by whoever first hands `send_to_ui` a sender.
fn install_console_sink() {
    crate::libraries::log::set_sink(Box::new(|level, line| {
        let message = match level {
            LogLevel::Info => ConsoleMessageType::Info(line.to_owned()),
            LogLevel::Warning => ConsoleMessageType::Warning(line.to_owned()),
            LogLevel::Error => ConsoleMessageType::Error(line.to_owned()),
        };
        send_to_ui(UiMessageType::SrvToUiConsoleMessage(message), None);
    }));
}
