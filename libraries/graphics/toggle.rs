use crate::libraries::graphics as gfx;
use gfx::theme::BUTTON_BORDER_COLOR;
use gfx::{BaseUiElement, UiElement};

/// A two state switch: a rounded bar with a knob that slides from one end to the other.
pub struct Toggle {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
    pub orientation: gfx::Orientation,
    pub padding: f32,
    pub left_color: gfx::Color,
    pub right_color: gfx::Color,
    pub border_color: gfx::Color,
    pub button_color: gfx::Color,
    pub toggled: bool,
    toggle_progress: f32,
    hover_progress: f32,
    /// See `Button::animation_timer`.
    animation_timer: gfx::AnimationTimer,
    /// Whether the most recent press of the left button landed on this toggle. Exactly
    /// `Button::pressed_inside`, and for the same reason: a click is a press *and* a release on
    /// the same widget, so a press that landed somewhere else must not flip whatever the
    /// pointer happens to be over when it comes back up.
    pressed_inside: bool,
    pub changed: bool,
}

impl Toggle {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pos: gfx::FloatPos(0.0, 0.0),
            size: gfx::FloatSize(82.0, 50.0),
            orientation: gfx::TOP_LEFT,
            padding: 5.0,
            left_color: gfx::Color::new(210, 0, 0, 255),
            right_color: gfx::Color::new(0, 210, 0, 255),
            border_color: BUTTON_BORDER_COLOR,
            button_color: gfx::WHITE,
            toggled: false,
            toggle_progress: 0.0,
            hover_progress: 0.0,
            animation_timer: gfx::AnimationTimer::new(1),
            pressed_inside: false,
            changed: true,
        }
    }

    #[must_use]
    pub const fn get_size(&self) -> gfx::FloatSize {
        self.size
    }

    /// Pins both animations for the golden-image tests. See `Button::settle_hover`.
    #[cfg(feature = "render-tests")]
    pub const fn settle_animation(&mut self, toggle_progress: f32, hover_progress: f32) {
        self.toggle_progress = toggle_progress;
        self.hover_progress = hover_progress;
        self.animation_timer.freeze();
    }
}

impl UiElement for Toggle {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let container = self.get_container(graphics, parent_container);
        let toggle_target = if self.toggled { 1.0 } else { 0.0 };
        let hover_target = if self.is_hovered(graphics, parent_container) { 1.0 } else { 0.0 };

        while self.animation_timer.frame_ready() {
            self.toggle_progress = gfx::approach(self.toggle_progress, toggle_target, 40.0, 0.01);
            self.hover_progress = gfx::approach(self.hover_progress, hover_target, 40.0, 0.01);
        }

        // The bar is dimmed to 80% until the mouse is over it.
        let fill_color = gfx::interpolate_colors(self.left_color, self.right_color, self.toggle_progress);
        let dimmed = gfx::Color::new((fill_color.r as f32 * 0.8) as u8, (fill_color.g as f32 * 0.8) as u8, (fill_color.b as f32 * 0.8) as u8, 255);
        let fill_color = gfx::interpolate_colors(dimmed, fill_color, self.hover_progress);

        // The border is the container itself; the bar is that rectangle inset by the padding.
        //
        // Inset in absolute coordinates, **not** by re-laying out a smaller container: a
        // container is placed by its orientation, so shrinking one moves it by the orientation
        // too. That is only symmetric at `CENTER` - a `RIGHT` toggle, which is what the
        // settings menu uses, came out flush against its right edge with twice the padding
        // showing on the left.
        let border = *container.get_absolute_rect();
        border.render(graphics, self.border_color);
        let bar = gfx::Rect::new(
            border.pos + gfx::FloatPos(self.padding, self.padding),
            border.size - gfx::FloatSize(self.padding * 2.0, self.padding * 2.0),
        );
        bar.render(graphics, fill_color);

        // The knob is a square inset from the bar by the padding again, sliding from one end
        // to the other as the toggle animates.
        let knob_size = bar.size.1 - 2.0 * self.padding;
        let knob_x = self.padding * (1.0 - self.toggle_progress) + (bar.size.0 - knob_size - self.padding) * self.toggle_progress;
        let knob = gfx::Rect::new(bar.pos + gfx::FloatPos(knob_x, (bar.size.1 - knob_size) / 2.0), gfx::FloatSize(knob_size, knob_size));
        knob.render(graphics, self.button_color);
    }

    /// Flips on the release of a press that landed on this same toggle, so neither half of a
    /// click that started or finished somewhere else counts. See `Button::on_event_inner`.
    fn on_event_inner(&mut self, graphics: &mut dyn gfx::UiContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        match event {
            gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) => {
                self.pressed_inside = self.is_hovered(graphics, parent_container);
            }
            gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) if self.pressed_inside && self.is_hovered(graphics, parent_container) => {
                self.toggled = !self.toggled;
                self.changed = true;
                return true;
            }
            _ => {}
        }
        false
    }

    fn get_container(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent_container))
    }
}
