use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Mutex, PoisonError};
use std::thread::sleep;

use anyhow::{anyhow, Result};

use crate::libraries::events::EventManager;
use crate::server::server_core::chat::server_chat_on_event;
use crate::server::server_core::entities::ServerEntities;
use crate::server::server_core::items::ServerItems;
use crate::server::server_core::networking::{DisconnectEvent, NewConnectionEvent};
use crate::server::server_core::players::ServerPlayers;
use crate::server::server_ui::{ConsoleMessageType, PlayerEventType, ServerState, UiMessageType};

use super::blocks::ServerBlocks;
use super::commands::{Command, CommandManager};
use super::mod_manager::ServerModManager;
use super::networking::ServerNetworking;
use super::walls::ServerWalls;
use super::world_generator::WorldGenerator;

pub const SINGLEPLAYER_PORT: u16 = 49152;
pub const MULTIPLAYER_PORT: u16 = 49153;

pub struct Server {
    pub tps_limit: f32,
    pub state: ServerState,
    events: EventManager,
    networking: ServerNetworking,
    mods: ServerModManager,
    blocks: ServerBlocks,
    walls: ServerWalls,
    entities: ServerEntities,
    items: ServerItems,
    players: ServerPlayers,
    ui_event_sender: Option<Sender<UiMessageType>>,
    ui_event_receiver: Option<Receiver<UiMessageType>>,
    commands: CommandManager,
}

impl Server {
    #[must_use]
    pub fn new(
        port: u16,
        ui_event_sender: Option<Sender<UiMessageType>>,
        ui_event_receiver: Option<Receiver<UiMessageType>>,
    ) -> Self {
        let blocks = ServerBlocks::new();
        let walls = ServerWalls::new(&mut blocks.get_blocks());
        let mut commands = CommandManager::new();
        commands.add_command(Command {
            call_name: "help".to_owned(),
            name: "Help".to_owned(),
            description: "Shows all commands".to_owned(),
            function: super::commands::help,
        });
        commands.add_command(Command {
            call_name: "stop".to_owned(),
            name: "Stop".to_owned(),
            description: "Stops the server".to_owned(),
            function: super::commands::stop,
        });
        commands.add_command(Command {
            call_name: "give".to_owned(),
            name: "Give".to_owned(),
            description: "Gives an item to a player".to_owned(),
            function: super::commands::give,
        });
        Self {
            tps_limit: 20.0,
            state: ServerState::Nothing,
            events: EventManager::new(),
            networking: ServerNetworking::new(port),
            mods: ServerModManager::new(Vec::new()),
            blocks,
            walls,
            entities: ServerEntities::new(),
            items: ServerItems::new(),
            players: ServerPlayers::new(),
            ui_event_sender,
            ui_event_receiver,
            commands,
        }
    }

    /// Starts the server
    /// # Errors
    /// A lot of server crashes are caused by mods and other stuff, so this function returns a Result
    #[allow(clippy::too_many_lines)]
    pub fn start(
        &mut self,
        status_text: &Mutex<String>,
        mods_serialized: Vec<Vec<u8>>,
        world_path: &Path,
    ) -> Result<()> {
        self.print_to_console("Starting server...", 0);
        let timer = std::time::Instant::now();
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Starting server".to_owned();
        self.state = ServerState::Starting;
        self.send_to_ui(UiMessageType::ServerState(self.state));

        let mut mods = Vec::new();
        for game_mod in mods_serialized {
            // decompress mod with snap
            let game_mod = snap::raw::Decoder::new().decompress_vec(&game_mod)?;
            mods.push(bincode::deserialize(&game_mod)?);
        }
        self.mods = ServerModManager::new(mods);

        // init modules
        self.networking.init();
        self.blocks.init(&mut self.mods.mod_manager)?;
        self.walls.init(&mut self.mods.mod_manager)?;
        self.items.init(&mut self.mods.mod_manager)?;

        let mut generator = WorldGenerator::new();
        generator.init(&mut self.mods.mod_manager)?;

        self.state = ServerState::InitMods;
        self.send_to_ui(UiMessageType::ServerState(self.state));
        self.print_to_console("initializing mods", 0);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) =
            "Initializing mods".to_owned();
        self.mods.init()?;

