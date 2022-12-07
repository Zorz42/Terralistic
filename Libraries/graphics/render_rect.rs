use crate::{Color, GraphicsContext};
use crate::Container;
use crate::Orientation;
use crate::Rect;
use crate::Renderer;

/*
The struct RenderRect contains a container and
moves smoothly visually to the saved position
it has a smooth_factor. At every render the position
of the container is changed by the distance to the
target position divided by the smooth_factor. It is 1 by default.
 */
pub struct RenderRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    render_x: f32,
    render_y: f32,
    render_w: f32,
    render_h: f32,
    pub fill_color: Color,
    pub border_color: Color,
    pub smooth_factor: f32,
    pub orientation: Orientation,
}

impl RenderRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, fill_color: Color, border_color: Color, orientation: Orientation) -> Self {
        RenderRect {
            x,
            y,
            w,
            h,
            render_x: x,
            render_y: y,
            render_w: w,
            render_h: h,
            fill_color,
            border_color,
            smooth_factor: 1.0,
            orientation,
        }
    }

    /*
    This function renders the rectangle, it uses Rect class to render.
    It also approcaches the position to the target position.
     */
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        self.render_x += (self.x - self.render_x) / self.smooth_factor;
        self.render_y += (self.y - self.render_y) / self.smooth_factor;
        self.render_w += (self.w - self.render_w) / self.smooth_factor;
        self.render_h += (self.h - self.render_h) / self.smooth_factor;

        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        rect.render(graphics, self.fill_color);
    }

    /*
    This function returns the container of the rectangle.
    The container has the position of render rect.
     */
    pub fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.render_x as i32, self.render_y as i32, self.render_w as i32, self.render_h as i32, self.orientation);
        container.update(graphics, parent_container);
        container
    }

    /*
    This function jumps the rectangle to the target position.
     */
    pub fn jump_to_target(&mut self) {
        self.render_x = self.x;
        self.render_y = self.y;
        self.render_w = self.w;
        self.render_h = self.h;
    }
}