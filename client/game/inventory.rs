use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};

pub struct ClientInventory {
    is_open: bool,
    back_rect: gfx::RenderRect,
}

impl ClientInventory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_open: false,
            back_rect: gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
        }
    }

    pub fn init(&mut self) {
        self.back_rect.orientation = gfx::TOP;
        self.back_rect.pos = FloatPos(0.0, gfx::SPACING);
        self.back_rect.size = FloatSize(600.0, 70.0);
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.blur_radius = gfx::BLUR / 2;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY / 2;
        self.back_rect.smooth_factor = 20.0;
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext) {
        self.back_rect.size.1 = if self.is_open { 140.0 } else { 70.0 };
        self.back_rect.render(graphics, None);
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(gfx::Event::KeyPress { 0: key, .. }) = event.downcast::<gfx::Event>() {
            if *key == gfx::Key::E {
                self.is_open = !self.is_open;
            }
        }
    }
}
