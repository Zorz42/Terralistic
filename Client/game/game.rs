use graphics::GraphicsContext;
use graphics as gfx;

pub struct Game {

}

impl Game {
    pub fn new() -> Game {
        Game {}
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

            graphics.renderer.update_window();
        }
    }
}