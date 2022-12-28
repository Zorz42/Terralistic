use graphics::GraphicsContext;
use graphics as gfx;

/**
Camera is a struct that handles the camera position.
 */

pub struct Camera {
    position_x: f32,
    position_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position_x = x;
        self.position_y = y;
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.position_x, self.position_y)
    }

    pub fn update_ms(&mut self, graphics: &mut GraphicsContext) {
        if graphics.renderer.get_key_state(gfx::Key::W) {
            self.position_y -= 0.5;
        }

        if graphics.renderer.get_key_state(gfx::Key::S) {
            self.position_y += 0.5;
        }

        if graphics.renderer.get_key_state(gfx::Key::A) {
            self.position_x -= 0.5;
        }

        if graphics.renderer.get_key_state(gfx::Key::D) {
            self.position_x += 0.5;
        }
    }

    /**
    This function gets the position of the top left corner of the screen in world coordinates.
     */
    pub fn get_top_left(&self, graphics: &mut GraphicsContext) -> (f32, f32) {
        let width= graphics.renderer.get_window_width();
        let height = graphics.renderer.get_window_height();
        (self.position_x - width as f32 / 2.0, self.position_y - height as f32 / 2.0)
    }

    /**
    This function gets the position of the bottom right corner of the screen in world coordinates.
     */
    pub fn get_bottom_right(&self, graphics: &mut GraphicsContext) -> (f32, f32) {
        let width= graphics.renderer.get_window_width();
        let height = graphics.renderer.get_window_height();
        (self.position_x + width as f32 / 2.0, self.position_y + height as f32 / 2.0)
    }
}