        if world_path.exists() {
            self.state = ServerState::LoadingWorld;
            self.send_to_ui(UiMessageType::ServerState(self.state));
            self.print_to_console("loading world", 0);
            *status_text.lock().unwrap_or_else(PoisonError::into_inner) =
                "Loading world".to_owned();
            self.load_world(world_path)?;
        } else {
            self.state = ServerState::GeneratingWorld;
            self.send_to_ui(UiMessageType::ServerState(self.state));
            generator.generate(
                (&mut *self.blocks.get_blocks(), &mut self.walls.walls),
                &mut self.mods.mod_manager,
                4400,
                1200,
                423_657,
                status_text,
            )?;
        }

        self.print_to_console(
            &format!("server started in {}ms", timer.elapsed().as_millis()),
            0,
        );
        status_text
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();
        Ok(())
    }

    pub fn run(
        &mut self,
        is_running: &AtomicBool,
        status_text: &Mutex<String>,
        mods_serialized: Vec<Vec<u8>>,
        world_path: &Path,
    ) -> Result<()> {
        let ms_timer = std::time::Instant::now();
        let mut ms_counter = 0;
        let mut last_time = std::time::Instant::now();
        let mut sec_counter = std::time::Instant::now();
        let mut ticks = 1;
        let mut micros = 0;

        self.start(status_text, mods_serialized, world_path)?;

        self.state = ServerState::Running;
        self.send_to_ui(UiMessageType::ServerState(self.state));

        loop {
            let delta_time = last_time.elapsed().as_secs_f32() * 1000.0;
            last_time = std::time::Instant::now();

            self.update(
                delta_time,
                ms_timer,
                &mut ms_counter,
                &mut sec_counter,
                &mut micros,
                last_time,
                &mut ticks,
            )?;

            // sleep
            let sleep_time = 1000.0 / self.tps_limit - last_time.elapsed().as_secs_f32() * 1000.0;
            if sleep_time > 0.0 {
                sleep(Duration::from_secs_f32(sleep_time / 1000.0));
            }
            ticks += 1;

            if !is_running.load(Ordering::Relaxed) || self.state == ServerState::Stopping {
                //state is there so outside events can stop it
                break;
            }
        }

        self.stop(status_text, world_path)?;

        Ok(())
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        ms_timer: std::time::Instant,
        ms_counter: &mut i32,
        sec_counter: &mut std::time::Instant,
        micros: &mut u64,
        last_time: std::time::Instant,
        ticks: &mut u64,
    ) -> Result<()> {
        // update modules
        self.networking.update(&mut self.events)?;
        self.mods.update()?;
        self.blocks.update(&mut self.events, delta_time)?;
        self.walls.update(delta_time, &mut self.events)?;

        // handle events
        self.handle_events()?;

        while *ms_counter < ms_timer.elapsed().as_millis() as i32 {
            self.players.update(
                &mut self.entities.entities,
                &self.blocks.get_blocks(),
                &mut self.events,
                &mut self.items.items,
                &mut self.networking,
            )?;
            self.entities
                .entities
                .update_entities_ms(&self.blocks.get_blocks());
            *ms_counter += 5;
        }

        *micros += last_time.elapsed().as_micros() as u64;
        if sec_counter.elapsed().as_secs() >= 1 {
            self.entities.sync_entities(&mut self.networking)?;
            *sec_counter = std::time::Instant::now();

            self.send_to_ui(UiMessageType::MsptUpdate(
                //send microseconds per tick each second
                *micros / *ticks,
            ));
            *ticks = 0;
            *micros = 0;
        }

        Ok(())
    }

    pub fn stop(&mut self, status_text: &Mutex<String>, world_path: &Path) -> Result<()> {
        // stop modules
        self.networking.stop(&mut self.events)?;
        self.mods.stop()?;
        self.handle_events()?;

        self.state = ServerState::Stopping;
        self.send_to_ui(UiMessageType::ServerState(self.state));
        self.print_to_console("saving world", 0);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Saving world".to_owned();

        self.save_world(world_path)?;

        self.print_to_console("stopping server", 0);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Stopping server".to_owned();

        self.state = ServerState::Stopped;
        self.send_to_ui(UiMessageType::ServerState(self.state));
        status_text
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();
        self.print_to_console("server stopped.", 0);

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(receiver) = &self.ui_event_receiver {
            self.commands.execute_commands(
                receiver,
                &self.ui_event_sender,
                &mut self.state,
                &mut self.players,
                &mut self.items,
                &mut self.entities,
                &mut self.events,
            );
        }

        while let Some(event) = self.events.pop_event() {
            if let Some(disconnect) = event.downcast::<DisconnectEvent>() {
                self.send_to_ui(UiMessageType::PlayerEvent(PlayerEventType::Leave(
                    disconnect.conn.address.addr(),
                )));
            }
            if let Some(connect) = event.downcast::<NewConnectionEvent>() {
                self.send_to_ui(UiMessageType::PlayerEvent(PlayerEventType::Join((
                    connect.name.clone(),
                    connect.conn.address.addr(),
                ))));
            }
            if let Some(event) = event.downcast::<UiMessageType>() {
                self.send_to_ui(event.clone());
            }

            self.commands.on_event(
                &event,
                &mut self.state,
                &mut self.players,
                &mut self.items,
                &mut self.entities,
                &mut self.events,
                &self.ui_event_sender,
            );

            self.mods.on_event(&event, &mut self.networking)?;
            self.blocks.on_event(
                &event,
                &mut self.events,
                &mut self.networking,
                &mut self.entities.entities,
                &mut self.players,
                &mut self.items.items,
                &mut self.mods.mod_manager,
            )?;
            self.walls.on_event(&event, &mut self.networking)?;
            self.items.on_event(
                &event,
                &mut self.entities.entities,
                &mut self.events,
                &mut self.networking,
            )?;
            self.players.on_event(
                &event,
                &mut self.entities.entities,
                &self.blocks.get_blocks(),
                &mut self.networking,
                &mut self.events,
            )?;
            ServerEntities::on_event(&event, &mut self.networking)?;
            self.networking.on_event(&event, &mut self.events)?;
            server_chat_on_event(&event, &mut self.networking)?;
        }

        Ok(())
    }

    fn load_world(&mut self, world_path: &Path) -> Result<()> {
        // load world file into Vec<u8>
        let world_file = std::fs::read(world_path)?;
        // decode world file as HashMap<String, Vec<u8>>
        let world: HashMap<String, Vec<u8>> = bincode::deserialize(&world_file)?;

        self.blocks
            .get_blocks()
            .deserialize(world.get("blocks").unwrap_or(&Vec::new()))?;
        self.walls
            .walls
            .deserialize(world.get("walls").unwrap_or(&Vec::new()))?;
        self.players
            .deserialize(world.get("players").unwrap_or(&Vec::new()))?;
        Ok(())
    }

    fn save_world(&self, world_path: &Path) -> Result<()> {
        let mut world = HashMap::new();
        world.insert("blocks".to_owned(), self.blocks.get_blocks().serialize()?);
        world.insert("walls".to_owned(), self.walls.walls.serialize()?);
        world.insert("players".to_owned(), self.players.serialize()?);

        let world_file = bincode::serialize(&world)?;
        if !world_path.exists() {
            std::fs::create_dir_all(
                world_path
                    .parent()
                    .ok_or_else(|| anyhow!("could not get parent folder"))?,
            )?;
        }
        std::fs::write(world_path, world_file)?;
        Ok(())
    }

    fn send_to_ui(&self, data: UiMessageType) {
        send_to_ui(data, &self.ui_event_sender);
    }

    //prints to the terminal the server was started in and sends it to the ui
    fn print_to_console(&self, text: &str, warn_level: u8) {
        let formatted_text;
        if warn_level == 0 {
            formatted_text = format!("[INFO] {text}");
        } else if warn_level == 1 {
            formatted_text = format!("[WARNING] {text}");
        } else {
            formatted_text = format!("[ERROR] {text}");
        }
        println!("{formatted_text}");
        let text_with_type = match warn_level {
            0 => ConsoleMessageType::Info(formatted_text),
            1 => ConsoleMessageType::Warning(formatted_text),
            _ => ConsoleMessageType::Error(formatted_text),
        };
        self.send_to_ui(UiMessageType::SrvToUiConsoleMessage(text_with_type));
    }
}

//sends any data to the ui if the server was started without nogui flag
pub fn send_to_ui(data: UiMessageType, ui_event_sender: &Option<Sender<UiMessageType>>) {
    if let Some(sender) = ui_event_sender {
        let result = sender.send(data);

        if let Err(_e) = result {
            println!("error sending data to ui");
        }
    }
}
