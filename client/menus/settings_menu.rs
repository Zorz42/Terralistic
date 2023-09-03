use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;

enum SettingUi {
    Toggle {

    },
    Choice {

    },
    Slider {

    },
}

const SETTINGS_WIDTH: f32 = 500.0;

pub struct SettingsMenu {
    back_button: gfx::Button,
}

impl SettingsMenu {
    #[must_use]
    pub fn new() -> Self {
        Self {
            back_button: gfx::Button::new(),
        }
    }

    pub fn init(&mut self, graphics: &mut gfx::GraphicsContext, settings: &Settings) {
        self.back_button.scale = 3.0;
        self.back_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));
        self.back_button.pos.1 = -gfx::SPACING;
        self.back_button.orientation = gfx::BOTTOM;
    }

    /// returns the required width of background container
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext) -> f32 {
        self.back_button.render(graphics, None);
        SETTINGS_WIDTH + 2.0 * gfx::SPACING
    }

    /// returns true, if settings menu has been closed
    pub fn on_event(&mut self, event: &gfx::Event, graphics: &mut gfx::GraphicsContext) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            if self.back_button.is_hovered(graphics, None) {
                return true;
            }
        }
        false
    }
}
