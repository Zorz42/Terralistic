use super::theme::{
    GFX_DEFAULT_BUTTON_BORDER_COLOR, GFX_DEFAULT_BUTTON_COLOR, GFX_DEFAULT_BUTTON_PADDING,
    GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR, GFX_DEFAULT_HOVERED_BUTTON_COLOR,
};
use super::{Color, Container, GraphicsContext, Key, Orientation, Rect, Texture, TOP_LEFT};
use crate::libraries::graphics::{FloatPos, FloatSize};

/// A Button is a rectangle with an image in it.
/// It can be clicked and has a hover animation.
pub struct Button {
    pub pos: FloatPos,
    pub orientation: Orientation,
    pub texture: Texture,
    pub padding: f32,
    pub scale: f32,
    pub color: Color,
    pub border_color: Color,
    pub hover_color: Color,
    pub hover_border_color: Color,
    pub disabled: bool,
    pub darken_on_disabled: bool,
    pub hover_progress: f32,
    timer: std::time::Instant,
    timer_counter: u32,
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    /// Creates a new button.
    #[must_use]
    pub fn new() -> Self {
        Self {
            pos: FloatPos(0.0, 0.0),
            orientation: TOP_LEFT,
            texture: Texture::new(),
            padding: GFX_DEFAULT_BUTTON_PADDING,
            scale: 1.0,
            color: GFX_DEFAULT_BUTTON_COLOR,
            border_color: GFX_DEFAULT_BUTTON_BORDER_COLOR,
            hover_color: GFX_DEFAULT_HOVERED_BUTTON_COLOR,
            hover_border_color: GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR,
            disabled: false,
            darken_on_disabled: false,
            hover_progress: 0.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
        }
    }

    /// Calculates the size based on the image height and the margin.
    #[must_use]
    pub fn get_size(&self) -> FloatSize {
        FloatSize(
            (self.texture.get_texture_size().0 + self.padding * 2.0) * self.scale,
            (self.texture.get_texture_size().1 + self.padding * 2.0) * self.scale,
        )
    }

    /// Generates the container for the button.
    #[must_use]
    pub fn get_container(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> Container {
        Container::new(
            graphics,
            self.pos,
            self.get_size(),
            self.orientation,
            parent_container,
        )
    }

    /// Checks if the button is hovered with a mouse.
    #[must_use]
    pub fn is_hovered(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> bool {
        if self.disabled {
            return false;
        }

        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_pos = graphics.renderer.get_mouse_pos();
        mouse_pos.0 >= rect.pos.0
            && mouse_pos.0 <= rect.pos.0 + rect.size.0
            && mouse_pos.1 >= rect.pos.1
            && mouse_pos.1 <= rect.pos.1 + rect.size.1
    }

    /// Renders the button.
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        let hover_progress_target = if self.is_hovered(graphics, parent_container) {
            if graphics.renderer.get_key_state(Key::MouseLeft) {
                0.8
            } else {
                1.0
            }
        } else {
            0.0
        };

        while self.timer_counter < self.timer.elapsed().as_millis() as u32 {
            self.hover_progress += (hover_progress_target - self.hover_progress) / 40.0;
            if (hover_progress_target - self.hover_progress).abs() <= 0.01 {
                self.hover_progress = hover_progress_target;
            }
            self.timer_counter += 1;
        }

        let button_color = Color::new(
            (self.hover_color.r as f32 * self.hover_progress
                + self.color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.g as f32 * self.hover_progress
                + self.color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.b as f32 * self.hover_progress
                + self.color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.a as f32 * self.hover_progress
                + self.color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        let button_border_color = Color::new(
            (self.hover_border_color.r as f32 * self.hover_progress
                + self.border_color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.g as f32 * self.hover_progress
                + self.border_color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.b as f32 * self.hover_progress
                + self.border_color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.a as f32 * self.hover_progress
                + self.border_color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        let padding = (1.0 - self.hover_progress) * 30.0;
        let hover_rect = Rect::new(
            rect.pos + FloatPos(padding, padding),
            FloatSize(
                f32::max(0.0, rect.size.0 - 2.0 * padding),
                f32::max(0.0, rect.size.1 - 2.0 * padding),
            ),
        );
        rect.render(graphics, self.color);
        rect.render_outline(graphics, self.border_color);
        hover_rect.render(graphics, button_color);
        hover_rect.render_outline(graphics, button_border_color);

        let texture_scale = self.scale + self.hover_progress * 0.4;
        let x = rect.pos.0 + rect.size.0 / 2.0
            - self.texture.get_texture_size().0 * texture_scale / 2.0;
        let y = rect.pos.1 + rect.size.1 / 2.0
            - self.texture.get_texture_size().1 * texture_scale / 2.0;
        self.texture.render(
            &graphics.renderer,
            texture_scale,
            FloatPos(x, y),
            None,
            false,
            None,
        );
        if self.disabled && self.darken_on_disabled {
            rect.render(graphics, Color::new(0, 0, 0, 100));
        }
    }
}
