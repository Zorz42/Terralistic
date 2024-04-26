use super::Menu;
use crate::libraries::graphics as gfx;
use std::cell::Cell;
use std::rc::Rc;

use gfx::BaseUiElement;

pub struct ChoiceMenu {
    title_container: gfx::Container,
    button_container: gfx::Container,
    buttons: Vec<gfx::Button>,
    title_lines: Vec<gfx::Sprite>,
    esc_choice: Option<usize>,
    enter_choice: Option<usize>,
    close: Rc<Cell<bool>>,
}

impl ChoiceMenu {
    pub fn new(menu_title: &str, graphics: &gfx::GraphicsContext, buttons_properties: Vec<(&str, Box<dyn Fn()>)>, esc_choice: Option<usize>, enter_choice: Option<usize>) -> Self {
        let mut buttons: Vec<gfx::Button> = Vec::new();
        let mut buttons_width = 0.0;
        let mut max_button_height: f32 = 0.0;
        let close = Rc::new(Cell::new(false));
        for (text, function) in buttons_properties {
            let close_cloned = close.clone();
            let mut button_sprite = gfx::Button::new(move || {
                function();
                close_cloned.set(true);
            });
            button_sprite.scale = 3.0;
            button_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
            button_sprite.pos.0 = buttons_width;
            buttons_width += button_sprite.get_size().0 + gfx::SPACING;
            max_button_height = max_button_height.max(button_sprite.get_size().1);

            buttons.push(button_sprite);
        }
        let mut button_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);
        button_container.rect.size = gfx::FloatSize(buttons_width, max_button_height);
        button_container.rect.pos.1 = -gfx::SPACING;

        let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();
        let mut title_lines: Vec<gfx::Sprite> = Vec::new();
        let mut curr_y = 0.0;
        for line in text_lines_vec {
            let mut sprite = gfx::Sprite::new();
            sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(line, Some(200))));
            sprite.scale = 3.0;
            sprite.orientation = gfx::TOP;
            sprite.pos.1 = curr_y;
            curr_y += sprite.get_size().1 + gfx::SPACING;
            title_lines.push(sprite);
        }
        let title_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(600.0, 0.0), gfx::Orientation { x: 0.5, y: 0.3 }, None);

        Self {
            title_container,
            button_container,
            buttons,
            title_lines,
            esc_choice,
            enter_choice,
            close,
        }
    }
}

impl gfx::UiElement for ChoiceMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.title_container, &mut self.button_container]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.title_container, &self.button_container]
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, _parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::Escape => {
                    if let Some(button) = self.buttons.get(self.esc_choice.unwrap_or(usize::MAX)) {
                        button.press();
                    }
                    return true;
                }
                gfx::Key::Enter => {
                    if let Some(button) = self.buttons.get(self.enter_choice.unwrap_or(usize::MAX)) {
                        button.press();
                    }
                    return true;
                } //temporarily disabled
                gfx::Key::MouseLeft => {
                    for button in &mut self.buttons {
                        button.on_event(graphics, event, &self.button_container);
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, _parent_container: &gfx::Container) {
        for button in &mut self.buttons {
            button.update(graphics, &self.button_container);
        }
        for line in &mut self.title_lines {
            line.update(graphics, &self.title_container);
        }
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, _parent_container: &gfx::Container) {
        for button in &mut self.buttons {
            button.render(graphics, &self.button_container);
        }
        for line in &mut self.title_lines {
            line.render(graphics, &self.title_container);
        }
    }
}

impl Menu for ChoiceMenu {
    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }

    fn should_close(&mut self) -> bool {
        self.close.get()
    }
}
