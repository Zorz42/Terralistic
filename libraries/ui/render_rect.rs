use crate::libraries::graphics as gfx;
use crate::libraries::timing;

/// A rectangle that slides towards its target instead of jumping to it.
///
/// Every ready frame, `render_pos` and `render_size` move a `1 / smooth_factor` fraction of
/// the way to `pos` and `size`. It can also draw a border, a shadow and a blur.
#[derive(Debug)]
pub struct RenderRect {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
    pub render_pos: gfx::FloatPos,
    pub render_size: gfx::FloatSize,
    pub fill_color: gfx::Color,
    pub border_color: gfx::Color,
    pub smooth_factor: f32,
    pub orientation: super::Orientation,
    pub blur_radius: i32,
    pub shadow_intensity: i32,
    animation_timer: timing::FixedStep,
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
            orientation: super::TOP_LEFT,
            blur_radius: 0,
            shadow_intensity: 0,
            animation_timer: timing::FixedStep::for_animation(1),
        }
    }

    /// Skips the animation and puts the rectangle where it is headed.
    pub const fn jump_to_target(&mut self) {
        self.render_pos = self.pos;
        self.render_size = self.size;
    }
}

impl super::UiElement for RenderRect {
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &super::Container) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        // Both are off for most rects, and the shadow is nine commands or more, so skipping
        // is worth more than letting them record nothing.
        if self.blur_radius > 0 {
            graphics.blur_rect(*rect, self.blur_radius);
        }
        if self.shadow_intensity > 0 {
            graphics.shadow_context.render(graphics, rect, self.shadow_intensity as f32 / 255.0);
        }

        rect.render(graphics, self.fill_color);
        rect.render_outline(graphics, self.border_color);
    }

    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &super::Container) {
        while self.animation_timer.step() {
            self.render_pos.0 = super::approach(self.render_pos.0, self.pos.0, self.smooth_factor, 0.01);
            self.render_pos.1 = super::approach(self.render_pos.1, self.pos.1, self.smooth_factor, 0.01);
            self.render_size.0 = super::approach(self.render_size.0, self.size.0, self.smooth_factor, 0.01);
            self.render_size.1 = super::approach(self.render_size.1, self.size.1, self.smooth_factor, 0.01);
        }
    }

    /// Built from `render_pos`, not `pos`, which is what makes the rectangle appear to slide.
    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> super::Container {
        super::Container::new(graphics, self.render_pos, self.render_size, self.orientation, Some(parent_container))
    }
}
