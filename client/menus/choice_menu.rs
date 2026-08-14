use crate::libraries::graphics as gfx;
use crate::libraries::ui;
use crate::libraries::ui::Menu;
use std::cell::Cell;
use std::rc::Rc;

use ui::BaseUiElement;

pub struct ChoiceMenu {
    title_container: ui::Container,
    button_container: ui::Container,
    buttons: Vec<ui::Button>,
    title_lines: Vec<ui::Sprite>,
    esc_choice: Option<usize>,
    enter_choice: Option<usize>,
    close: Rc<Cell<bool>>,
}

impl ChoiceMenu {
    pub fn new(menu_title: &str, graphics: &gfx::GraphicsContext, buttons_properties: Vec<(&str, Box<dyn Fn()>)>, esc_choice: Option<usize>, enter_choice: Option<usize>) -> Self {
        let mut buttons: Vec<ui::Button> = Vec::new();
        let mut buttons_width = 0.0;
        let mut max_button_height: f32 = 0.0;
        let close = Rc::new(Cell::new(false));
        for (text, function) in buttons_properties {
            let close_cloned = close.clone();
            let mut button_sprite = ui::Button::new(move || {
                function();
                close_cloned.set(true);
            });
            button_sprite.scale = 3.0;
            button_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
            button_sprite.pos.0 = buttons_width;
            buttons_width += button_sprite.get_size().0 + ui::SPACING;
            max_button_height = max_button_height.max(button_sprite.get_size().1);

            buttons.push(button_sprite);
        }
        let mut button_container = ui::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), ui::BOTTOM, None);
        button_container.rect.size = gfx::FloatSize(buttons_width, max_button_height);
        button_container.rect.pos.1 = -ui::SPACING;

        let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();
        let mut title_lines: Vec<ui::Sprite> = Vec::new();
        let mut curr_y = 0.0;
        for line in text_lines_vec {
            let mut sprite = ui::Sprite::new();
            sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(line, Some(200))));
            sprite.scale = 3.0;
            sprite.orientation = ui::TOP;
            sprite.pos.1 = curr_y;
            curr_y += sprite.get_size().1 + ui::SPACING;
            title_lines.push(sprite);
        }
        let title_container = ui::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(600.0, 0.0), ui::Orientation { x: 0.5, y: 0.3 }, None);

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

impl ui::UiElement for ChoiceMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.title_container, &mut self.button_container]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.title_container, &self.button_container]
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, _parent_container: &ui::Container) {
        for button in &mut self.buttons {
            button.render(graphics, &self.button_container);
        }
        for line in &mut self.title_lines {
            line.render(graphics, &self.title_container);
        }
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, _parent_container: &ui::Container) {
        for button in &mut self.buttons {
            button.update(graphics, &self.button_container);
        }
        for line in &mut self.title_lines {
            line.update(graphics, &self.title_container);
        }
    }

    fn on_event_inner(&mut self, graphics: &mut dyn ui::UiContext, event: &gfx::Event, _parent_container: &ui::Container) -> bool {
        // Every event, not only the release: a `ui::Button` fires on a release that completes
        // a press it saw land on itself, so handing it one half of a click does nothing.
        // The buttons are not sub-elements - they are positioned against `button_container`
        // rather than against this menu - so this loop is the only thing that reaches them.
        for button in &mut self.buttons {
            button.on_event(graphics, event, &self.button_container);
        }

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
                }
                _ => {}
            }
        }
        false
    }

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

impl Menu for ChoiceMenu {
    fn should_close(&mut self) -> bool {
        self.close.get()
    }

    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }
}
