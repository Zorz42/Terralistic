use super::Container;
use super::Orientation;
use crate::libraries::graphics as gfx;

/// The struct `RenderRect` contains a container and
/// moves smoothly visually to the saved position
/// it has a `smooth_factor`. At every render the position
/// of the container is changed by the distance to the
/// target position divided by the `smooth_factor`. It is 1 by default.
pub struct RenderRect {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
    pub render_pos: gfx::FloatPos,
    pub render_size: gfx::FloatSize,
    pub fill_color: gfx::Color,
    pub border_color: gfx::Color,
    pub smooth_factor: f32,
    pub orientation: Orientation,
    pub blur_radius: i32,
    pub shadow_intensity: i32,
    ms_counter: u32,
    approach_timer: std::time::Instant,
}

impl RenderRect {
    #[must_use]
    pub fn new(pos: gfx::FloatPos, size: gfx::FloatSize) -> Self {
        Self {
            pos,
            size,
            render_pos: pos,
            render_size: size,
            fill_color: gfx::Color::new(0, 0, 0, 255),
            border_color: gfx::Color::new(0, 0, 0, 0),
            smooth_factor: 1.0,
            orientation: gfx::TOP_LEFT,
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

    /// This function renders the rectangle, it uses Rect class to render.
    /// It also approaches the position to the target position.
    pub fn render(&mut self, graphics: &gfx::GraphicsContext, parent_container: Option<&Container>) {
        while self.ms_counter < self.approach_timer.elapsed().as_millis() as u32 {
            self.ms_counter += 1;
            self.render_pos.0 = Self::approach(self.render_pos.0, self.pos.0, self.smooth_factor);
            self.render_pos.1 = Self::approach(self.render_pos.1, self.pos.1, self.smooth_factor);
            self.render_size.0 = Self::approach(self.render_size.0, self.size.0, self.smooth_factor);
            self.render_size.1 = Self::approach(self.render_size.1, self.size.1, self.smooth_factor);
        }

        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        graphics.renderer.blur_rect(*rect, self.blur_radius);
        graphics.renderer.shadow_context.render(
            graphics,
            rect,
            self.shadow_intensity as f32 / 255.0,
        );

        rect.render(graphics, self.fill_color);
    }

    /// This function returns the container of the rectangle.
    /// The container has the position of render rect.
    #[must_use]
    pub fn get_container(
        &self,
        graphics: &gfx::GraphicsContext,
        parent_container: Option<&Container>,
    ) -> Container {
        Container::new(
            graphics,
            self.render_pos,
            self.render_size,
            self.orientation,
            parent_container,
        )
    }

    /// This function jumps the rectangle to the target position.
    pub fn jump_to_target(&mut self) {
        self.render_pos = self.pos;
        self.render_size = self.size;
    }
}
