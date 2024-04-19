use crate::gfx::{BaseUiElement, UiElement};

pub trait Menu: UiElement + BaseUiElement {
    fn should_close(&mut self) -> bool;
}
