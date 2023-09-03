use crate::libraries::graphics::{Container, GraphicsContext};

/// `BackgroundRect` is a trait that defines a background which
/// also has a rectangle. You can set the rectangle's width and
/// retrieve the rectangle's width. The height is always the same
/// as the height of the window. You can also retrieve the back rectangle's
/// container.
pub trait BackgroundRect {
    fn render_back(&mut self, graphics: &mut GraphicsContext);
    fn set_back_rect_width(&mut self, width: f32);
    fn get_back_rect_width(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> f32;
    fn get_back_rect_container(&self) -> &Container;
}
