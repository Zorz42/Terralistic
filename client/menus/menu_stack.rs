use super::Menu;
use crate::libraries::graphics as gfx;
use gfx::UiElement;

pub struct MenuStack {
    stack: Vec<(Box<dyn Menu>, String)>,
}

impl MenuStack {
    #[must_use]
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn add_menu(&mut self, menu: (Box<dyn Menu>, String)) {
        self.stack.push(menu);
    }

    pub fn get_top_menu(&self) -> Option<&(Box<dyn Menu>, String)> {
        self.stack.last()
    }
}

impl UiElement for MenuStack {
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        self.stack.last().map_or_else(
            || {
                gfx::Container::new(
                    graphics,
                    parent_container.get_absolute_rect().pos,
                    parent_container.get_absolute_rect().size,
                    parent_container.orientation,
                    None,
                )
            },
            |element| element.0.get_container(graphics, parent_container),
        )
    }

    fn get_sub_elements(&self) -> Vec<&dyn gfx::BaseUiElement> {
        self.stack.last().map_or_else(Vec::new, |element| element.0.get_sub_elements())
    }

    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn gfx::BaseUiElement> {
        self.stack.last_mut().map_or_else(Vec::new, |element| element.0.get_sub_elements_mut())
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        if let Some(top_menu) = self.stack.last_mut() {
            if let Some(mut new_menu) = top_menu.0.open_menu(graphics) {
                new_menu.0.update_inner(graphics, parent_container);
                self.stack.push(new_menu);
            }
        }

        let mut close = false;
        if let Some(top_menu) = self.stack.last_mut() {
            top_menu.0.update_inner(graphics, parent_container);
            close = top_menu.0.should_close();
        }
        if close {
            self.stack.pop();
            if let Some(top_menu) = self.stack.last_mut() {
                top_menu.0.on_focus(graphics);
            }
        }
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        if let Some(element) = self.stack.last_mut() {
            element.0.render_inner(graphics, parent_container);
        }
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let Some(element) = self.stack.last_mut() {
            return element.0.on_event_inner(graphics, event, parent_container);
        }
        false
    }
}

impl Menu for MenuStack {
    fn should_close(&mut self) -> bool {
        self.stack.is_empty()
    }

    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }
}
