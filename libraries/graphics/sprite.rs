use crate::libraries::graphics as gfx;

/// Sprite is a struct that represents a texture that can be rendered to the screen.
/// It has a position, a scale, and an orientation. It can also be flipped and has
/// a color.
pub struct Sprite {
    pub texture: gfx::Texture,
    pub pos: gfx::FloatPos,
    pub scale: f32,
    pub orientation: gfx::Orientation,
    pub flip: bool,
    pub color: gfx::Color,
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
        }
    }

    #[must_use]
    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(
            self.texture.get_texture_size().0 * self.scale,
            self.texture.get_texture_size().1 * self.scale,
        )
    }

    /// Generates containers for the sprite.
    #[must_use]
    pub fn get_container(
        &self,
        graphics: &gfx::GraphicsContext,
        parent: Option<&gfx::Container>,
    ) -> gfx::Container {
        gfx::Container::new(
            graphics,
            self.pos,
            self.get_size(),
            self.orientation,
            parent,
        )
    }

    /// Renders the sprite.
    pub fn render(
        &self,
        graphics: &gfx::GraphicsContext,
        parent: Option<&gfx::Container>,
        src_rect: Option<gfx::Rect>,
    ) {
        let container = self.get_container(graphics, parent);
        self.texture.render(
            &graphics.renderer,
            self.scale,
            container.get_absolute_rect().pos,
            src_rect,
            self.flip,
            Some(self.color),
        );
    }
}
