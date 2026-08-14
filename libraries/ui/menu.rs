use crate::libraries::graphics as gfx;
use crate::libraries::ui::{BaseUiElement, Container, UiContext, UiElement};

/// A screen that can ask to be closed and can push a successor.
///
/// The `String` a menu is paired with is a name for it, so an owner can tell which screen is
/// on top without downcasting.
pub trait Menu: UiElement + BaseUiElement {
    #[must_use]
    fn should_close(&mut self) -> bool;
    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)>;
    fn on_focus(&mut self, _: &gfx::GraphicsContext) {}
}

/// A stack of screens where only the top one is live.
///
/// The top menu gets the events, the updates and the rendering; the ones under it are held
/// but not driven. `open_menu` returning `Some` pushes, `should_close` pops, and whatever is
/// revealed by a pop is told it has focus again.
///
/// `MenuStack` is itself a `Menu`, so a stack nests inside a stack.
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

    #[must_use]
    pub fn get_top_menu(&self) -> Option<&(Box<dyn Menu>, String)> {
        self.stack.last()
    }
}

impl UiElement for MenuStack {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        self.stack.last_mut().map_or_else(Vec::new, |element| element.0.get_sub_elements_mut())
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        self.stack.last().map_or_else(Vec::new, |element| element.0.get_sub_elements())
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &Container) {
        if let Some(element) = self.stack.last_mut() {
            element.0.render_inner(graphics, parent_container);
        }
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &Container) {
        if let Some(top_menu) = self.stack.last_mut() {
            if let Some(mut new_menu) = top_menu.0.open_menu(graphics) {
                new_menu.0.update_inner(graphics, parent_container);
                self.stack.push(new_menu);
            }
        }

        let close = self.stack.last_mut().is_some_and(|top_menu| {
            top_menu.0.update_inner(graphics, parent_container);
            top_menu.0.should_close()
        });
        if close {
            self.stack.pop();
            if let Some(top_menu) = self.stack.last_mut() {
                top_menu.0.on_focus(graphics);
            }
        }
    }

    fn on_event_inner(&mut self, graphics: &mut dyn UiContext, event: &gfx::Event, parent_container: &Container) -> bool {
        if let Some(element) = self.stack.last_mut() {
            return element.0.on_event_inner(graphics, event, parent_container);
        }
        false
    }

    fn get_container(&self, graphics: &dyn UiContext, parent_container: &Container) -> Container {
        self.stack.last().map_or_else(
            || {
                Container::new(
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
}

impl Menu for MenuStack {
    fn should_close(&mut self) -> bool {
        self.stack.is_empty()
    }

    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }
}
