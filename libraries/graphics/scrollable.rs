use crate::libraries::graphics as gfx;
use gfx::UiElement;

/// A scroll position with momentum, which the world and server lists offset their rows by.
/// It draws nothing itself.
pub struct Scrollable {
    pub rect: gfx::Rect,
    pub orientation: gfx::Orientation,
    scroll_velocity: f32,
    scroll_pos: f32,
    pub scroll_size: f32,
    animation_timer: gfx::AnimationTimer,
    pub scroll_smooth_factor: f32,
    pub boundary_smooth_factor: f32,
}

impl Scrollable {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            orientation: gfx::TOP_LEFT,
            scroll_velocity: 0.0,
            scroll_pos: 0.0,
            scroll_size: 0.0,
            animation_timer: gfx::AnimationTimer::new(1),
            scroll_smooth_factor: 1.0,
            boundary_smooth_factor: 1.0,
        }
    }

    /// Where a list should start drawing: the scrollable's own offset, less how far it has been
    /// scrolled. Laying a container out to read this back would give the same number -
    /// `Container::rect` is the position and size it was handed - so it does not need one.
    ///
    /// **This is the vertical axis, and everything else here is too**: `scroll_pos` is bounded
    /// against `rect.size.1`, and both callers add the result to a y coordinate. It used to be
    /// `get_scroll_x` and read `rect.pos.0`, which only ever gave the right answer because the
    /// two menus leave their x at zero and add the y offset back by hand.
    #[must_use]
    pub const fn get_scroll_y(&self) -> f32 {
        self.rect.pos.1 - self.scroll_pos
    }

    #[must_use]
    pub const fn get_scroll_pos(&self) -> f32 {
        self.scroll_pos
    }

    /// One frame of scrolling: the velocity moves the position, the position is pulled back
    /// inside its bounds, and the velocity decays.
    ///
    /// Both pulls are `gfx::approach`, like every other animation in the toolkit, which is
    /// what makes them *land* rather than close in on the target forever. Subtracting a
    /// fraction of the remaining distance - which is all this used to do - leaves a list
    /// flicked past its end a fraction of a pixel past it for as long as the menu is open.
    ///
    /// Split out of `update_inner` because that takes a `GraphicsContext` a headless test has
    /// no way to build, even though none of this needs one.
    pub(super) fn advance_frame(&mut self) {
        self.scroll_pos += self.scroll_velocity;

        let upper_bound = f32::max(self.scroll_size - self.rect.size.1, 0.0);
        if self.scroll_pos < 0.0 {
            self.scroll_pos = gfx::approach(self.scroll_pos, 0.0, self.boundary_smooth_factor, 0.01);
        } else if self.scroll_pos > upper_bound {
            self.scroll_pos = gfx::approach(self.scroll_pos, upper_bound, self.boundary_smooth_factor, 0.01);
        }

        self.scroll_velocity = gfx::approach(self.scroll_velocity, 0.0, self.scroll_smooth_factor, 0.01);
    }
}

impl UiElement for Scrollable {
    /// Advances the scroll, and does nothing else.
    ///
    /// **`update_inner`, not `render_inner`** - moving is not drawing, and stepping it while
    /// rendering froze the scroll for any caller that laid the list out without drawing it, a
    /// menu sliding offscreen for instance. The parent reads `get_scroll_y` from its own
    /// `update_inner`, which the recursion in `BaseUiElement::update` runs first, so it sees
    /// the previous frame's position.
    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &gfx::Container) {
        while self.animation_timer.frame_ready() {
            self.advance_frame();
        }
    }

    fn on_event_inner(&mut self, _: &mut dyn gfx::UiContext, event: &gfx::Event, _: &gfx::Container) -> bool {
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

    fn get_container(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.rect.pos, self.rect.size, self.orientation, Some(parent_container))
    }
}
