use crate::{Color, Container, GraphicsContext, Texture, Sprite};
use super::Button;

const TEXT_INPUT_COLOR: Color = Color { r: 40, g: 40, b: 40, a: 255 };
const TEXT_INPUT_HOVER_COLOR: Color = Color { r: 80, g: 80, b: 80, a: 255 };

pub struct TextInput {
    pub button: Button,
    pub text: String,
    texture: Sprite,
    text_changed: bool,
}

impl TextInput {
    pub fn new() -> TextInput {
        let mut temp = TextInput {
            button: Button::new(),
            text: String::new(),
            texture: Sprite::new(),
            text_changed: false,
        };
        temp.button.hover_color = TEXT_INPUT_HOVER_COLOR;
        temp.button.color = TEXT_INPUT_COLOR;
        temp
    }
    
    /**
    Calculates the width based on the image width and the margin.
     */
    pub fn get_width(&self) -> i32 {
        self.button.get_width()
    }
    
    /**
    Calculates the height based on the image height and the margin.
     */
    pub fn get_height(&self) -> i32 {
        self.button.get_height()
    }
    
    /**
    Generates the container for the text input.
     */
    pub fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.button.x, self.button.y, self.button.get_width(), self.button.get_height(), self.button.orientation);
        container.update(graphics, parent_container);
        container
    }
    
    /**checks if the button is hovered with a mouse*/
    pub fn is_hovered(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> bool {
        if self.button.disabled {
            return false;
        }

        let container = self.button.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_x = graphics.renderer.get_mouse_x() as i32;
        let mouse_y = graphics.renderer.get_mouse_y() as i32;
        mouse_x >= rect.x && mouse_x <= rect.x + rect.w && mouse_y >= rect.y && mouse_y <= rect.y + rect.h
    }

    /**returns the text in the input box*/
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /**sets the text in the input box*/
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.text_changed = true;
    }

    /**renders the text input*/
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        if self.text_changed {
            self.texture.texture = Texture::load_from_surface(&graphics.font.create_text_surface(self.text.clone()));
            self.text_changed = false;
        }

        if self.text.is_empty() {
            self.button.render_text = true;
            self.button.text_color = if self.is_hovered(graphics, parent_container) == self.text.is_empty() {
                Some(self.button.color)
            } else {
                Some(self.button.hover_color)
            };
        } else {
            self.button.render_text = false;
        }

        let texture_scale = self.button.scale + self.button.hover_progress * 0.4;
        let x = rect.x + rect.w / 2 - (self.texture.texture.get_texture_width() as f32 * texture_scale / 2.0) as i32;
        let y = rect.y + rect.h / 2 - (self.texture.texture.get_texture_height() as f32 * texture_scale / 2.0) as i32;
        self.button.render(graphics, parent_container);
        self.texture.texture.render(&graphics.renderer, texture_scale, x, y, None, false, None);
    }
}