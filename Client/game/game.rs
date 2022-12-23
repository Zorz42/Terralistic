use graphics::GraphicsContext;
use graphics as gfx;
use crate::game::networking::ClientNetworking;

pub struct Game {
    networking: ClientNetworking,
}

impl Game {
    pub fn new(server_port: u16, server_address: String) -> Game {
        Game {
            networking: ClientNetworking::new(server_port, server_address),
        }
    }

    pub fn run(&mut self, graphics: &mut GraphicsContext) {
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

            graphics.renderer.update_window();
        }
    }
}