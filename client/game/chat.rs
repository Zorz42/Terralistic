use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, GraphicsContext};

pub struct ClientChat {
    text_input: gfx::TextInput,
}

impl ClientChat {
    pub fn new(graphics: &mut GraphicsContext) -> Self {
        Self {
            text_input: gfx::TextInput::new(graphics),
        }
    }

    pub fn init(&mut self) {
        self.text_input.orientation = gfx::BOTTOM_LEFT;
        self.text_input.pos = FloatPos(gfx::SPACING, -gfx::SPACING);
        self.text_input.scale = 3.0;
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext) {
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
