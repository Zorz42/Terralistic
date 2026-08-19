use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::ui;
use crate::libraries::ui::BaseUiElement;
use crate::shared::blocks::RENDER_BLOCK_WIDTH;

/// How fast the camera closes the distance to the player, and how near counts as arrived.
/// The epsilon is in blocks, so a hundredth of a block is far inside one pixel.
const CAMERA_SMOOTH_FACTOR: f32 = 33.3;
const CAMERA_EPSILON: f32 = 0.01;

/// Camera is a struct that handles the camera position.
use crate::libraries::ui::UiContext;
pub struct Camera {
    target_position_x: f32,
    target_position_y: f32,
    position_x: f32,
    position_y: f32,
    /// Where the camera was one tick ago, and how far through the current one the frame being
    /// drawn is. In steady state the camera moves at exactly the speed it is following, so a
    /// camera left on tick boundaries judders the whole world by as much as an uninterpolated
    /// player judders against it.
    previous_position_x: f32,
    previous_position_y: f32,
    fraction_of_step: f32,
    detached: bool,
    detached_text: ui::Sprite,
}

impl Camera {
    pub const fn new() -> Self {
        Self {
            target_position_x: 0.0,
            target_position_y: 0.0,
            position_x: 0.0,
            position_y: 0.0,
            previous_position_x: 0.0,
            previous_position_y: 0.0,
            fraction_of_step: 0.0,
            detached: false,
            detached_text: ui::Sprite::new(),
        }
    }

    pub fn load_resources(&mut self, graphics: &gfx::GraphicsContext) {
        self.detached_text
            .set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Camera is detached", None)));
        self.detached_text.orientation = ui::BOTTOM;
        self.detached_text.pos = gfx::FloatPos(0.0, -ui::SPACING);
        self.detached_text.scale = 3.0;
        self.detached_text.color = gfx::Color::new(255, 0, 0, 255);
    }

    pub const fn set_position(&mut self, x: f32, y: f32) {
        if !self.detached {
            self.target_position_x = x;
            self.target_position_y = y;
        }
    }

    pub const fn get_position(&self) -> gfx::FloatPos {
        let (x, y) = self.render_position();
        gfx::FloatPos(x, y)
    }

    /// How far through the current tick the frame being drawn is. Set once a frame, before
    /// anything renders.
    pub const fn set_fraction_of_step(&mut self, fraction: f32) {
        self.fraction_of_step = fraction;
    }

    /// Where to draw from: between the last two ticks rather than on the newer of them.
    const fn render_position(&self) -> (f32, f32) {
        (
            self.previous_position_x + (self.position_x - self.previous_position_x) * self.fraction_of_step,
            self.previous_position_y + (self.position_y - self.previous_position_y) * self.fraction_of_step,
        )
    }

    pub fn update_ms(&mut self, graphics: &gfx::GraphicsContext) {
        // 0.03 is 1/33.3, which is what this used to be written as - the only smoothing in
        // the game that was spelled as a multiply
        self.previous_position_x = self.position_x;
        self.previous_position_y = self.position_y;
        self.position_x = ui::approach(self.position_x, self.target_position_x, CAMERA_SMOOTH_FACTOR, CAMERA_EPSILON);
        self.position_y = ui::approach(self.position_y, self.target_position_y, CAMERA_SMOOTH_FACTOR, CAMERA_EPSILON);

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
            self.detached_text.render(graphics, &ui::Container::default(graphics));
        }
    }

    /// This function gets the position of the top left corner of the screen in world coordinates.
    pub fn get_top_left(&self, graphics: &gfx::GraphicsContext) -> (f32, f32) {
        let width = graphics.get_window_size().0 / RENDER_BLOCK_WIDTH;
        let height = graphics.get_window_size().1 / RENDER_BLOCK_WIDTH;
        let (x, y) = self.render_position();
        (x - width / 2.0, y - height / 2.0)
    }

    /// This function gets the position of the bottom right corner of the screen in world coordinates.
    pub fn get_bottom_right(&self, graphics: &gfx::GraphicsContext) -> (f32, f32) {
        let width = graphics.get_window_size().0 / RENDER_BLOCK_WIDTH;
        let height = graphics.get_window_size().1 / RENDER_BLOCK_WIDTH;
        let (x, y) = self.render_position();
        (x + width / 2.0, y + height / 2.0)
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
