use super::UiElement;
use crate::libraries::graphics as gfx;
use crate::libraries::timing;

/// A scroll position with momentum, which the world and server lists offset their rows by.
/// It draws nothing itself.
pub struct Scrollable {
    pub rect: gfx::Rect,
    pub orientation: super::Orientation,
    scroll_velocity: f32,
    scroll_pos: f32,
    pub scroll_size: f32,
    animation_timer: timing::FixedStep,
    pub scroll_smooth_factor: f32,
    pub boundary_smooth_factor: f32,
}

impl Scrollable {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            orientation: super::TOP_LEFT,
            scroll_velocity: 0.0,
            scroll_pos: 0.0,
            scroll_size: 0.0,
            animation_timer: timing::FixedStep::for_animation(1),
            scroll_smooth_factor: 1.0,
            boundary_smooth_factor: 1.0,
        }
    }

    /// Where a list should start drawing: the scrollable's offset, less how far it is scrolled.
    ///
    /// **Vertical, and so is everything else here**: `scroll_pos` is bounded against
    /// `rect.size.1` and both callers add the result to a y coordinate. Adding a horizontal one
    /// means adding the axis, not reinterpreting this.
    #[must_use]
    pub const fn get_scroll_y(&self) -> f32 {
        self.rect.pos.1 - self.scroll_pos
    }

    #[must_use]
    pub const fn get_scroll_pos(&self) -> f32 {
        self.scroll_pos
    }

    /// One frame of scrolling: velocity moves the position, the position is pulled back inside
    /// its bounds, and the velocity decays. Both pulls are `super::approach`, whose epsilon is
    /// what makes them *land* rather than leave a flicked list a fraction of a pixel past its
    /// end forever. Split out of `update_inner`, which needs a `GraphicsContext` and this does
    /// not.
    pub(super) fn advance_frame(&mut self) {
        self.scroll_pos += self.scroll_velocity;

        let upper_bound = f32::max(self.scroll_size - self.rect.size.1, 0.0);
        if self.scroll_pos < 0.0 {
            self.scroll_pos = super::approach(self.scroll_pos, 0.0, self.boundary_smooth_factor, 0.01);
        } else if self.scroll_pos > upper_bound {
            self.scroll_pos = super::approach(self.scroll_pos, upper_bound, self.boundary_smooth_factor, 0.01);
        }

        self.scroll_velocity = super::approach(self.scroll_velocity, 0.0, self.scroll_smooth_factor, 0.01);
    }
}

impl UiElement for Scrollable {
    /// Advances the scroll, and nothing else. **`update_inner`, not `render_inner`**: stepping
    /// while rendering froze the scroll for any caller that lays the list out without drawing
    /// it - a menu sliding offscreen. The parent reads `get_scroll_y` from its own
    /// `update_inner`, which runs first, so it sees the previous frame's position.
    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &super::Container) {
        while self.animation_timer.step() {
            self.advance_frame();
        }
    }

    fn on_event_inner(&mut self, _: &mut dyn super::UiContext, event: &gfx::Event, _: &super::Container) -> bool {
        if let gfx::Event::MouseScroll(delta) = event {
            let delta = -*delta * 0.8;
            if delta > 0.0 {
                self.scroll_velocity = f32::max(self.scroll_velocity, delta);
            } else if delta < 0.0 {
                self.scroll_velocity = f32::min(self.scroll_velocity, delta);
            }
        }
        false
    }

    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> super::Container {
        super::Container::new(graphics, self.rect.pos, self.rect.size, self.orientation, Some(parent_container))
    }
}
