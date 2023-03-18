use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use core::time::Duration;
use core::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, PoisonError};
use std::thread::sleep;

use anyhow::{anyhow, Result};
use crate::libraries::events::EventManager;
use crate::server::server_core::entities::ServerEntities;
use crate::server::server_core::items::ServerItems;
use crate::server::server_core::players::ServerPlayers;

use super::blocks::ServerBlocks;
use super::mod_manager::ServerModManager;
use super::networking::ServerNetworking;
use super::walls::ServerWalls;
use super::world_generator::WorldGenerator;

pub const SINGLEPLAYER_PORT: u16 = 49152;
pub const MULTIPLAYER_PORT: u16 = 49153;

#[derive(Copy, Clone, serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Eq)]
pub enum ServerState {
    Nothing,
    Starting,
    InitMods,
    LoadingWorld,
    Running,
    Stopping,
    Stopped }

impl core::fmt::Display for ServerState {//this only stays for now since i debug things with it. remove later
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub struct Server {
    tps_limit: f32,
    state: ServerState,
    events: EventManager,
    networking: ServerNetworking,
    mods: ServerModManager,
    blocks: ServerBlocks,
    walls: ServerWalls,
    entities: ServerEntities,
    items: ServerItems,
    players: ServerPlayers,
    ui_event_sender: Option<Sender<Vec<u8>>>,
}

impl Server {
    #[must_use]
    pub fn new(port: u16, ui_event_sender: Option<Sender<Vec<u8>>>) -> Self {
        let mut blocks = ServerBlocks::new();
        let walls = ServerWalls::new(&mut blocks.blocks);
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
        }
    }

    /// Starts the server
    /// # Errors
    /// A lot of server crashes are caused by mods and other stuff, so this function returns a Result
    #[allow(clippy::too_many_lines)]
    pub fn start(
        &mut self,
        is_running: &AtomicBool,
        status_text: &Mutex<String>,
        mods_serialized: Vec<Vec<u8>>,
        world_path: &Path,
    ) -> Result<()> {
        println!("Starting server...");
        let timer = std::time::Instant::now();
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Starting server".to_owned();
        self.state = ServerState::Starting;
        self.send_to_ui(&self.state);

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
        self.send_to_ui(&self.state);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) =
            "Initializing mods".to_owned();
        self.mods.init()?;

        self.state = ServerState::LoadingWorld;
        self.send_to_ui(&self.state);
        if world_path.exists() {
            *status_text.lock().unwrap_or_else(PoisonError::into_inner) =
                "Loading world".to_owned();
            self.load_world(world_path)?;
        } else {
            generator.generate(
                (&mut self.blocks.blocks, &mut self.walls.walls),
                &mut self.mods.mod_manager,
                4400,
                1200,
                423_657,
                status_text,
            )?;
        }

        self.state = ServerState::Running;
        self.send_to_ui(&self.state);
        // start server loop
        println!("server started in {}ms", timer.elapsed().as_millis());
        status_text
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();
        let ms_timer = std::time::Instant::now();
        let mut ms_counter = 0;
        let mut last_time = std::time::Instant::now();
        let mut sec_counter = std::time::Instant::now();
        loop {
            let delta_time = last_time.elapsed().as_secs_f32() * 1000.0;
            last_time = std::time::Instant::now();

            // update modules
            self.networking.update(&mut self.events)?;
            self.mods.update()?;
            self.blocks.update(&mut self.events, delta_time)?;
            self.walls.update(delta_time, &mut self.events)?;

            // handle events
            while let Some(event) = self.events.pop_event() {
                self.mods.on_event(&event, &mut self.networking)?;
                self.blocks
                    .on_event(&event, &mut self.events, &mut self.networking)?;
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
                    &self.blocks.blocks,
                    &mut self.networking,
                )?;
                self.networking.on_event(&event, &mut self.events)?;
            }

            while ms_counter < ms_timer.elapsed().as_millis() as i32 {
                ServerPlayers::update(&mut self.entities.entities, &self.blocks.blocks);
                self.entities
                    .entities
                    .update_entities_ms(&self.blocks.blocks);
                ms_counter += 5;
            }

            if sec_counter.elapsed().as_secs() >= 1 {
                self.entities.sync_entities(&mut self.networking)?;
                sec_counter = std::time::Instant::now();
            }
            if !is_running.load(Ordering::Relaxed) {
                break;
            }
            // sleep
            let sleep_time = 1000.0 / self.tps_limit - last_time.elapsed().as_secs_f32() * 1000.0;
            if sleep_time > 0.0 {
                sleep(Duration::from_secs_f32(sleep_time / 1000.0));
            }
        }

        self.state = ServerState::Stopping;
        self.send_to_ui(&self.state);
        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Saving world".to_owned();
        self.save_world(world_path)?;

        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Stopping server".to_owned();
        // stop modules
        self.networking.stop()?;
        self.mods.stop()?;

        self.state = ServerState::Stopped;
        self.send_to_ui(&self.state);
        status_text
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();
        println!("server stopped.");
        Ok(())
    }

    fn load_world(&mut self, world_path: &Path) -> Result<()> {
        // load world file into Vec<u8>
        let world_file = std::fs::read(world_path)?;
        // decode world file as HashMap<String, Vec<u8>>
        let world: HashMap<String, Vec<u8>> = bincode::deserialize(&world_file)?;

        self.blocks
            .blocks
            .deserialize(world.get("blocks").unwrap_or(&Vec::new()))?;
        self.walls
            .walls
            .deserialize(world.get("walls").unwrap_or(&Vec::new()))?;
        Ok(())
    }

    fn save_world(&self, world_path: &Path) -> Result<()> {
        let mut world = HashMap::new();
        world.insert("blocks".to_owned(), self.blocks.blocks.serialize()?);
        world.insert("walls".to_owned(), self.walls.walls.serialize()?);

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

    fn send_to_ui<T: Any + Send + serde::Serialize>(&self, data: &T) {
        if let Some(sender) = &self.ui_event_sender {
            let data = &bincode::serialize(&data).unwrap_or_default();
            let data = snap::raw::Encoder::new().compress_vec(data).unwrap_or_default();
            let result = sender.send(data);

            if let Err(_e) = result {
                println!("could not send to server ui");
            }
        }
    }
}
