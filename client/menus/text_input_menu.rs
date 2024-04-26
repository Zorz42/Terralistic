use std::cell::RefCell;
use std::rc::Rc;

use crate::libraries::graphics::{self as gfx, UiElement};
use gfx::BaseUiElement;

use super::Menu;

pub struct TextInputMenu {
    title: gfx::Sprite,
    back_button: gfx::Button,
    confirm_button: gfx::Button,
    input_field: gfx::TextInput,
    text: Rc<RefCell<Option<String>>>,
    close_menu: bool,
}

impl TextInputMenu {
    pub fn new(graphics: &gfx::GraphicsContext, title_text: &str, text: Rc<RefCell<Option<String>>>) -> Self {
        let back_str = "Back";
        let mut back_button = gfx::Button::new(|| {});
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(back_str, None));
        back_button.orientation = gfx::BOTTOM;

        let confirm_str = "Continue";
        let mut confirm_button = gfx::Button::new(|| {});
        confirm_button.scale = 3.0;
        confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(confirm_str, None));
        confirm_button.pos.0 = back_button.get_size().0 + gfx::SPACING;
        confirm_button.orientation = gfx::BOTTOM;

        back_button.pos = gfx::FloatPos(-confirm_button.get_size().0 / 2.0 - gfx::SPACING, -gfx::SPACING);
        confirm_button.pos = gfx::FloatPos(back_button.get_size().0 / 2.0 + gfx::SPACING, -gfx::SPACING);

        let mut input_field = gfx::TextInput::new(graphics);
        input_field.scale = 3.0;
        input_field.orientation = gfx::CENTER;

        let mut title_sprite = gfx::Sprite::new();
        title_sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(title_text, None)));
        title_sprite.scale = 3.0;
        title_sprite.orientation = gfx::TOP;

        Self {
            title: title_sprite,
            back_button,
            confirm_button,
            input_field,
            text,
            close_menu: false,
        }
    }
}

impl UiElement for TextInputMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.back_button, &mut self.confirm_button, &mut self.input_field, &mut self.title]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.back_button, &self.confirm_button, &self.input_field, &self.title]
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if self.back_button.on_event_inner(graphics, event, parent_container) {
            self.close_menu = true;
            return true;
        }
        if self.confirm_button.on_event_inner(graphics, event, parent_container) {
            *self.text.borrow_mut() = Some(self.input_field.get_text().to_owned());
            self.close_menu = true;
            return true;
        }

        //sorts out the events
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::Escape => {
                    self.close_menu = true;
                    return true;
                }
                gfx::Key::Enter => {
                    *self.text.borrow_mut() = Some(self.input_field.get_text().to_owned());
                    self.close_menu = true;
                    return true;
                }
                _ => {}
            }
        }

        false
    }
}

impl Menu for TextInputMenu {
    fn should_close(&mut self) -> bool {
        self.close_menu
    }

    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }
}
