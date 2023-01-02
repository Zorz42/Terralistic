use graphics::GraphicsContext;
use graphics as gfx;
use shared::mod_manager::GameMod;
use events::EventManager;
use crate::game::background::Background;
use crate::game::blocks::ClientBlocks;
use crate::game::camera::Camera;
use crate::game::mod_manager::ClientModManager;
use crate::game::networking::ClientNetworking;

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

    pub fn run(&mut self, graphics: &mut GraphicsContext) {
        // load base game mod
        let base_mod = GameMod::from_bytes(include_bytes!("../../BaseGame/BaseGame.mod").to_vec());
        self.mods.mod_manager.add_mod(base_mod);

        self.networking.init(&mut self.events);
        self.flush_events();
        self.background.init();
        self.blocks.init(&mut self.mods.mod_manager);

        self.mods.init();

        self.blocks.load_resources(&mut self.mods.mod_manager);

        let ms_timer = gfx::Timer::new();
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

            while ms_counter < ms_timer.get_time() as i32 {
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
            self.networking.on_event(event);
        }
    }
}