use crate::{Color, Container, GraphicsContext, Key, Orientation, Rect, Texture, TOP_LEFT};
use crate::theme::{GFX_DEFAULT_BUTTON_COLOR, GFX_DEFAULT_BUTTON_BORDER_COLOR, GFX_DEFAULT_HOVERED_BUTTON_COLOR, GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR, GFX_DEFAULT_BUTTON_MARGIN};

/**
A Button is a rectangle with an image in it.
It can be clicked and has a hover animation.
 */
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
    pub texture: Texture,
    pub margin: i32,
    pub scale: f32,
    pub color: Color,
    pub border_color: Color,
    pub hover_color: Color,
    pub hover_border_color: Color,
    pub disabled: bool,
    pub hover_progress: f32,
    timer: std::time::Instant,
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
            texture: Texture::new(),
            margin: GFX_DEFAULT_BUTTON_MARGIN,
            scale: 1.0,
            color: GFX_DEFAULT_BUTTON_COLOR,
            border_color: GFX_DEFAULT_BUTTON_BORDER_COLOR,
            hover_color: GFX_DEFAULT_HOVERED_BUTTON_COLOR,
            hover_border_color: GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR,
            disabled: false,
            hover_progress: 0.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
        }
    }

    /**
    Calculates the width based on the image width and the margin.
     */
    pub fn get_width(&self) -> i32 {
        ((self.texture.get_texture_width() as f32 + self.margin as f32 * 2.0) * self.scale) as i32
    }

    /**
    Calculates the height based on the image height and the margin.
     */
    pub fn get_height(&self) -> i32 {
        ((self.texture.get_texture_height() as f32 + self.margin as f32 * 2.0) * self.scale) as i32
    }

    /**
    Generates the container for the button.
     */
    pub fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.x, self.y, self.get_width(), self.get_height(), self.orientation);
        container.update(graphics, parent_container);
        container
    }

    /**
    Checks if the button is hovered with a mouse.
     */
    pub fn is_hovered(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> bool {
        if self.disabled {
            return false;
        }

        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_x = graphics.renderer.get_mouse_x() as i32;
        let mouse_y = graphics.renderer.get_mouse_y() as i32;
        mouse_x >= rect.x && mouse_x <= rect.x + rect.w && mouse_y >= rect.y && mouse_y <= rect.y + rect.h
    }

    /**
    Renders the button.
     */
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
            (self.hover_color.r as f32 * self.hover_progress + self.color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.g as f32 * self.hover_progress + self.color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.b as f32 * self.hover_progress + self.color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.a as f32 * self.hover_progress + self.color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        let button_border_color = Color::new(
            (self.hover_border_color.r as f32 * self.hover_progress + self.border_color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.g as f32 * self.hover_progress + self.border_color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.b as f32 * self.hover_progress + self.border_color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.a as f32 * self.hover_progress + self.border_color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        let padding = (1.0 - self.hover_progress) * 30.0;
        let hover_rect = Rect::new(rect.x + padding as i32, rect.y + padding as i32, std::cmp::max(0, rect.w - 2 * padding as i32), std::cmp::max(0, rect.h - 2 * padding as i32));
        rect.render(&graphics, self.color);
        rect.render_outline(&graphics, self.border_color);
        hover_rect.render(&graphics, button_color);
        hover_rect.render_outline(&graphics, button_border_color);

        let texture_scale = self.scale + self.hover_progress * 0.4;
        let x = rect.x + rect.w / 2 - (self.texture.get_texture_width() as f32 * texture_scale / 2.0) as i32;
        let y = rect.y + rect.h / 2 - (self.texture.get_texture_height() as f32 * texture_scale / 2.0) as i32;
        self.texture.render(&graphics.renderer, texture_scale, x, y, None, false, None);
    }
}