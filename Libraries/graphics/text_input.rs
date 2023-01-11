use crate::{Color, Container, GraphicsContext, Texture, Sprite, Orientation, TOP_LEFT, Rect, Key, Event};
use super::Button;
use std::cmp::max;

const TEXT_INPUT_COLOR: Color = Color { r: 40, g: 40, b: 40, a: 255 };
const TEXT_INPUT_HOVER_COLOR: Color = Color { r: 80, g: 80, b: 80, a: 255 };
use crate::theme::{GFX_DEFAULT_BUTTON_COLOR, GFX_DEFAULT_BUTTON_BORDER_COLOR, GFX_DEFAULT_HOVERED_BUTTON_COLOR, GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR, GFX_DEFAULT_BUTTON_MARGIN};

pub struct TextInput {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
    pub backtext_texture: Texture,
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
    pub inputed_text: String,
    inputed_text_texture: Sprite,
    text_changed: bool,
    selected: bool,
    cursor: [usize; 2],
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
            x: 0,
            y: 0,
            orientation: TOP_LEFT,
            backtext_texture: Texture::new(),
            margin: GFX_DEFAULT_BUTTON_MARGIN,
            scale: 1.0,
            color: TEXT_INPUT_COLOR,
            border_color: GFX_DEFAULT_BUTTON_BORDER_COLOR,
            hover_color: TEXT_INPUT_HOVER_COLOR,
            hover_border_color: GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR,
            disabled: false,
            hover_progress: 0.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
            inputed_text: String::new(),
            inputed_text_texture: Sprite::new(),
            text_changed: false,
            selected: false,
            cursor: [0, 0],
        }
    }
    
    /**
    Calculates the width based on the image width and the margin.
     */
    pub fn get_width(&self) -> i32 {
        max(
            ((self.backtext_texture.get_texture_width() as f32 + self.margin as f32 * 2.0) * self.scale) as i32,
            ((self.inputed_text_texture.texture.get_texture_width() as f32 + self.margin as f32 * 2.0) * self.scale * 1.1) as i32)
    }
    
    /**
    Calculates the height based on the image height and the margin.
     */
    pub fn get_height(&self) -> i32 {
        ((self.backtext_texture.get_texture_height() as f32 + self.margin as f32 * 2.0) * self.scale) as i32
    }
    
    /**
    Generates the container for the text input.
     */
    pub fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.x, self.y, self.get_width(), self.get_height(), self.orientation);
        container.update(graphics, parent_container);
        container
    }
    
    /**checks if the button is hovered with a mouse*/
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

    /**returns the text in the input box*/
    pub fn get_text(&self) -> &str {
        &self.inputed_text
    }

    /**sets the text in the input box*/
    pub fn set_text(&mut self, text: &str) {
        self.inputed_text = text.to_string();
        self.text_changed = true;
    }

    /**renders the text input*/
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        if self.text_changed && self.inputed_text.len() > 0 {
            self.inputed_text_texture.texture = Texture::load_from_surface(&graphics.font.create_text_surface(self.inputed_text.clone()));
        }
        self.text_changed = false;

        let hover_progress_target = if self.is_hovered(graphics, parent_container) {
            if graphics.renderer.get_key_state(Key::MouseLeft) {
                0.8
            } else {
                1.0
            }
        } else {
            0.0
        };
        if self.selected {
            self.hover_progress = 0.8;
        }

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
        let texture_scale = self.scale + self.hover_progress * 0.4;

        rect.render(graphics, self.color);
        rect.render_outline(&graphics, self.border_color);
        hover_rect.render(&graphics, button_color);
        hover_rect.render_outline(&graphics, button_border_color);
        if self.inputed_text.is_empty() && !self.selected {
            self.backtext_texture.render(
                &graphics.renderer, texture_scale,
                rect.x + rect.w / 2 - (self.backtext_texture.get_texture_width() as f32 * texture_scale / 2.0) as i32,
                rect.y + rect.h / 2 - (self.backtext_texture.get_texture_height() as f32 * texture_scale / 2.0) as i32,
                None, false,
                if self.is_hovered(graphics, parent_container) { Some(self.color) } else { Some(self.hover_color) });
        } else if !self.inputed_text.is_empty() {
            self.inputed_text_texture.texture.render(
                &graphics.renderer, texture_scale,
                rect.x + rect.w / 2 - (self.inputed_text_texture.texture.get_texture_width() as f32 * texture_scale / 2.0) as i32,
                rect.y + rect.h / 2 - (self.inputed_text_texture.texture.get_texture_height() as f32 * texture_scale / 2.0) as i32,
                None, false, None);
        }
    }

    pub fn on_event(&mut self, event: &Event, graphics: &GraphicsContext, parent_container: Option<&Container>) {//TODO add text processing with closures?
        match event {
            Event::KeyPress (key) => {
                if key == &Key::MouseLeft {
                    if self.is_hovered(&graphics, parent_container) {
                        if !self.disabled {
                            self.selected = true;
                        }
                    } else {
                        self.selected = false;
                    }
                }
                if !self.selected {
                    return;
                }
                match key {
                    Key::Backspace => {
                        if self.cursor[0] != self.cursor[1] {
                            self.inputed_text.replace_range(self.cursor[0]..self.cursor[1], "");
                            self.cursor[1] = self.cursor[0];
                        } else if self.cursor[0] > 0 {
                            self.inputed_text.remove(self.cursor[0] - 1);
                            self.cursor[0] -= 1;
                        }
                        self.cursor[1] = self.cursor[0];
                        self.text_changed = true;
                    },
                    _ => {
                        let input = TextInput::translate_keycode_to_char(*key);
                        if input.is_some() {
                            if self.cursor[0] != self.cursor[1] {
                                self.inputed_text.replace_range(self.cursor[0]..self.cursor[1], "");
                                self.cursor[1] = self.cursor[0];
                            }
                            self.inputed_text.insert(self.cursor[0], input.unwrap());
                            self.cursor[0] += 1;
                            self.text_changed = true;
                            self.cursor[1] = self.cursor[0];
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /**this function returns the inputed character from keycode*/
    fn translate_keycode_to_char(keycode: Key) -> Option<char> {
        match keycode {
            Key::A => Some('a'),
            Key::B => Some('b'),
            Key::C => Some('c'),
            Key::D => Some('d'),
            Key::E => Some('e'),
            Key::F => Some('f'),
            Key::G => Some('g'),
            Key::H => Some('h'),
            Key::I => Some('i'),
            Key::J => Some('j'),
            Key::K => Some('k'),
            Key::L => Some('l'),
            Key::M => Some('m'),
            Key::N => Some('n'),
            Key::O => Some('o'),
            Key::P => Some('p'),
            Key::Q => Some('q'),
            Key::R => Some('r'),
            Key::S => Some('s'),
            Key::T => Some('t'),
            Key::U => Some('u'),
            Key::V => Some('v'),
            Key::W => Some('w'),
            Key::X => Some('x'),
            Key::Y => Some('y'),
            Key::Z => Some('z'),
            Key::Space => Some(' '),
            _ => None
        }
    }
}