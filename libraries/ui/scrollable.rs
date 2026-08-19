use super::UiElement;
use crate::libraries::graphics as gfx;
use crate::libraries::timing;

/// How far past either end the list may be pushed. Scrolling that leaves the bounds is scaled
/// by how much of this is already used up, so the band stiffens the further out it gets and a
/// hard swipe cannot throw the list a screen past its end. An ordinary scroll never reaches far
/// enough to notice it.
const OVERSCROLL_LIMIT: f32 = 200.0;

/// A scroll position, which the world and server lists offset their rows by. It draws nothing
/// itself.
///
/// **The input is a distance, not a speed.** `scroll_target` is the sum of the scroll events,
/// so it tracks a trackpad finger exactly, and `scroll_pos` follows it. It used to be a
/// velocity the events raised and a decay that spent it, which is momentum - and a macOS
/// trackpad already sends its own, as a stream of pixel deltas that carries on after the finger
/// lifts. Two momenta over one gesture is the jitter: every event in the stream restarted a
/// glide the last one was still running, and out of bounds each restart shoved the list back
/// out of a boundary it was in the middle of returning to.
pub struct Scrollable {
    pub rect: gfx::Rect,
    pub orientation: super::Orientation,
    /// Where the scroll has been asked to be, before smoothing. Outside the bounds while the
    /// band is stretched, and pulled back onto them once nothing is pushing.
    scroll_target: f32,
    scroll_pos: f32,
    pub scroll_size: f32,
    animation_timer: timing::FixedStep,
    /// How closely the drawn position follows the target. Small: this is the smoothing that
    /// turns a wheel detent into a glide, not a glide of its own.
    pub scroll_smooth_factor: f32,
    /// How quickly a stretched band returns to its end.
    pub boundary_smooth_factor: f32,
}

impl Scrollable {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            orientation: super::TOP_LEFT,
            scroll_target: 0.0,
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

    /// How far the list can be scrolled before it runs out of rows. Zero when they all fit.
    fn upper_bound(&self) -> f32 {
        f32::max(self.scroll_size - self.rect.size.1, 0.0)
    }

    /// How far the target is currently past either end, or zero inside them.
    fn overscroll(&self) -> f32 {
        f32::max(-self.scroll_target, self.scroll_target - self.upper_bound()).max(0.0)
    }

    /// One frame of scrolling: a stretched band returns to its end, and the drawn position
    /// follows the target. Both are `super::approach`, whose epsilon is what makes them *land*
    /// rather than leave a scrolled list a fraction of a pixel short forever. Split out of
    /// `update_inner`, which needs a `GraphicsContext` and this does not.
    pub(super) fn advance_frame(&mut self) {
        let upper_bound = self.upper_bound();
        if self.scroll_target < 0.0 {
            self.scroll_target = super::approach(self.scroll_target, 0.0, self.boundary_smooth_factor, 0.01);
        } else if self.scroll_target > upper_bound {
            self.scroll_target = super::approach(self.scroll_target, upper_bound, self.boundary_smooth_factor, 0.01);
        }

        self.scroll_pos = super::approach(self.scroll_pos, self.scroll_target, self.scroll_smooth_factor, 0.01);
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

    /// Adds the scrolled distance to the target, resisted while it leaves the bounds. Coming
    /// back is never resisted - a list on its way home is not fighting anything.
    fn on_event_inner(&mut self, _: &mut dyn super::UiContext, event: &gfx::Event, _: &super::Container) -> bool {
        if let gfx::Event::MouseScroll(pixels) = event {
            let mut delta = -*pixels;
            if (self.scroll_target < 0.0 && delta < 0.0) || (self.scroll_target > self.upper_bound() && delta > 0.0) {
                delta *= 1.0 - (self.overscroll() / OVERSCROLL_LIMIT).clamp(0.0, 1.0);
            }
            self.scroll_target += delta;
        }
        false
    }

    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> super::Container {
        super::Container::new(graphics, self.rect.pos, self.rect.size, self.orientation, Some(parent_container))
    }
}
