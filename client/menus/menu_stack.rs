use super::Menu;
use crate::libraries::graphics as gfx;
use gfx::UiElement;

pub struct MenuStack {
    stack: Vec<Box<dyn Menu>>,
}

impl MenuStack {
    #[must_use]
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn add_menu(&mut self, menu: Box<dyn Menu>) {
        self.stack.push(menu);
    }
}

impl UiElement for MenuStack {
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        if let Some(element) = self.stack.last() {
            element.get_container(graphics, parent_container)
        } else {
            gfx::Container::new(
                graphics,
                parent_container.get_absolute_rect().pos,
                parent_container.get_absolute_rect().size,
                parent_container.orientation,
                None,
            )
        }
    }

    fn get_sub_elements(&self) -> Vec<&dyn gfx::BaseUiElement> {
        if let Some(element) = self.stack.last() {
            element.get_sub_elements()
        } else {
            vec![]
        }
    }

    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn gfx::BaseUiElement> {
        if let Some(element) = self.stack.last_mut() {
            element.get_sub_elements_mut()
        } else {
            vec![]
        }
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let mut close = false;
        if let Some(top_menu) = self.stack.last_mut() {
            top_menu.update_inner(graphics, parent_container);
            close = top_menu.should_close();
            if let Some(mut new_menu) = top_menu.open_menu() {
                new_menu.update_inner(graphics, parent_container);
                self.stack.push(new_menu);
            }
        }
        if close {
            self.stack.pop();
            if let Some(top_menu) = self.stack.last_mut() {
                top_menu.on_focus(graphics);
            }
        }
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        if let Some(element) = self.stack.last_mut() {
            element.render_inner(graphics, parent_container);
        }
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let Some(element) = self.stack.last_mut() {
            return element.on_event_inner(graphics, event, parent_container);
        }
        false
    }
}

impl Menu for MenuStack {
    fn should_close(&mut self) -> bool {
        self.stack.is_empty()
    }

    fn open_menu(&mut self) -> Option<Box<dyn Menu>> {
        None
    }
}
