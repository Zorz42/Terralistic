use anyhow::Result;

use crate::client::game::camera::Camera;
use crate::libraries::graphics as gfx;

/// Floating text is text that randomly appears in the map and stays there.
/// It is used to display health changes, damage, chats, etc.
pub struct FloatingText {
    text: String,
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
    pub fn new(
        graphics: &mut gfx::GraphicsContext,
        text: String,
        x: f32,
        y: f32,
        lifetime_ms: i32,
        color: gfx::Color,
        scale: f32,
    ) -> Result<Self> {
        let text_texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&text, None));
        Ok(Self {
            text,
            text_texture,
            x,
            y,
            lifetime_ms,
            color,
            spawn_time: std::time::Instant::now(),
            scale,
        })
    }

    pub fn render(&self, graphics: &mut gfx::GraphicsContext, camera: &Camera) {
        self.text_texture.render(
            &graphics.renderer,
            3.0,
            gfx::FloatPos(self.x, self.y) - camera.get_position(),
            None,
            false,
            None,
        );
    }
}
