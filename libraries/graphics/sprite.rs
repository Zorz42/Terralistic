use super::{Color, Container, GraphicsContext, Orientation, Texture, TOP_LEFT};

/**
Sprite is a struct that represents a texture that can be rendered to the screen.
It has a position, a scale, and an orientation. It can also be flipped and has
a color.
 */
pub struct Sprite {
    pub texture: Texture,
    pub x: i32,
    pub y: i32,
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
    /**
    Creates a new Sprite with default values.
     */
    pub fn new() -> Sprite {
        Sprite {
            texture: Texture::new(),
            x: 0,
            y: 0,
            scale: 1.0,
            orientation: TOP_LEFT,
            flip: false,
            color: Color::new(255, 255, 255, 255),
        }
    }

    /**
    Returns width of the sprite.
     */
    pub fn get_width(&self) -> i32 {
        (self.texture.get_texture_width() as f32 * self.scale) as i32
    }

    /**
    Returns height of the sprite.
     */
    pub fn get_height(&self) -> i32 {
        (self.texture.get_texture_height() as f32 * self.scale) as i32
    }

    /**
    Generates containers for the sprite.
     */
    pub fn get_container(
        &self,
        graphics: &mut GraphicsContext,
        parent: Option<&Container>,
    ) -> Container {
        let mut container = Container::new(
            self.x,
            self.y,
            self.get_width(),
            self.get_height(),
            self.orientation,
        );
        container.update(graphics, parent);
        container
    }

    /**
    Renders the sprite.
     */
    pub fn render(&self, graphics: &mut GraphicsContext, parent: Option<&Container>) {
        let container = self.get_container(graphics, parent);
        self.texture.render(
            &graphics.renderer,
            self.scale,
            (
                container.get_absolute_rect().x,
                container.get_absolute_rect().y,
            ),
            None,
            self.flip,
            Some(self.color),
        );
    }
}
