use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};

pub trait Menu: UiElement + BaseUiElement {
    #[must_use]
    fn should_close(&mut self) -> bool;
    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)>;
    fn on_focus(&mut self, _: &gfx::GraphicsContext) {}
}
