use graphics::GraphicsContext;
use graphics as gfx;
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
        self.networking.init();
        self.mods.init();
        self.background.init(graphics);
        self.blocks.init(graphics);

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
            self.blocks.render(graphics);

            while let Some(event) = self.events.pop_event() {
                self.networking.on_event(event);
            }

            graphics.renderer.update_window();
        }

        self.networking.stop();
        self.mods.stop();
    }
}