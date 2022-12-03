use super::background_rect::{Background, BackgroundRect};
use graphics as gfx;

/*
MenuBack is a struct that contains the background rectangle for
the most main menus. It implements the BackgroundRect trait. It
draws the background.opa image scaled to the window's height and
scrolled to the left.
*/
pub struct MenuBack {
    temp_container: gfx::Container,
    background: gfx::Texture,
    background_timer: gfx::Timer,
}

impl MenuBack {
    /*
    Creates a new MenuBack.
    */
    pub fn new() -> Self {
        MenuBack {
            temp_container: gfx::Container::new(0, 0, 0, 0, gfx::CENTER, None),
            background: gfx::Texture::load_from_surface(&gfx::Surface::deserialize(include_bytes!("../../Build/Resources/background.opa").to_vec())),
            background_timer: gfx::Timer::new(),
        }
    }
}

impl Background for MenuBack {
    /*
    Renders the background.
    */
    fn render_back(&self, graphics: &mut gfx::GraphicsContext) {
        let scale = graphics.renderer.get_window_height() as f32 / self.background.get_texture_height() as f32;
        let texture_width_scaled = self.background.get_texture_width() as f32 * scale;
        let pos = ((self.background_timer.get_time() * scale as f64 / 150.0) as u64 % texture_width_scaled as u64) as i32;

        for i in -1..graphics.renderer.get_window_width() as i32 / (self.background.get_texture_width() as f32 * scale) as i32 + 2 {
            self.background.render(&graphics.renderer, scale, pos + (i as f32 * texture_width_scaled) as i32, 0, gfx::Rect::new(0, 0, self.background.get_texture_width(), self.background.get_texture_height()), false, gfx::Color{r: 255, g: 255, b: 255, a: 255});
        }
    }
}

impl BackgroundRect for MenuBack {
    /*
    Sets the width of the background rectangle.
    */
    fn set_back_rect_width(&mut self, width: i32) {

    }

    /*
    Gets the width of the background rectangle.
    */
    fn get_back_rect_width(&self) -> i32 {
        0
    }

    /*
    Gets the background rectangle's container.
    */
    fn get_back_rect_container(&self) -> &gfx::Container {
        &self.temp_container
    }
}