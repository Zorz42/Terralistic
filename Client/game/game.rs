use graphics::GraphicsContext;
use graphics as gfx;
use events::EventManager;
use crate::game::networking::ClientNetworking;

pub struct Game {
    events: EventManager,
    networking: ClientNetworking,
}

impl Game {
    pub fn new(server_port: u16, server_address: String) -> Self {
        Self {
            events: EventManager::new(),
            networking: ClientNetworking::new(server_port, server_address),
        }
    }

    pub fn run(&mut self, graphics: &mut GraphicsContext) {
        self.networking.init();

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

            gfx::Rect::new(0, 0, graphics.renderer.get_window_width() as i32, graphics.renderer.get_window_height() as i32).render(graphics, gfx::GREY);

            self.networking.update(&mut self.events);

            while let Some(event) = self.events.pop_event() {
                self.networking.on_event(event);
            }

            graphics.renderer.update_window();
        }

        self.networking.stop();
    }
}