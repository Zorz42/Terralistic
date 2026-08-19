use super::UiElement;
use crate::libraries::graphics as gfx;
use crate::libraries::timing;

/// How far past either end the list may be pushed. Movement leaving the bounds is scaled by how
/// much of this is already used up, so a flick is compressed the further out it gets rather than
/// running on: 377 pixels of overshoot for a hard trackpad swipe becomes 150. It only binds on
/// the hard ones - an ordinary notch bounces 5 pixels and never notices it.
const OVERSCROLL_LIMIT: f32 = 200.0;

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

    /// One frame of scrolling: velocity moves the position, compressed while it is leaving the
    /// bounds; outside them the flick is spent into the boundary and the position pulled back
    /// onto it; then the velocity decays. Every pull is `super::approach`, whose epsilon is what
    /// makes them *land* rather than leave a flicked list a fraction of a pixel past its end
    /// forever. Split out of `update_inner`, which needs a `GraphicsContext` and this does not.
    ///
    /// **Outside the bounds the flick belongs to the boundary, not to the momentum**, which is
    /// why the velocity decays at `boundary_smooth_factor` there rather than the scroll's own.
    /// Left on the scroll's, a bounce is fed by momentum for as long as that lasts: the pull
    /// brings the list to the edge, the momentum pushes it back out, and what should be one
    /// bounce becomes a slow crawl home three times as long as the pull alone.
    pub(super) fn advance_frame(&mut self) {
        let upper_bound = f32::max(self.scroll_size - self.rect.size.1, 0.0);
        let overscroll = f32::max(-self.scroll_pos, self.scroll_pos - upper_bound).max(0.0);
        let resistance = if self.is_leaving_bounds(upper_bound) {
            1.0 - (overscroll / OVERSCROLL_LIMIT).clamp(0.0, 1.0)
        } else {
            1.0
        };

        self.scroll_pos += self.scroll_velocity * resistance;

        if overscroll > 0.0 {
            self.scroll_velocity = super::approach(self.scroll_velocity, 0.0, self.boundary_smooth_factor, 0.01);
        }
        if self.scroll_pos < 0.0 {
            self.scroll_pos = super::approach(self.scroll_pos, 0.0, self.boundary_smooth_factor, 0.01);
        } else if self.scroll_pos > upper_bound {
            self.scroll_pos = super::approach(self.scroll_pos, upper_bound, self.boundary_smooth_factor, 0.01);
        }

        self.scroll_velocity = super::approach(self.scroll_velocity, 0.0, self.scroll_smooth_factor, 0.01);
    }

    /// Whether the velocity is carrying the list further out of bounds, as opposed to back in.
    /// Only the former is compressed - a list on its way home is not fighting anything.
    fn is_leaving_bounds(&self, upper_bound: f32) -> bool {
        (self.scroll_pos < 0.0 && self.scroll_velocity < 0.0) || (self.scroll_pos > upper_bound && self.scroll_velocity > 0.0)
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
