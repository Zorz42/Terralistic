mod color;
pub use color::Color;

mod transformation;
pub use transformation::Transformation;

mod surface;
pub use surface::Surface;

mod glfw_abstraction;

pub fn init(window_width: i32, window_height: i32, window_title: String) {
    glfw_abstraction::init_glfw(window_width, window_height, window_title);
}