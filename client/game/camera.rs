use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::BaseUiElement;
use crate::shared::blocks::RENDER_BLOCK_WIDTH;

/// Camera is a struct that handles the camera position.

pub struct Camera {
    target_position_x: f32,
    target_position_y: f32,
    position_x: f32,
    position_y: f32,
    detached: bool,
    detached_text: gfx::Sprite,
}

impl Camera {
    pub const fn new() -> Self {
        Self {
            target_position_x: 0.0,
            target_position_y: 0.0,
            position_x: 0.0,
            position_y: 0.0,
            detached: false,
            detached_text: gfx::Sprite::new(),
        }
    }

    pub fn load_resources(&mut self, graphics: &gfx::GraphicsContext) {
        self.detached_text
            .set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Camera is detached", None)));
        self.detached_text.orientation = gfx::BOTTOM;
        self.detached_text.pos = gfx::FloatPos(0.0, -gfx::SPACING);
        self.detached_text.scale = 3.0;
        self.detached_text.color = gfx::Color::new(255, 0, 0, 255);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        if !self.detached {
            self.target_position_x = x;
            self.target_position_y = y;
        }
    }

    pub const fn get_position(&self) -> gfx::FloatPos {
        gfx::FloatPos(self.position_x, self.position_y)
    }

    pub fn update_ms(&mut self, graphics: &gfx::GraphicsContext) {
        self.position_x += (self.target_position_x - self.position_x) * 0.03;
        self.position_y += (self.target_position_y - self.position_y) * 0.03;

        if self.detached {
            if graphics.get_key_state(gfx::Key::W) {
                self.target_position_y -= 0.5;
            }

            if graphics.get_key_state(gfx::Key::S) {
                self.target_position_y += 0.5;
            }

            if graphics.get_key_state(gfx::Key::A) {
                self.target_position_x -= 0.5;
            }

            if graphics.get_key_state(gfx::Key::D) {
                self.target_position_x += 0.5;
            }
        }
    }

    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext) {
        if self.detached {
            self.detached_text.render(graphics, &gfx::Container::default(graphics));
        }
    }

    /// This function gets the position of the top left corner of the screen in world coordinates.
    pub fn get_top_left(&self, graphics: &gfx::GraphicsContext) -> (f32, f32) {
        let width = graphics.get_window_size().0 / RENDER_BLOCK_WIDTH;
        let height = graphics.get_window_size().1 / RENDER_BLOCK_WIDTH;
        (self.position_x - width / 2.0, self.position_y - height / 2.0)
    }

    /// This function gets the position of the bottom right corner of the screen in world coordinates.
    pub fn get_bottom_right(&self, graphics: &gfx::GraphicsContext) -> (f32, f32) {
        let width = graphics.get_window_size().0 / RENDER_BLOCK_WIDTH;
        let height = graphics.get_window_size().1 / RENDER_BLOCK_WIDTH;
        (self.position_x + width / 2.0, self.position_y + height / 2.0)
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(event) = event.downcast::<gfx::Event>() {
            if matches!(event, gfx::Event::KeyPress(gfx::Key::C, false)) {
                self.detached = !self.detached;
            }
        }
    }

    pub const fn is_detached(&self) -> bool {
        self.detached
    }
}
