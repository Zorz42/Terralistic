use crate::{Color, Container, GraphicsContext, Texture, Sprite, Orientation, TOP_LEFT, Rect, Key, Event};
use super::Button;
use std::cmp::max;
use sdl2::libc::scanf;
use serde::de::Unexpected::Str;
use crate::theme::{GFX_DEFAULT_BUTTON_COLOR, GFX_DEFAULT_BUTTON_BORDER_COLOR, GFX_DEFAULT_HOVERED_BUTTON_COLOR, GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR, GFX_DEFAULT_BUTTON_PADDING, GFX_DEFAULT_TEXT_INPUT_WIDTH, GFX_DEFAULT_TEXT_INPUT_COLOR, GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR, GFX_DEFAULT_TEXT_INPUT_PADDING};

pub struct TextInput {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
    pub width: i32,
    hint_texture: Texture,
    pub padding: i32,
    pub scale: f32,
    pub color: Color,
    pub border_color: Color,
    pub hover_color: Color,
    pub hover_border_color: Color,
    hover_progress: f32,
    timer: std::time::Instant,
    timer_counter: u32,
    text: String,
    text_texture: Texture,
    text_changed: bool,
    pub selected: bool,
    cursor: (usize, usize),
}

impl TextInput {
    pub fn new(graphics: &mut GraphicsContext) -> TextInput {
        TextInput {
            x: 0,
            y: 0,
            orientation: TOP_LEFT,
            width: GFX_DEFAULT_TEXT_INPUT_WIDTH,
            hint_texture: Texture::new(),
            padding: GFX_DEFAULT_TEXT_INPUT_PADDING,
            scale: 1.0,
            color: GFX_DEFAULT_TEXT_INPUT_COLOR,
            border_color: GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR,
            hover_color: GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR,
            hover_border_color: GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR,
            hover_progress: 0.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
            text: String::new(),
            text_texture: Texture::load_from_surface(&graphics.font.create_text_surface(String::from(""))),
            text_changed: false,
            selected: false,
            cursor: (0, 0),
        }
    }
    
    /**
    Calculates the width.
     */
    pub fn get_width(&self) -> i32 {
        ((self.width + self.padding * 2) as f32 * self.scale) as i32
    }
    
    /**
    Calculates the height.
     */
    pub fn get_height(&self) -> i32 {
        ((self.text_texture.get_texture_height() + self.padding * 2) as f32 * self.scale) as i32
    }
    
    /**
    Generates the container for the text input. It it private, since a text input should never contain other elements.
     */
    fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.x, self.y, self.get_width(), self.get_height(), self.orientation);
        container.update(graphics, parent_container);
        container
    }
    
    /**
    Checks if the button is hovered with a mouse
     */
    pub fn is_hovered(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> bool {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_x = graphics.renderer.get_mouse_x() as i32;
        let mouse_y = graphics.renderer.get_mouse_y() as i32;
        mouse_x >= rect.x && mouse_x <= rect.x + rect.w && mouse_y >= rect.y && mouse_y <= rect.y + rect.h
    }

    /**
    returns the text in the input box
     */
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /**
    sets the text in the input box
     */
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_changed = true;
    }

    /**
    renders the text input
     */
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        if self.text_changed && self.text.len() > 0 {
            self.text_texture = Texture::load_from_surface(&graphics.font.create_text_surface(self.text.clone()));
            self.text_changed = false;
        }

        let hover_progress_target = if self.is_hovered(graphics, parent_container) { 1.0 } else { 0.0 };

        while self.timer_counter < self.timer.elapsed().as_millis() as u32 {
            self.hover_progress += (hover_progress_target - self.hover_progress) / 40.0;
            if (hover_progress_target - self.hover_progress).abs() <= 0.01 {
                self.hover_progress = hover_progress_target;
            }
            self.timer_counter += 1;
        }

        let color = Color::new(
            (self.hover_color.r as f32 * self.hover_progress + self.color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.g as f32 * self.hover_progress + self.color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.b as f32 * self.hover_progress + self.color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.a as f32 * self.hover_progress + self.color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        let border_color = Color::new(
            (self.hover_border_color.r as f32 * self.hover_progress + self.border_color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.g as f32 * self.hover_progress + self.border_color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.b as f32 * self.hover_progress + self.border_color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.a as f32 * self.hover_progress + self.border_color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        rect.render(graphics, color);
        rect.render_outline(&graphics, border_color);

        if self.text.is_empty() && !self.selected {
            /*self.backtext_texture.render(
                &graphics.renderer, texture_scale,
                rect.x + rect.w / 2 - (self.backtext_texture.get_texture_width() as f32 * texture_scale / 2.0) as i32,
                rect.y + rect.h / 2 - (self.backtext_texture.get_texture_height() as f32 * texture_scale / 2.0) as i32,
                None, false,
                if self.is_hovered(graphics, parent_container) { Some(self.color) } else { Some(self.hover_color) });*/
        } else {
            if !self.text.is_empty() {
                let mut src_rect = Rect::new(0, 0, self.text_texture.get_texture_width(), self.text_texture.get_texture_height());
                src_rect.w = i32::min(src_rect.w, self.width);
                src_rect.x = self.text_texture.get_texture_width() as i32 - src_rect.w;

                self.text_texture.render(
                    &graphics.renderer, self.scale,
                    rect.x + (self.padding as f32 * self.scale) as i32,
                    rect.y + rect.h / 2 - (self.text_texture.get_texture_height() as f32 * self.scale / 2.0) as i32,
                    Some(src_rect), false, None
                );
            }
            //TODO render cursor
        }
    }

    pub fn on_event(&mut self, event: &Event, graphics: &GraphicsContext, parent_container: Option<&Container>) {//TODO add text processing with closures?
        match event {
            Event::TextInput(text) => {
                if self.selected {
                    if self.cursor.0 != self.cursor.1 {
                        self.text.replace_range(self.cursor.0..self.cursor.1, "");
                        self.cursor.1 = self.cursor.0;
                    }
                    self.text.insert_str(self.cursor.0, text);
                    self.cursor.0 += text.len();
                    self.text_changed = true;
                    self.cursor.1 = self.cursor.0;
                }
            }

            Event::KeyPress (key) => {
                if key == &Key::MouseLeft {
                    self.selected = self.is_hovered(graphics, parent_container);
                }

                if !self.selected {
                    return;
                }

                match key {
                    Key::Backspace => {
                        if self.cursor.0 != self.cursor.1 {
                            self.text.replace_range(self.cursor.0..self.cursor.1, "");
                            self.cursor.1 = self.cursor.0;
                        } else if self.cursor.0 > 0 {
                            self.text.remove(self.cursor.0 - 1);
                            self.cursor.0 -= 1;
                        }
                        self.cursor.1 = self.cursor.0;
                        self.text_changed = true;
                    },
                    Key::Delete => {
                        if self.cursor.0 != self.cursor.1 {
                            self.text.replace_range(self.cursor.0..self.cursor.1, "");
                            self.cursor.1 = self.cursor.0;
                        } else if self.text.len() > self.cursor.0 {
                            self.text.remove(self.cursor.0);
                        }
                        self.text_changed = true;
                    },
                    Key::Left => {
                        if self.cursor.0 == self.cursor.1 {
                            if self.cursor.0 > 0 {
                                self.cursor.0 -= 1;
                            }
                        }
                        self.cursor.1 = self.cursor.0;
                    },
                    Key::Right => {
                        if self.cursor.0 == self.cursor.1 {
                            if self.cursor.0 < self.text.len() {
                                self.cursor.1 += 1;
                            }
                        }
                        self.cursor.0 = self.cursor.1;
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}