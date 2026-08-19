use super::UiElement;
use crate::libraries::graphics as gfx;
use crate::libraries::timing;

/// How far the band can stretch past either end, however hard the list is pushed.
///
/// The stretch is `LIMIT * push / (LIMIT + push)`: it follows the push pixel for pixel at first
/// and stiffens towards this, so a hard fling cannot throw the list a screen past its end.
const OVERSCROLL_LIMIT: f32 = 100.0;

/// A scroll position, which the world and server lists offset their rows by. It draws nothing
/// itself.
///
/// **The input is a distance, not a speed**, and it is spread rather than applied whole.
/// `scroll_push` is the sum of the scroll events and `scroll_pos` is where the ends put that.
///
/// It used to be a velocity the events raised and a decay that spent it, which is momentum - and
/// a macOS trackpad already sends its own. **Its momentum arrives as ordinary scroll events**,
/// a decaying stream a frame apart carrying on for a third of a second after the finger lifts,
/// and nothing in them says which side of the lift they are from. So the deltas are the whole
/// gesture and there is nothing here to add: no second momentum, and no waiting for a gesture
/// to "end" - a wait long enough to cover the tail holds the band stretched for all of it and
/// then lets go, which is a list that hangs and snaps.
///
/// Spreading is what lets the band return while the stream is still arriving. An event applied
/// whole is 16 ms of scrolling in one millisecond, and against a return running every
/// millisecond that is a sawtooth at frame frequency - the shake. Drained over
/// `scroll_smooth_factor`, the push is a rate rather than a series of jumps, the two settle
/// against each other, and the same drain is what turns a wheel detent into a glide.
pub struct Scrollable {
    pub rect: gfx::Rect,
    pub orientation: super::Orientation,
    /// Everything the gesture has asked for, unbounded. What lies past an end is the band's
    /// stretch, and returns to the end at `boundary_smooth_factor` whatever else is happening.
    scroll_push: f32,
    /// Scrolling that has arrived but not yet been applied, drained into the push over
    /// `scroll_smooth_factor` milliseconds.
    pending_scroll: f32,
    scroll_pos: f32,
    pub scroll_size: f32,
    animation_timer: timing::FixedStep,
    /// Over how many milliseconds a scroll event is applied. It has to cover a frame, which is
    /// what stops the events beating against the boundary's return.
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
            pending_scroll: 0.0,
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

    /// The push with the ends applied: the part inside them as it is, and whatever lies past
    /// one compressed into the band's stretch. A pure function of the push, which is what keeps
    /// a gesture running past an end smooth - nothing is resisting it frame by frame.
    fn stretched_position(&self) -> f32 {
        let bounded = self.scroll_push.clamp(0.0, self.upper_bound());
        let push_past_end = self.scroll_push - bounded;
        bounded + push_past_end * OVERSCROLL_LIMIT / (OVERSCROLL_LIMIT + push_past_end.abs())
    }

    /// One frame of scrolling: some of what has arrived is applied, a stretched band returns
    /// towards its end, and the drawn position is where that leaves the push. Both movements are
    /// `super::approach`, whose epsilon is what makes them *land* rather than leave a scrolled
    /// list a fraction of a pixel short forever. Split out of `update_inner`, which needs a
    /// `GraphicsContext` and this does not.
    pub(super) fn advance_frame(&mut self) {
        let still_pending = super::approach(self.pending_scroll, 0.0, self.scroll_smooth_factor, 0.01);
        self.scroll_push += self.pending_scroll - still_pending;
        self.pending_scroll = still_pending;

        let bounded = self.scroll_push.clamp(0.0, self.upper_bound());
        self.scroll_push = super::approach(self.scroll_push, bounded, self.boundary_smooth_factor, 0.01);

        self.scroll_pos = self.stretched_position();
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

    /// Queues the scrolled distance. The ends are `stretched_position`'s business and the timing
    /// is `advance_frame`'s, so there is nothing to do here but add it up.
    fn on_event_inner(&mut self, _: &mut dyn super::UiContext, event: &gfx::Event, _: &super::Container) -> bool {
        if let gfx::Event::MouseScroll(pixels) = event {
            self.pending_scroll -= *pixels;
        }
        false
    }

    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> super::Container {
        super::Container::new(graphics, self.rect.pos, self.rect.size, self.orientation, Some(parent_container))
    }
}
