use crate::libraries::graphics as gfx;
use std::cell::RefCell;
use std::rc::Rc;

use gfx::BaseUiElement;

pub struct ChoiceMenu {
    title_container: gfx::Container,
    button_container: gfx::Container,
    esc_choice: Option<usize>,
    enter_choice: Option<usize>,
    align_left: bool,
    button_press: Rc<RefCell<Option<usize>>>,
}

impl ChoiceMenu {
    pub fn new(
        menu_title: &str,
        graphics: &gfx::GraphicsContext,
        button_texts: &[&str],
        esc_choice: Option<usize>,
        enter_choice: Option<usize>,
        button_press: Rc<RefCell<Option<usize>>>,
        align_left: bool,
    ) -> Self {
        let mut buttons: Vec<Box<dyn BaseUiElement>> = Vec::new();
        let mut buttons_width = 0.0;
        let mut max_button_height: f32 = 0.0;
        for (index, text) in button_texts.iter().enumerate() {
            let button_press = button_press.clone();
            let mut button_sprite = gfx::Button::new(move || *button_press.borrow_mut() = Some(index));
            button_sprite.scale = 3.0;
            button_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
            button_sprite.pos.0 = buttons_width;
            buttons_width += button_sprite.get_size().0 + gfx::SPACING;
            max_button_height = max_button_height.max(button_sprite.get_size().1);

            buttons.push(Box::new(button_sprite));
        }
        let mut button_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);
        button_container.sub_elemnts = buttons;
        button_container.rect.size = gfx::FloatSize(buttons_width, max_button_height);
        button_container.rect.pos.1 = -gfx::SPACING;

        let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();
        let mut title_lines: Vec<Box<dyn BaseUiElement>> = Vec::new();
        let mut curr_y = 0.0;
        for line in text_lines_vec {
            let mut sprite = gfx::Sprite::new();
            sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(line, Some(200))));
            sprite.scale = 3.0;
            sprite.orientation = if align_left { gfx::TOP_LEFT } else { gfx::TOP };
            sprite.pos.1 = curr_y;
            curr_y += sprite.get_size().1 + gfx::SPACING;
            title_lines.push(Box::new(sprite));
        }
        let mut title_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(600.0, 0.0), gfx::Orientation { x: 0.5, y: 0.3 }, None);
        title_container.sub_elemnts = title_lines;

        Self {
            title_container,
            button_container,
            esc_choice,
            enter_choice,
            align_left,
            button_press,
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

    fn on_event_inner(&mut self, _: &mut gfx::GraphicsContext, event: &gfx::Event, _: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::Escape => {
                    if let Some(choice) = &self.esc_choice {
                        *self.button_press.borrow_mut() = Some(*choice);
                        return true;
                    }
                }
                gfx::Key::Enter => {
                    if let Some(choice) = &self.enter_choice {
                        *self.button_press.borrow_mut() = Some(*choice);
                        return true;
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
}
