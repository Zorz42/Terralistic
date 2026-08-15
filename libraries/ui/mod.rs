//! An immediate-mode-ish widget toolkit: layout, hit testing, input routing and widgets.
//!
//! What a widget *is* is the `UiElement` / `BaseUiElement` pair in `ui_element.rs` - implement
//! `UiElement`, whose only required method is `get_container`, and call `BaseUiElement`, which is
//! blanket implemented and recurses into children.
//!
//! **The line this library draws** is `UiContext`: the whole non-rendering surface a widget
//! needs, so layout and input work with no GPU. `HeadlessContext` is a `#[cfg(test)]` struct
//! whose inputs are plain fields, and it makes hit testing and event routing testable in CI.
//!
//! Drawing is the one edge back into `graphics`: `render_inner` and `update_inner` take a
//! `&mut GraphicsContext` because a few widgets genuinely build textures. **Keep the two
//! apart** - a widget that animates while rendering freezes for any caller that lays a list
//! out without drawing it.
//!
//! **Not in scope**: pixels, and what the widgets are *for*. A list of worlds is a `ListPage`
//! of rows, and what a row means belongs to whoever built it.

pub use button::Button;
pub use container::{Container, Orientation, BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTER, LEFT, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};
pub use dock::{area_at_path, DockArea, DockNode, DockSplit, SplitType};
pub use interpolate::approach;
pub use list_page::{ListPage, ListRow};
pub use menu::{Menu, MenuStack};
pub use render_rect::RenderRect;
pub use scrollable::Scrollable;
pub use sprite::Sprite;
pub use text_input::TextInput;
pub use theme::{BLACK, BLUR, BORDER_COLOR, DARK_GREY, GREY, LIGHT_GREY, SHADOW_INTENSITY, SPACING, TEXT_INPUT_WIDTH, TRANSPARENCY, TRANSPARENT, WHITE};
pub use toggle::Toggle;
#[cfg(test)]
pub use ui_context::HeadlessContext;
pub use ui_context::UiContext;
pub use ui_element::{BaseUiElement, ClickTracker, UiElement};

mod button;
mod container;
mod dock;
mod interpolate;
mod list_page;
mod menu;
mod render_rect;
mod scrollable;
mod sprite;
mod tests;
mod text_input;
mod theme;
mod toggle;
mod ui_context;
mod ui_element;
