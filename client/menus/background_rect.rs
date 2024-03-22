use crate::libraries::graphics as gfx;
use gfx::BaseUiElement;

/// `BackgroundRect` is a trait that defines a background which
/// also has a rectangle. You can set the rectangle's width and
/// retrieve the rectangle's width. The height is always the same
/// as the height of the window. You can also retrieve the back rectangle's
/// container.
pub trait BackgroundRect: BaseUiElement {
    fn render_back(&mut self, graphics: &mut gfx::GraphicsContext);
    fn set_back_rect_width(&mut self, width: f32, instant: bool);
    fn get_back_rect_width(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> f32;
    fn get_back_rect_container(&self, graphics: &mut gfx::GraphicsContext) -> gfx::Container;
    fn set_x_position(&mut self, center_pos: f32);
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement>;
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement>;
    fn get_elements_vec_mut(&mut self) -> &mut Vec<Box<dyn BaseUiElement>>;
}
