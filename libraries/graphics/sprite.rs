use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{BaseUiElement, UiElement};

/// Sprite is a struct that represents a texture that can be rendered to the screen.
/// It has a position, a scale, and an orientation. It can also be flipped and has
/// a color.
pub struct Sprite {
    texture: gfx::Texture,
    pub pos: gfx::FloatPos,
    pub scale: f32,
    pub orientation: gfx::Orientation,
    pub flip: bool,
    pub color: gfx::Color,
    pub src_rect: gfx::Rect,
}

impl Sprite {
    /// Creates a new Sprite with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            texture: gfx::Texture::new(),
            pos: gfx::FloatPos(0.0, 0.0),
            scale: 1.0,
            orientation: gfx::TOP_LEFT,
            flip: false,
            color: gfx::Color::new(255, 255, 255, 255),
            src_rect: gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
        }
    }

    pub fn set_texture(&mut self, texture: gfx::Texture) {
        self.texture = texture;
        self.src_rect.size = self.texture.get_texture_size();
        self.src_rect.pos = gfx::FloatPos(0.0, 0.0);
    }

    #[must_use]
    pub const fn get_texture(&self) -> &gfx::Texture {
        &self.texture
    }

    #[must_use]
    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(self.texture.get_texture_size().0 * self.scale, self.texture.get_texture_size().1 * self.scale)
    }
}

impl UiElement for Sprite {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }

    /// Renders the sprite.
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent: &gfx::Container) {
        let size = gfx::FloatSize(self.src_rect.size.0 * self.scale, self.src_rect.size.1 * self.scale);

        let container = gfx::Container::new(graphics, self.pos, size, self.orientation, Some(parent));

        self.texture
            .render(graphics, self.scale, container.get_absolute_rect().pos, Some(self.src_rect), self.flip, Some(self.color));
    }

    /// Generates containers for the sprite.
    #[must_use]
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent))
    }
}
