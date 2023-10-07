use crate::client::game::camera::Camera;
use crate::libraries::graphics as gfx;
use crate::shared::blocks::RENDER_BLOCK_WIDTH;

const FADE_OUT_TIME_MS: i32 = 1000;

/// Floating text is text that randomly appears in the map and stays there.
/// It is used to display health changes, damage, chats, etc.
pub struct FloatingText {
    text_texture: gfx::Texture,
    x: f32,
    y: f32,
    lifetime_ms: i32,
    color: gfx::Color,
    spawn_time: std::time::Instant,
    scale: f32,
}

impl FloatingText {
    /// # Errors
    /// If the font couldn't be loaded.
    #[must_use]
    pub fn new(
        graphics: &gfx::GraphicsContext,
        text: &str,
        x: f32,
        y: f32,
        lifetime_ms: i32,
        color: gfx::Color,
        scale: f32,
    ) -> Self {
        let text_texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
        Self {
            text_texture,
            x,
            y,
            lifetime_ms,
            color,
            spawn_time: std::time::Instant::now(),
            scale,
        }
    }

    fn render(&self, graphics: &gfx::GraphicsContext, camera: &Camera) {
        let camera_pos = camera.get_top_left(graphics);
        let texture_size = self.text_texture.get_texture_size();

        self.text_texture.render(
            &graphics.renderer,
            self.scale,
            gfx::FloatPos(
                (self.x - camera_pos.0) * RENDER_BLOCK_WIDTH - texture_size.0 * self.scale / 2.0,
                (self.y - camera_pos.1) * RENDER_BLOCK_WIDTH - texture_size.1 * self.scale / 2.0,
            ),
            None,
            false,
            Some(self.color),
        );
    }

    fn is_dead(&self) -> bool {
        self.spawn_time.elapsed().as_millis() as i32 > self.lifetime_ms + FADE_OUT_TIME_MS
    }
}

/// Floating text manager.
pub struct FloatingTextManager {
    floating_texts: Vec<FloatingText>,
}

impl FloatingTextManager {
    pub const fn new() -> Self {
        Self {
            floating_texts: Vec::new(),
        }
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, camera: &Camera) {
        for floating_text in &self.floating_texts {
            floating_text.render(graphics, camera);
        }

        self.floating_texts
            .retain(|floating_text| !floating_text.is_dead());
    }

    pub fn spawn_text(&mut self, floating_text: FloatingText) {
        self.floating_texts.push(floating_text);
    }
}
