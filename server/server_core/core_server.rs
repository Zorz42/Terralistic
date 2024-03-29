use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Mutex, PoisonError};
use std::thread::sleep;
use std::time::Duration;

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
    ui_event_receiver: Option<Receiver<UiMessageType>>,
    commands: CommandManager,
}

impl Server {
    #[must_use]
    pub fn new(port: u16, ui_event_receiver: Option<Receiver<UiMessageType>>, ui_event_sender: Option<Sender<UiMessageType>>) -> Self {
        send_to_ui(UiMessageType::ServerState(ServerState::Nothing), ui_event_sender); //this is useless but sets the ui event sender
        let blocks = ServerBlocks::new();
        let walls = ServerWalls::new(&mut blocks.get_blocks());
        let mut commands = CommandManager::new();
        commands.add_command(Command {
            call_name: "help".to_owned(),
            name: "Help".to_owned(),
            description: "Shows all commands".to_owned(),
            function: super::commands::help_command,
        });
        commands.add_command(Command {
            call_name: "stop".to_owned(),
            name: "Stop".to_owned(),
            description: "Stops the server".to_owned(),
            function: super::commands::stop_command,
        });
        commands.add_command(Command {
            call_name: "give".to_owned(),
            name: "Give".to_owned(),
            description: "Gives an item to a player".to_owned(),
            function: super::commands::give_command,
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
            ui_event_receiver,
            commands,
        }
    }

    /// Starts the server - manual way. It only inits the server but doesn't run a loop
    #[allow(clippy::too_many_lines)]
    pub fn start(&mut self, status_text: &Mutex<String>, mods_serialized: Vec<Vec<u8>>, world_path: &Path) -> Result<()> {
        print_to_console("Starting server...", 0);
        let timer = std::time::Instant::now();
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Starting server".to_owned();
        self.state = ServerState::Starting;
        send_to_ui(UiMessageType::ServerState(self.state), None);

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
        send_to_ui(UiMessageType::ServerState(self.state), None);
        print_to_console("initializing mods", 0);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Initializing mods".to_owned();
        self.mods.init()?;

        if world_path.exists() {
            self.state = ServerState::LoadingWorld;
            send_to_ui(UiMessageType::ServerState(self.state), None);
            print_to_console("loading world", 0);
            *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Loading world".to_owned();
            self.load_world(world_path)?;
        } else {
            self.state = ServerState::GeneratingWorld;
            send_to_ui(UiMessageType::ServerState(self.state), None);
            generator.generate(
                (&mut *self.blocks.get_blocks(), &mut self.walls.get_walls()),
                &mut self.mods.mod_manager,
                4400,
                1200,
                423_657,
                status_text,
            )?;
        }

        self.state = ServerState::Running;
        send_to_ui(UiMessageType::ServerState(self.state), None);

        print_to_console(&format!("server started in {}ms", timer.elapsed().as_millis()), 0);
        status_text.lock().unwrap_or_else(PoisonError::into_inner).clear();
        Ok(())
    }

    ///Runs the server - automated way. It starts (inits) the server, runs it until it has top be stopped, then stops it and returns
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

            if !is_running.load(Ordering::Relaxed) || self.state == ServerState::Stopping {
                //state is there so outside events can stop it
                break;
            }
        }

        self.stop(status_text, world_path)?;

