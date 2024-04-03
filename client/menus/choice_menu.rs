use crate::libraries::graphics as gfx;

use gfx::BaseUiElement;

pub struct ChoiceMenu {
    title_container: gfx::Container,
    button_container: gfx::Container,
    esc_choice: Option<usize>,
    enter_choice: Option<usize>,
    pub button_press: Option<usize>,
}

impl ChoiceMenu {
    pub fn new(menu_title: &str, graphics: &gfx::GraphicsContext, button_texts: &[&str], esc_choice: Option<usize>, enter_choice: Option<usize>) -> Self {
        let mut buttons: Vec<Box<dyn BaseUiElement>> = Vec::new();
        let mut buttons_width = 0.0;
        let mut max_button_height: f32 = 0.0;
        for text in button_texts {
            let mut button_sprite = gfx::Button::new(|| {});
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
            sprite.orientation = gfx::TOP;
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
            button_press: None,
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

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::Escape => {
                    self.button_press = self.esc_choice;
                    return true;
                }
                gfx::Key::Enter => {
                    self.button_press = self.enter_choice;
                    return true;
                }
                gfx::Key::MouseLeft => {
                    let container = gfx::Container::new(
                        graphics,
                        self.button_container.rect.pos,
                        self.button_container.rect.size,
                        self.button_container.orientation,
                        Some(parent_container),
                    );
                    for (index, button) in self.button_container.sub_elemnts.iter_mut().enumerate() {
                        if button.on_event_inner(graphics, event, &container) {
                            self.button_press = Some(index);
                        }
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
