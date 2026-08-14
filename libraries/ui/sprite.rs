use super::UiElement;
use crate::libraries::graphics as gfx;

/// A texture that positions itself like a UI element: a position, a scale and an orientation,
/// plus a flip, a tint and a source rectangle.
pub struct Sprite {
    texture: gfx::Texture,
    pub pos: gfx::FloatPos,
    pub scale: f32,
    pub orientation: super::Orientation,
    pub flip: bool,
    pub color: gfx::Color,
    pub src_rect: gfx::Rect,
}

impl Sprite {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            texture: gfx::Texture::new(),
            pos: gfx::FloatPos(0.0, 0.0),
            scale: 1.0,
            orientation: super::TOP_LEFT,
            flip: false,
            color: gfx::Color::new(255, 255, 255, 255),
            src_rect: gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
        }
    }

    /// Replaces the texture and resets the source rectangle to the whole of it, so a sprite
    /// reused for a new texture does not keep cropping to the old one.
    pub fn set_texture(&mut self, texture: gfx::Texture) {
        self.texture = texture;
        self.src_rect = gfx::Rect::new(gfx::FloatPos(0.0, 0.0), self.texture.get_texture_size());
    }

    #[must_use]
    pub const fn get_texture(&self) -> &gfx::Texture {
        &self.texture
    }

    /// The drawn size, which is the *source rectangle* scaled rather than the whole texture -
    /// layout and drawing have to agree when a sprite crops.
    #[must_use]
    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(self.src_rect.size.0 * self.scale, self.src_rect.size.1 * self.scale)
    }
}

impl UiElement for Sprite {
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent: &super::Container) {
        let container = self.get_container(graphics, parent);
        self.texture
            .render(graphics, self.scale, container.get_absolute_rect().pos, Some(self.src_rect), self.flip, Some(self.color));
    }

    fn get_container(&self, graphics: &dyn super::UiContext, parent: &super::Container) -> super::Container {
        super::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent))
    }
}
