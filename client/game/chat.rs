use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};

pub struct ClientChat {
    back_rect: gfx::RenderRect,
    text_input: gfx::TextInput,
}

impl ClientChat {
    pub fn new(graphics: &mut GraphicsContext) -> Self {
        Self {
            text_input: gfx::TextInput::new(graphics),
            back_rect: gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
        }
    }

    pub fn init(&mut self) {
        self.text_input.orientation = gfx::BOTTOM_LEFT;
        self.text_input.pos = FloatPos(gfx::SPACING, -gfx::SPACING);
        self.text_input.scale = 3.0;
        self.text_input.border_color = gfx::BORDER_COLOR;

        self.back_rect.fill_color = gfx::TRANSPARENT;
        self.back_rect.orientation = gfx::BOTTOM_LEFT;
        self.back_rect.pos = FloatPos(gfx::SPACING, -gfx::SPACING);
        self.back_rect.size.1 = self.text_input.get_size().1;
        self.back_rect.blur_radius = gfx::BLUR;
        self.back_rect.smooth_factor = 60.0;
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext) {
        if self.text_input.selected {
            self.back_rect.size.0 = gfx::TEXT_INPUT_WIDTH * self.text_input.scale;
        } else {
            self.back_rect.size.0 = gfx::TEXT_INPUT_WIDTH * self.text_input.scale * 0.6;
        }

        self.back_rect.render(graphics, None);

        self.text_input.width =
            self.back_rect.get_container(graphics, None).rect.size.0 / self.text_input.scale;
        self.text_input.render(graphics, None);
    }

    pub fn on_event(&mut self, event: &Event, graphics: &mut GraphicsContext) -> bool {
        if let Some(event) = event.downcast::<gfx::Event>() {
            self.text_input.on_event(event, graphics, None);

            if let gfx::Event::KeyPress(gfx::Key::Enter, ..) = event {
                self.text_input.set_text(String::new());
                self.text_input.selected = false;
            }

            if self.text_input.selected
                && (matches!(event, gfx::Event::KeyPress(..))
                    || matches!(event, gfx::Event::KeyRelease(..)))
            {
                return true;
            }
        }
        false
    }
}
