use crate::{Color, Container, GraphicsContext, Orientation, Texture, Timer, TOP_LEFT};
use crate::theme::{
    GFX_DEFAULT_BUTTON_COLOR,
    GFX_DEFAULT_BUTTON_BORDER_COLOR,
    GFX_DEFAULT_HOVERED_BUTTON_COLOR,
    GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR,
    GFX_DEFAULT_TEXT_COLOR,
    GFX_SHADOW_BLUR,
};

/**
A Button is a rectangle with an image in it.
It can be clicked and has a hover animation.
 */
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
    pub image: Texture,
    pub margin: i32,
    pub scale: f32,
    pub color: Color,
    pub border_color: Color,
    pub hover_color: Color,
    pub hover_border_color: Color,
    pub disabled: bool,
    hover_progress: f32,
    timer: Timer,
    timer_counter: u32,
}

impl Button {
    /**
    Creates a new button.
     */
    pub fn new() -> Self {
        Button {
            x: 0,
            y: 0,
            orientation: TOP_LEFT,
            image: Texture::new(),
            margin: 0,
            scale: 1.0,
            color: GFX_DEFAULT_BUTTON_COLOR,
            border_color: GFX_DEFAULT_BUTTON_COLOR,
            hover_color: GFX_DEFAULT_HOVERED_BUTTON_COLOR,
            hover_border_color: GFX_DEFAULT_HOVERED_BUTTON_COLOR,
            disabled: false,
            hover_progress: 0.0,
            timer: Timer::new(),
            timer_counter: 0,
        }
    }

    /**
    Calculates the width based on the image width and the margin.
     */
    pub fn get_width(&self) -> i32 {
        ((self.image.get_texture_width() as f32 + self.margin as f32 * 2.0) * self.scale) as i32
    }

    /**
    Calculates the height based on the image height and the margin.
     */
    pub fn get_height(&self) -> i32 {
        ((self.image.get_texture_height() as f32 + self.margin as f32 * 2.0) * self.scale) as i32
    }

    /**
    Generates the container for the button.
     */
    pub fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.x, self.x, self.get_width(), self.get_height(), self.orientation);
        container.update(graphics, parent_container);
        container
    }

    /**
    Checks if the button is hovered with a mouse.
     */
    pub fn is_hovered(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> bool {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        //rect.is_hovered(graphics)
        false
    }

    /**
    Renders the button.
     */
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {

    }
}