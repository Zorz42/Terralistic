use graphics::GraphicsContext;
use graphics as gfx;
use shared::blocks::BLOCK_WIDTH;
use shared::mod_manager::GameMod;
use shared_mut::SharedMut;
use events::EventManager;
use crate::game::background::Background;
use crate::game::blocks::ClientBlocks;
use crate::game::camera::Camera;
use crate::game::mod_manager::ClientModManager;
use crate::game::networking::ClientNetworking;
use crate::menus::{BackgroundRect, run_loading_screen};

pub struct Game {
    events: EventManager,
    networking: ClientNetworking,
    mods: ClientModManager,
    camera: Camera,
    background: Background,
    blocks: ClientBlocks,
}

impl Game {
    pub fn new(server_port: u16, server_address: String) -> Self {
        Self {
            events: EventManager::new(),
            networking: ClientNetworking::new(server_port, server_address),
            mods: ClientModManager::new(),
            camera: Camera::new(),
            background: Background::new(),
            blocks: ClientBlocks::new(),
        }
    }

    pub fn run(&mut self, graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect) {
        // load base game mod
        let timer = std::time::Instant::now();

        let mut events = EventManager::new();
        self.networking.init(&mut events);

        let loading_text = SharedMut::new(String::from("Loading"));
        let loading_text2 = loading_text.clone();
        let init_thread = std::thread::spawn(move || {
            *loading_text2.borrow() = "Loading mods".to_string();
            let mut mods = ClientModManager::new();
            let mut blocks = ClientBlocks::new();

            while let Some(event) = events.pop_event() {
                blocks.on_event(&event);
            }

            let base_mod = GameMod::from_bytes(include_bytes!("../../BaseGame/BaseGame.mod").to_vec());
            mods.mod_manager.add_mod(base_mod);

            blocks.init(&mut mods.mod_manager);

            *loading_text2.borrow() = "Initializing mods".to_string();
            mods.init();

            loading_text2.borrow().clear();
            (mods, blocks)
        });

        run_loading_screen(graphics, menu_back, loading_text);

        let result = init_thread.join().unwrap();
        self.mods = result.0;
        self.blocks = result.1;

        self.camera.set_position(self.blocks.blocks.get_width() as f32 / 2.0 * BLOCK_WIDTH as f32, self.blocks.blocks.get_height() as f32 / 3.0 * BLOCK_WIDTH as f32);

        self.background.init();

        self.blocks.load_resources(&mut self.mods.mod_manager);

        // print the time it took to initialize
        println!("Game joined in {}ms", timer.elapsed().as_millis());

        let ms_timer = std::time::Instant::now();
        let mut ms_counter = 0;
        'main_loop: while graphics.renderer.is_window_open() {
            while let Some(event) = graphics.renderer.get_event() {
                match event {
                    gfx::Event::KeyRelease(key) => {
                        if key == gfx::Key::Escape {
                            break 'main_loop;
                        }
                    }
                    _ => {}
                }
            }

            self.networking.update(&mut self.events);
            self.mods.update();

            while ms_counter < ms_timer.elapsed().as_millis() as i32 {
                self.camera.update_ms(graphics);
                ms_counter += 1;
            }

            self.background.render(graphics, &self.camera);
            self.blocks.render(graphics, &self.camera);

            self.flush_events();

            graphics.renderer.update_window();
        }

        self.networking.stop();
        self.mods.stop();
    }

    pub fn flush_events(&mut self) {
        while let Some(event) = self.events.pop_event() {
            self.blocks.on_event(&event);
            //self.networking.on_event(event);
        }
    }
}