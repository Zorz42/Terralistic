use crate::libraries::graphics as gfx;
use gfx::theme::BUTTON_BORDER_COLOR;
use gfx::{BaseUiElement, UiElement};

/// A two state switch: a rounded bar with a knob that slides from one end to the other.
pub struct Toggle {
    pub pos: gfx::FloatPos,
    pub size: gfx::FloatSize,
    pub orientation: gfx::Orientation,
    pub padding: f32,
    pub scale: f32,
    pub left_color: gfx::Color,
    pub right_color: gfx::Color,
    pub border_color: gfx::Color,
    pub button_color: gfx::Color,
    pub toggled: bool,
    pub hovered: bool,
    toggle_progress: f32,
    hover_progress: f32,
    timer: std::time::Instant,
    /// Milliseconds of animation already applied. See `Button::timer_counter`.
    timer_counter: u64,
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
            scale: 1.0,
            left_color: gfx::Color::new(210, 0, 0, 255),
            right_color: gfx::Color::new(0, 210, 0, 255),
            border_color: BUTTON_BORDER_COLOR,
            button_color: gfx::WHITE,
            toggled: false,
            hovered: false,
            toggle_progress: 0.0,
            hover_progress: 0.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
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
        self.timer_counter = u64::MAX;
    }

    #[must_use]
    pub fn is_hovered(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> bool {
        self.get_container(graphics, parent_container).get_absolute_rect().contains(graphics.get_mouse_pos())
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
        let mut container = self.get_container(graphics, parent_container);
        let toggle_target = if self.toggled { 1.0 } else { 0.0 };
        let hover_target = if self.is_hovered(graphics, parent_container) { 1.0 } else { 0.0 };

        while self.timer_counter < self.timer.elapsed().as_millis() as u64 {
            self.toggle_progress = gfx::approach(self.toggle_progress, toggle_target, 40.0, 0.01);
            self.hover_progress = gfx::approach(self.hover_progress, hover_target, 40.0, 0.01);
            self.timer_counter += 1;
        }

        // The bar is dimmed to 80% until the mouse is over it.
        let fill_color = gfx::interpolate_colors(self.left_color, self.right_color, self.toggle_progress);
        let dimmed = gfx::Color::new((fill_color.r as f32 * 0.8) as u8, (fill_color.g as f32 * 0.8) as u8, (fill_color.b as f32 * 0.8) as u8, 255);
        let fill_color = gfx::interpolate_colors(dimmed, fill_color, self.hover_progress);

        // The border is the container itself; the bar is the same rectangle inset by the
        // padding, which is why the container is resized in place here.
        container.rect.render(graphics, self.border_color);
        container.rect.size = container.rect.size - gfx::FloatSize(self.padding * 2.0, self.padding * 2.0);
        container.update(graphics, parent_container);
        container.get_absolute_rect().render(graphics, fill_color);

        let knob_size = gfx::FloatSize(container.rect.size.1 - 2.0 * self.padding, container.rect.size.1 - 2.0 * self.padding);
        let knob_pos = gfx::FloatPos(
            self.padding * (1.0 - self.toggle_progress) + (container.rect.size.0 - knob_size.0 - self.padding) * self.toggle_progress,
            0.0,
        );
        let knob = gfx::Container::new(graphics, knob_pos, knob_size, gfx::LEFT, Some(&container));
        knob.get_absolute_rect().render(graphics, self.button_color);
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.hovered = self.is_hovered(graphics, parent_container);
    }

    fn on_event_inner(&mut self, graphics: &mut dyn gfx::UiContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            if self.is_hovered(graphics, parent_container) {
                self.toggled = !self.toggled;
                self.changed = true;
                return true;
            }
        }
        false
    }

    fn get_container(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent_container))
    }
}
