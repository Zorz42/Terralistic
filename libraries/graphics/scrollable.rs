use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};

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

    #[must_use]
    pub fn get_scroll_x(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> f32 {
        let container = self.get_container(graphics, parent_container);
        container.rect.pos.0 - self.scroll_pos
    }

    #[must_use]
    pub const fn get_scroll_pos(&self) -> f32 {
        self.scroll_pos
    }
}

impl UiElement for Scrollable {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }

    fn render_inner(&mut self, _: &mut gfx::GraphicsContext, _: Option<&gfx::Container>) {
        while self.animation_timer.frame_ready() {
            self.scroll_pos += self.scroll_velocity;

            if self.scroll_pos < 0.0 {
                self.scroll_pos -= self.scroll_pos / self.boundary_smooth_factor;
            }

            let upper_bound = f32::max(self.scroll_size - self.rect.size.1, 0.0);
            if self.scroll_pos > upper_bound {
                self.scroll_pos -= (self.scroll_pos - upper_bound) / self.boundary_smooth_factor;
            }

            self.scroll_velocity -= self.scroll_velocity / self.scroll_smooth_factor;

            if self.scroll_velocity.abs() < 0.01 {
                self.scroll_velocity = 0.0;
            }
        }
    }

    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: Option<&gfx::Container>) {
        //the code in render should be here but i just want to get everything working si i'll leave it in render
    }

    fn on_event(&mut self, _: &mut gfx::GraphicsContext, event: &gfx::Event, _: Option<&gfx::Container>) {
        if let gfx::Event::MouseScroll(delta) = event {
            let delta = -*delta * 0.8;
            if delta > 0.0 {
                self.scroll_velocity = f32::max(self.scroll_velocity, delta);
            } else if delta < 0.0 {
                self.scroll_velocity = f32::min(self.scroll_velocity, delta);
            }
        }
    }

    /// This function returns the container of the rectangle.
    /// The container has the position of render rect.
    #[must_use]
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> gfx::Container {
        gfx::Container::new(graphics, self.rect.pos, self.rect.size, self.orientation, parent_container)
    }
}
