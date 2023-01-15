use crate::{Color, GraphicsContext, TOP_LEFT};
use crate::Container;
use crate::Orientation;

/**
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
    pub blur_radius: i32,
    pub shadow_intensity: i32,
    ms_counter: u32,
    approach_timer: std::time::Instant,
}

impl RenderRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        RenderRect {
            x,
            y,
            w,
            h,
            render_x: x,
            render_y: y,
            render_w: w,
            render_h: h,
            fill_color: Color::new(0, 0, 0, 255),
            border_color: Color::new(0, 0, 0, 0),
            smooth_factor: 1.0,
            orientation: TOP_LEFT,
            blur_radius: 0,
            shadow_intensity: 0,
            ms_counter: 0,
            approach_timer: std::time::Instant::now(),
        }
    }

    fn approach(position: f32, target: f32, smooth_factor: f32) -> f32 {
        if (target - position).abs() <= 0.01 {
            target
        } else {
            position + (target - position) / smooth_factor
        }
    }

    /**
    This function renders the rectangle, it uses Rect class to render.
    It also approaches the position to the target position.
     */
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        while self.ms_counter < self.approach_timer.elapsed().as_millis() as u32 {
            self.ms_counter += 1;
            self.render_x = Self::approach(self.render_x, self.x, self.smooth_factor);
            self.render_y = Self::approach(self.render_y, self.y, self.smooth_factor);
            self.render_w = Self::approach(self.render_w, self.w, self.smooth_factor);
            self.render_h = Self::approach(self.render_h, self.h, self.smooth_factor);
        }

        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        graphics.renderer.blur_rect(rect, self.blur_radius);
        graphics.renderer.shadow_context.render(graphics, rect, self.shadow_intensity as f32 / 255.0);

        rect.render(graphics, self.fill_color);
    }

    /**
    This function returns the container of the rectangle.
    The container has the position of render rect.
     */
    pub fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container {
        let mut container = Container::new(self.render_x as i32, self.render_y as i32, self.render_w as i32, self.render_h as i32, self.orientation);
        container.update(graphics, parent_container);
        container
    }

    /**
    This function jumps the rectangle to the target position.
     */
    pub fn jump_to_target(&mut self) {
        self.render_x = self.x;
        self.render_y = self.y;
        self.render_w = self.w;
        self.render_h = self.h;
    }
}