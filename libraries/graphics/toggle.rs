use crate::libraries::graphics as gfx;
use gfx::theme::GFX_DEFAULT_BUTTON_BORDER_COLOR;
use gfx::{BaseUiElement, UiElement};

/// A Toggle is a rectangle with 2 states.
/// It can be clicked to toggle between them.
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
    timer_counter: u32,
    pub changed: bool,
}

impl Toggle {
    /// Creates a new button.
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
            border_color: GFX_DEFAULT_BUTTON_BORDER_COLOR,
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

    /// returns the size.
    #[must_use]
    pub const fn get_size(&self) -> gfx::FloatSize {
        self.size
    }

    /// Checks if the toggle is hovered with a mouse.
    #[must_use]
    pub fn is_hovered(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> bool {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_pos = graphics.get_mouse_pos();
        rect.contains(mouse_pos)
    }
}

impl UiElement for Toggle {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }

    /// Renders the toggle.
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.hovered = self.is_hovered(graphics, parent_container);
        let mut container = self.get_container(graphics, parent_container);
        let float_size_padding = gfx::FloatSize(self.padding, self.padding);
        let toggle_target = if self.toggled { 1.0 } else { 0.0 };
        let hover_target = if self.is_hovered(graphics, parent_container) { 1.0 } else { 0.0 };

        while self.timer_counter < self.timer.elapsed().as_millis() as u32 {
            self.toggle_progress += (toggle_target - self.toggle_progress) / 40.0;
            if (toggle_target - self.toggle_progress).abs() <= 0.01 {
                self.toggle_progress = toggle_target;
            }
            self.hover_progress += (hover_target - self.hover_progress) / 40.0;
            if (hover_target - self.hover_progress).abs() <= 0.01 {
                self.hover_progress = hover_target;
            }
            self.timer_counter += 1;
        }

        let fill_color = gfx::interpolate_colors(self.left_color, self.right_color, self.toggle_progress);
        let fill_color = gfx::interpolate_colors(
            gfx::Color {
                r: (fill_color.r as f32 * 0.8) as u8,
                g: (fill_color.g as f32 * 0.8) as u8,
                b: (fill_color.b as f32 * 0.8) as u8,
                a: 255,
            },
            fill_color,
            self.hover_progress,
        );

        container.rect.render(graphics, self.border_color);
        container.rect.size = container.rect.size - float_size_padding - float_size_padding;
        container.update(graphics, Some(parent_container));
        container.get_absolute_rect().render(graphics, fill_color);
        let size = gfx::FloatSize(container.rect.size.1 - 2.0 * self.padding, container.rect.size.1 - 2.0 * self.padding);

        let position = gfx::FloatPos(
            self.padding * (1.0 - self.toggle_progress) + (container.rect.size.0 - size.0 - self.padding) * self.toggle_progress,
            0.0,
        );

        let button = gfx::container::Container::new(graphics, position, size, gfx::LEFT, Some(&container));
        button.get_absolute_rect().render(graphics, self.button_color);
    }

    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &gfx::Container) {
        //nothing to update for now
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            if self.is_hovered(graphics, parent_container) {
                self.toggled = !self.toggled;
                self.changed = true;
                return true;
            }
        }
        false
    }

    /// Generates the container for the toggle.
    #[must_use]
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent_container))
    }
}
