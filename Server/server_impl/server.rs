mod networking;
mod mod_manager;
mod blocks;
mod walls;
mod world_generator;

use crate::blocks::ServerBlocks;
use crate::mod_manager::ServerModManager;
use crate::networking::ServerNetworking;
use crate::walls::ServerWalls;
use crate::world_generator::WorldGenerator;
use events::EventManager;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

pub struct Server {
    tps_limit: f32,
    events: EventManager,
    networking: ServerNetworking,
    mods: ServerModManager,
    blocks: ServerBlocks,
    walls: ServerWalls,
}

impl Server {
    pub fn new(port: u16) -> Self {
        let mut blocks = ServerBlocks::new();
        let walls = ServerWalls::new(&mut blocks.blocks);
        Self {
            tps_limit: 20.0,
            events: EventManager::new(),
            networking: ServerNetworking::new(port),
            mods: ServerModManager::new(),
            blocks,
            walls,
        }
    }

    pub fn start(
        &mut self, is_running: &Mutex<bool>, status_text: &Mutex<String>, mods: Vec<Vec<u8>>,
        world_path: &Path,
    ) {
        println!("Starting server...");
        let timer = std::time::Instant::now();
        *status_text.lock().unwrap() = "Starting server".to_string();

        for mod_ in mods {
            // decompress mod with snap
            let mod_ = snap::raw::Decoder::new().decompress_vec(&mod_).unwrap();
            self.mods
                .mod_manager
                .add_mod(bincode::deserialize(&mod_).unwrap());
        }

        // init modules
        self.networking.init();
        self.blocks.init(&mut self.mods.mod_manager);
        self.walls.init(&mut self.mods.mod_manager);

        let mut generator = WorldGenerator::new();
        generator.init(&mut self.mods.mod_manager);

        *status_text.lock().unwrap() = "Initializing mods".to_string();
        self.mods.init();

        if world_path.exists() {
            *status_text.lock().unwrap() = "Loading world".to_string();
            self.load_world(world_path);
        } else {
            generator.generate(
                &mut self.blocks.blocks,
                &mut self.walls.walls,
                &mut self.mods.mod_manager,
                4400,
                1200,
                423657,
                status_text,
            );
        }

        // start server loop
        println!("Server started in {}ms", timer.elapsed().as_millis());
        status_text.lock().unwrap().clear();
        let mut last_time = std::time::Instant::now();
        loop {
            let delta_time = last_time.elapsed().as_secs_f32() * 1000.0;
            last_time = std::time::Instant::now();

            // update modules
            self.networking.update(&mut self.events);
            self.mods.update();
            self.blocks.update(&mut self.events, delta_time);
            self.walls.update(delta_time, &mut self.events);

            // handle events
            while let Some(event) = self.events.pop_event() {
                self.mods.on_event(&event, &mut self.networking);
                self.blocks.on_event(&event, &mut self.events, &mut self.networking);
                self.walls.on_event(&event, &mut self.networking);
                self.networking.on_event(&event);
            }

            if !*is_running.lock().unwrap() {
                break;
            }

            // sleep
            let sleep_time = 1000.0 / self.tps_limit
                - (std::time::Instant::now() - last_time).as_secs_f32() * 1000.0;
            if sleep_time > 0.0 {
                std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time / 1000.0));
            }
        }

        *status_text.lock().unwrap() = "Saving world".to_string();
        self.save_world(world_path);

        *status_text.lock().unwrap() = "Stopping server".to_string();
        // stop modules
        self.networking.stop();
        self.mods.stop();

        status_text.lock().unwrap().clear();
        println!("Server stopped.");
    }

    fn load_world(&mut self, world_path: &Path) {
        // load world file into Vec<u8>
        let world_file = std::fs::read(world_path).unwrap();
        // decode world file as HashMap<String, Vec<u8>>
        let world: HashMap<String, Vec<u8>> = bincode::deserialize(&world_file).unwrap();

        self.blocks
            .blocks
            .deserialize(world.get("blocks").unwrap())
            .unwrap();
        self.walls
            .walls
            .deserialize(world.get("walls").unwrap())
            .unwrap();
    }

    fn save_world(&self, world_path: &Path) {
        let mut world = HashMap::new();
        world.insert(
            "blocks".to_string(),
            self.blocks.blocks.serialize().unwrap(),
        );
        world.insert("walls".to_string(), self.walls.walls.serialize().unwrap());

        let world_file = bincode::serialize(&world).unwrap();
        if !world_path.exists() {
            std::fs::create_dir_all(world_path.parent().unwrap()).unwrap();
        }
        std::fs::write(world_path, world_file).unwrap();
    }
}
