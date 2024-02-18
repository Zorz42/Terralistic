use crate::libraries::graphics as gfx;

/// `BackgroundRect` is a trait that defines a background which
/// also has a rectangle. You can set the rectangle's width and
/// retrieve the rectangle's width. The height is always the same
/// as the height of the window. You can also retrieve the back rectangle's
/// container.
pub trait BackgroundRect {
    fn render_back(&mut self, graphics: &mut gfx::GraphicsContext);
    fn set_back_rect_width(&mut self, width: f32);
    fn get_back_rect_width(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> f32;
    fn get_back_rect_container(&self) -> &gfx::Container;
    fn set_x_position(&mut self, center_pos: f32);
}
