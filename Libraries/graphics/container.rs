use crate::{GraphicsContext, Rect};

pub struct Orientation {
    pub x: f32,
    pub y: f32,
}

pub const TOP_LEFT: Orientation =     Orientation{x: 0.0 , y: 0.0};
pub const TOP: Orientation =          Orientation{x: 0.5 , y: 0.0};
pub const TOP_RIGHT: Orientation =    Orientation{x: 1.0 , y: 0.0};
pub const LEFT: Orientation =         Orientation{x: 0.0 , y: 0.5};
pub const CENTER: Orientation =       Orientation{x: 0.5 , y: 0.5};
pub const RIGHT: Orientation =        Orientation{x: 1.0 , y: 0.5};
pub const BOTTOM_LEFT: Orientation =  Orientation{x: 0.0 , y: 1.0};
pub const BOTTOM: Orientation =       Orientation{x: 0.5 , y: 1.0};
pub const BOTTOM_RIGHT: Orientation = Orientation{x: 1.0 , y: 1.0};

/*
This struct has coodinates and size of a rectangle.
Is has orientation and parent container. It also has
a function to get absolute value of the rectangle.
*/
pub struct Container {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub orientation: Orientation,
    pub parent: Option<Box<Container>>,
}

impl Container {
    /*
    Creates a new container.
    */
    pub fn new(x: i32, y: i32, w: i32, h: i32, orientation: Orientation, parent: Option<Box<Container>>) -> Self {
        Container{
            x,
            y,
            w,
            h,
            orientation,
            parent,
        }
    }

    /*
    Returns the absolute rectangle of the container. Orientation
    is a percentage the container is offset from the top left corner
    of the parent container. If parent is None, the parent is the
    window. This function needs graphics_context to get the window size.
    */
    pub fn get_absolute_rect(&self, graphics_context: &mut GraphicsContext) -> Rect {
        let mut x = self.x;
        let mut y = self.y;
        let mut w = self.w;
        let mut h = self.h;

        if let Some(parent) = &self.parent {
            let parent_rect = parent.get_absolute_rect(graphics_context);
            x += (parent_rect.w as f32 * self.orientation.x) as i32;
            y += (parent_rect.h as f32 * self.orientation.y) as i32;
            w += (parent_rect.w as f32 * self.orientation.x) as i32;
            h += (parent_rect.h as f32 * self.orientation.y) as i32;
        } else {
            x += (graphics_context.renderer.glfw_window.get_size().0 as f32 * self.orientation.x) as i32;
            y += (graphics_context.renderer.glfw_window.get_size().1 as f32 * self.orientation.y) as i32;
            w += (graphics_context.renderer.glfw_window.get_size().0 as f32 * self.orientation.x) as i32;
            h += (graphics_context.renderer.glfw_window.get_size().1 as f32 * self.orientation.y) as i32;
        }

        Rect::new(x, y, w, h)
    }
}