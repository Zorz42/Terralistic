use super::{Color, Container, GraphicsContext, Orientation, Texture, TOP_LEFT};
use crate::libraries::graphics::{FloatPos, FloatSize};

/// Sprite is a struct that represents a texture that can be rendered to the screen.
/// It has a position, a scale, and an orientation. It can also be flipped and has
/// a color.
pub struct Sprite {
    pub texture: Texture,
    pub pos: FloatPos,
    pub scale: f32,
    pub orientation: Orientation,
    pub flip: bool,
    pub color: Color,
}

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}

impl Sprite {
    /// Creates a new Sprite with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            texture: Texture::new(),
            pos: FloatPos(0.0, 0.0),
            scale: 1.0,
            orientation: TOP_LEFT,
            flip: false,
            color: Color::new(255, 255, 255, 255),
        }
    }

    #[must_use]
    pub fn get_size(&self) -> FloatSize {
        FloatSize(
            self.texture.get_texture_size().0 * self.scale,
            self.texture.get_texture_size().1 * self.scale,
        )
    }

    /// Generates containers for the sprite.
    pub fn get_container(
        &self,
        graphics: &mut GraphicsContext,
        parent: Option<&Container>,
    ) -> Container {
        Container::new(
            graphics,
            self.pos,
            self.get_size(),
            self.orientation,
            parent,
        )
    }

    /// Renders the sprite.
    pub fn render(&self, graphics: &mut GraphicsContext, parent: Option<&Container>) {
        let container = self.get_container(graphics, parent);
        self.texture.render(
            &graphics.renderer,
            self.scale,
            container.get_absolute_rect().pos,
            None,
            self.flip,
            Some(self.color),
        );
    }
}
