use crate::libraries::graphics as gfx;

pub struct Scrollable {
    pub rect: gfx::Rect,
    pub orientation: gfx::Orientation,
    target_scroll_pos: f32,
    scroll_pos: f32,
    pub scroll_size: f32,
    ms_counter: u32,
    approach_timer: std::time::Instant,
    pub smooth_factor: f32,
}

impl Scrollable {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            orientation: gfx::TOP_LEFT,
            target_scroll_pos: 0.0,
            scroll_pos: 0.0,
            scroll_size: 0.0,
            ms_counter: 0,
            approach_timer: std::time::Instant::now(),
            smooth_factor: 1.0,
        }
    }

    pub fn on_event(&mut self, event: &gfx::Event) {
        if let gfx::Event::MouseScroll(delta) = event {
            self.target_scroll_pos += 3.0 * delta;
        }
    }

    pub fn render(&mut self) {
        while self.ms_counter < self.approach_timer.elapsed().as_millis() as u32 {
            self.ms_counter += 1;
            self.scroll_pos += (self.target_scroll_pos - self.scroll_pos) / self.smooth_factor;

            if self.target_scroll_pos < 0.0 {
                self.target_scroll_pos -= self.target_scroll_pos / self.smooth_factor;
            }

            let upper_bound = f32::max(self.scroll_size - self.rect.size.1, 0.0);
            if self.target_scroll_pos > upper_bound {
                self.target_scroll_pos -=
                    (self.target_scroll_pos - upper_bound) / self.smooth_factor;
            }
        }
    }

    /// This function returns the container of the rectangle.
    /// The container has the position of render rect.
    #[must_use]
    pub fn get_container(
        &self,
        graphics: &gfx::GraphicsContext,
        parent_container: Option<&gfx::Container>,
    ) -> gfx::Container {
        gfx::Container::new(
            graphics,
            self.rect.pos,
            self.rect.size,
            self.orientation,
            parent_container,
        )
    }

    #[must_use]
    pub fn get_scroll_x(
        &self,
        graphics: &mut gfx::GraphicsContext,
        parent_container: Option<&gfx::Container>,
    ) -> f32 {
        let container = self.get_container(graphics, parent_container);
        container.rect.pos.0 - self.scroll_pos
    }

    #[must_use]
    pub const fn get_scroll_pos(&self) -> f32 {
        self.scroll_pos
    }
}
