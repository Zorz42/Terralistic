use super::UiElement;
use crate::libraries::graphics as gfx;
use crate::libraries::timing;

/// How far the band can stretch past either end, however hard the list is pushed.
///
/// The stretch is `LIMIT * push / (LIMIT + push)`: it follows the gesture pixel for pixel at
/// first and stiffens towards this, so a hard fling cannot throw the list a screen past its
/// end. The push itself is capped at three times it - beyond that the stretch is within a few
/// pixels of the limit anyway, and an uncapped push would have to be scrolled all the way back
/// before the list moved again.
const OVERSCROLL_LIMIT: f32 = 100.0;

/// How long after a scroll event the gesture counts as finished.
///
/// A trackpad delivers one gesture as a stream of deltas a frame apart, and goes on delivering
/// it after the finger lifts. The band must not start returning between two events of the same
/// push: the return runs every millisecond and the push arrives every frame, so the two fight
/// at frame frequency and the list shakes. Held off, the stretch during a gesture is a plain
/// function of how far it has pushed, and the return is a movement of its own afterwards.
const GESTURE_GAP_MS: u32 = 40;

/// A scroll position, which the world and server lists offset their rows by. It draws nothing
/// itself.
///
/// **The input is a distance, not a speed.** `scroll_push` is the sum of the scroll events, so
/// it tracks a trackpad finger exactly, and `scroll_pos` follows where that lands. It used to be
/// a velocity the events raised and a decay that spent it, which is momentum - and a macOS
/// trackpad already sends its own, as a stream of pixel deltas that carries on after the finger
/// lifts. Two momenta over one gesture is the jitter: every event in the stream restarted a
/// glide the last one was still running.
pub struct Scrollable {
    pub rect: gfx::Rect,
    pub orientation: super::Orientation,
    /// Everything the gesture has asked for, unbounded. What lies past an end is the band's
    /// stretch, and it returns to the end once nothing is pushing.
    scroll_push: f32,
    scroll_pos: f32,
    /// Milliseconds since the last scroll event, so a gesture still arriving can be told from
    /// one that is over. Saturates, and only `GESTURE_GAP_MS` of it is ever read.
    ms_since_scroll: u32,
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
            scroll_push: 0.0,
            scroll_pos: 0.0,
            ms_since_scroll: 0,
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

    /// The push with the ends applied: the part inside them as it is, and whatever lies past
    /// one compressed into the band's stretch. A pure function of the push, which is what keeps
    /// a gesture running past an end smooth - nothing is resisting it frame by frame.
    fn stretched_position(&self) -> f32 {
        let bounded = self.scroll_push.clamp(0.0, self.upper_bound());
        let push_past_end = self.scroll_push - bounded;
        bounded + push_past_end * OVERSCROLL_LIMIT / (OVERSCROLL_LIMIT + push_past_end.abs())
    }

    /// One frame of scrolling: once the gesture is over a stretched band returns to its end, and
    /// the drawn position follows where the push lands. Both are `super::approach`, whose epsilon
    /// is what makes them *land* rather than leave a scrolled list a fraction of a pixel short
    /// forever. Split out of `update_inner`, which needs a `GraphicsContext` and this does not.
    pub(super) fn advance_frame(&mut self) {
        self.ms_since_scroll = self.ms_since_scroll.saturating_add(1);
        if self.ms_since_scroll > GESTURE_GAP_MS {
            let bounded = self.scroll_push.clamp(0.0, self.upper_bound());
            self.scroll_push = super::approach(self.scroll_push, bounded, self.boundary_smooth_factor, 0.01);
        }

        self.scroll_pos = super::approach(self.scroll_pos, self.stretched_position(), self.scroll_smooth_factor, 0.01);
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

    /// Adds the scrolled distance to the push. The ends are `stretched_position`'s business, so
    /// there is nothing to resist here - only a cap on how far past one a gesture can push.
    fn on_event_inner(&mut self, _: &mut dyn super::UiContext, event: &gfx::Event, _: &super::Container) -> bool {
        if let gfx::Event::MouseScroll(pixels) = event {
            let limit = 3.0 * OVERSCROLL_LIMIT;
            self.scroll_push = (self.scroll_push - *pixels).clamp(-limit, self.upper_bound() + limit);
            self.ms_since_scroll = 0;
        }
        false
    }

    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> super::Container {
        super::Container::new(graphics, self.rect.pos, self.rect.size, self.orientation, Some(parent_container))
    }
}
