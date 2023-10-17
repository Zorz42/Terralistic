use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::shared::blocks::RENDER_BLOCK_WIDTH;

/// Camera is a struct that handles the camera position.

pub struct Camera {
    target_position_x: f32,
    target_position_y: f32,
    position_x: f32,
    position_y: f32,
    detached_camera: bool,
    detached_camera_text: gfx::Sprite,
}

impl Camera {
    pub const fn new() -> Self {
        Self {
            target_position_x: 0.0,
            target_position_y: 0.0,
            position_x: 0.0,
            position_y: 0.0,
            detached_camera: false,
            detached_camera_text: gfx::Sprite::new(),
        }
    }

    pub fn load_resources(&mut self, graphics: &gfx::GraphicsContext) {
        self.detached_camera_text.texture = gfx::Texture::load_from_surface(
            &graphics
                .font
                .create_text_surface("Camera is detached", None),
        );
        self.detached_camera_text.orientation = gfx::BOTTOM;
        self.detached_camera_text.pos = gfx::FloatPos(0.0, -gfx::SPACING);
        self.detached_camera_text.scale = 3.0;
        self.detached_camera_text.color = gfx::Color::new(255, 0, 0, 255);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        if !self.detached_camera {
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

        if self.detached_camera {
            if graphics.renderer.get_key_state(gfx::Key::W) {
                self.target_position_y -= 0.5;
            }

            if graphics.renderer.get_key_state(gfx::Key::S) {
                self.target_position_y += 0.5;
            }

            if graphics.renderer.get_key_state(gfx::Key::A) {
                self.target_position_x -= 0.5;
            }

            if graphics.renderer.get_key_state(gfx::Key::D) {
                self.target_position_x += 0.5;
            }
        }
    }

    pub fn render(&self, graphics: &gfx::GraphicsContext) {
        if self.detached_camera {
            self.detached_camera_text.render(graphics, None, None);
        }
    }

    /// This function gets the position of the top left corner of the screen in world coordinates.
    pub fn get_top_left(&self, graphics: &gfx::GraphicsContext) -> (f32, f32) {
        let width = graphics.renderer.get_window_size().0 / RENDER_BLOCK_WIDTH;
        let height = graphics.renderer.get_window_size().1 / RENDER_BLOCK_WIDTH;
        (
            self.position_x - width / 2.0,
            self.position_y - height / 2.0,
        )
    }

    /// This function gets the position of the bottom right corner of the screen in world coordinates.
    pub fn get_bottom_right(&self, graphics: &gfx::GraphicsContext) -> (f32, f32) {
        let width = graphics.renderer.get_window_size().0 / RENDER_BLOCK_WIDTH;
        let height = graphics.renderer.get_window_size().1 / RENDER_BLOCK_WIDTH;
        (
            self.position_x + width / 2.0,
            self.position_y + height / 2.0,
        )
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(event) = event.downcast::<gfx::Event>() {
            if matches!(event, gfx::Event::KeyPress(gfx::Key::C, false)) {
                self.detached_camera = !self.detached_camera;
            }
        }
    }

    pub const fn is_detached(&self) -> bool {
        self.detached_camera
    }
}