        Ok(())
    }

    ///Updates the server - manual way. It updates the server once and returns
    pub fn update(&mut self) -> Result<()> {
        //there's no point in outside functions knowing about the counters. Letting outside functions manage these variables could lead to bugs
        static mut MS_COUNTER: i32 = 0;
        static mut SECONDS_COUNTER: i32 = 0;
        static mut MS_TIMER: Option<std::time::Instant> = None;
        static mut LAST_TIME: Option<std::time::Instant> = None;

        let Some((ms_timer, last_time)) =
            //Safety: safe, we're only updating and using the count without any multithreading
            (unsafe { get_timers_from_static(&mut MS_TIMER, &mut LAST_TIME) })
        else {
            return Ok(()); //we return early this time
        };

        let delta_time = last_time.elapsed().as_secs_f32() * 1000.0;

        // update modules
        self.networking.update(&mut self.events)?;
        self.mods.update()?;
        self.blocks.update(&mut self.events, delta_time)?;
        self.walls.update(delta_time, &mut self.events)?;

        // handle events
        self.handle_events()?;

        //Safety: safe, we're only updating and using the count without any multithreading
        while unsafe { MS_COUNTER < ms_timer.elapsed().as_millis() as i32 } {
            self.players.update(
                &mut self.entities.entities,
                &self.blocks.get_blocks(),
                &mut self.events,
                &mut self.items.get_items(),
                &mut self.networking,
            )?;
            self.entities.entities.update_entities_ms(&self.blocks.get_blocks(), &mut self.events)?;
            //Safety: safe, we're only updating and using the count without any multithreading
            unsafe {
                MS_COUNTER += 5;
            }
        }

        //Safety: safe, we're only updating and using the count without any multithreading
        unsafe {
            if SECONDS_COUNTER < MS_COUNTER / 1000 {
                self.entities.sync_entities(&mut self.networking)?;
                SECONDS_COUNTER = MS_COUNTER / 1000;
            }
        }

        Ok(())
    }

    ///Stops the server - manual way. It stops the server and returns
    pub fn stop(&mut self, status_text: &Mutex<String>, world_path: &Path) -> Result<()> {
        if self.state == ServerState::Stopped {
            //so we don't stop it twice
            return Ok(());
        }
        // stop modules
        self.networking.stop(&mut self.events)?;
        self.mods.stop()?;
        self.handle_events()?;

        self.state = ServerState::Stopping;
        send_to_ui(UiMessageType::ServerState(self.state), None);
        print_to_console("saving world", 0);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Saving world".to_owned();

        self.save_world(world_path)?;

        print_to_console("stopping server", 0);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Stopping server".to_owned();

        self.state = ServerState::Stopped;
        send_to_ui(UiMessageType::ServerState(self.state), None);
        status_text.lock().unwrap_or_else(PoisonError::into_inner).clear();
        print_to_console("server stopped.", 0);

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(receiver) = &self.ui_event_receiver {
            self.commands
                .execute_commands(receiver, &mut self.state, &mut self.players, &mut self.items, &mut self.entities, &mut self.events);
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

            self.commands
                .on_event(&event, &mut self.state, &mut self.players, &mut self.items, &mut self.entities, &mut self.events, &mut self.networking)?;

            self.mods.on_event(&event, &mut self.networking)?;
            self.blocks.on_event(
                &event,
                &mut self.events,
                &mut self.networking,
                &self.entities.entities,
                &self.players,
                &self.items.get_items(),
                &mut self.mods.mod_manager,
            )?;
            self.walls.on_event(&event, &mut self.networking)?;
            self.items.on_event(&event, &mut self.entities.entities, &mut self.events, &mut self.networking)?;
            self.players.on_event(
                &event,
                &mut self.entities.entities,
                &mut self.blocks.get_blocks(),
                &mut self.networking,
                &mut self.events,
                &mut self.items.get_items(),
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

        let world_file = bincode::serialize(&world)?;
        if !world_path.exists() {
            std::fs::create_dir_all(world_path.parent().ok_or_else(|| anyhow!("could not get parent folder"))?)?;
        }
        std::fs::write(world_path, world_file)?;
        Ok(())
    }
}

unsafe fn get_timers_from_static(ms_timer_static: &mut Option<std::time::Instant>, last_time_static: &mut Option<std::time::Instant>) -> Option<(std::time::Instant, std::time::Instant)> {
    let ms_timer = if let Some(timer) = ms_timer_static {
        *timer
    } else {
        *ms_timer_static = Some(std::time::Instant::now());
        *last_time_static = *ms_timer_static;
        return None; //we skip this time
    };

    let last_time = if let Some(instant) = last_time_static {
        *instant
    } else {
        *last_time_static = Some(std::time::Instant::now());
        return None; //we skip this time
    };

    *last_time_static = Some(std::time::Instant::now());

    Some((ms_timer, last_time))
}

//sends any data to the ui if the server was started without nogui flag
pub fn send_to_ui(data: UiMessageType, ui_event_sender: Option<Sender<UiMessageType>>) {
    static mut UI_EVENT_SENDER: Option<Sender<UiMessageType>> = None;

    //Safety: safe becuase the server is single threaded
    unsafe {
        if UI_EVENT_SENDER.is_none() {
            UI_EVENT_SENDER = ui_event_sender;
        }

        if let Some(sender) = &UI_EVENT_SENDER {
            let result = sender.send(data);

            if let Err(_e) = result {
                println!("error sending data to ui");
            }
        }
    }
}

//prints to the terminal the server was started in and sends it to the ui
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

//this function formats the string to add the timestamp
fn format_timestamp(message: &String) -> String {
    let timestamp = chrono::Local::now().naive_local().timestamp();
    let timestamp = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
    format!("[{}] {}", timestamp.map_or_else(|| "???".to_owned(), |time| time.format("%m-%d %H:%M:%S").to_string(),), message)
}
