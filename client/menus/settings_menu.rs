use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::TRANSPARENCY;

enum SettingUi {
    Toggle {
        setting_id: i32,
        text: gfx::Texture,
    },
    Choice {
        setting_id: i32,
        text: gfx::Texture,
    },
    Slider {
        setting_id: i32,
        text: gfx::Texture,
    },
}

const SETTINGS_WIDTH: f32 = 500.0;
const SETTINGS_BOX_HEIGHT: f32 = 70.0;

fn setting_to_ui(graphics: &mut gfx::GraphicsContext, setting: &Setting, setting_id: i32) -> SettingUi {
    let text = match setting {
        Setting::Toggle { text, .. } | Setting::Choice { text, .. } | Setting::Slider { text, .. } => {
            text
        }
    };

    let text_texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text));

    match setting {
        Setting::Toggle { .. } => {
            SettingUi::Toggle {
                setting_id,
                text: text_texture,
            }
        }
        Setting::Choice { .. } => {
            SettingUi::Choice {
                setting_id,
                text: text_texture,
            }
        }
        Setting::Slider { .. } => {
            SettingUi::Slider {
                setting_id,
                text: text_texture,
            }
        }
    }
}

fn render_setting_ui(graphics: &mut gfx::GraphicsContext, setting: &SettingUi, x: f32, y: f32) {
    let back_rect = gfx::Rect::new(gfx::FloatPos(x, y), gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT));
    back_rect.render(graphics, gfx::BLACK.set_a(TRANSPARENCY));

    match setting {
        SettingUi::Toggle { text, .. } => {
            let y = y + SETTINGS_BOX_HEIGHT / 2.0 - text.get_texture_size().1;
            text.render(&graphics.renderer, 2.0, gfx::FloatPos(x + gfx::SPACING, y), None, false, None);
        }
        SettingUi::Choice { .. } => {}
        SettingUi::Slider { .. } => {}
    }
}

pub struct SettingsMenu {
    back_button: gfx::Button,
    settings: Vec<SettingUi>,
}

impl SettingsMenu {
    #[must_use]
    pub fn new() -> Self {
        Self {
            back_button: gfx::Button::new(),
            settings: Vec::new(),
        }
    }

    pub fn init(&mut self, graphics: &mut gfx::GraphicsContext, settings: &Settings) {
        self.back_button.scale = 3.0;
        self.back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));
        self.back_button.pos.1 = -gfx::SPACING;
        self.back_button.orientation = gfx::BOTTOM;

        for (id, setting) in settings.get_all_settings() {
            self.settings.push(setting_to_ui(graphics, setting, *id));
        }
    }

    /// returns the required width of background container
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext) -> f32 {
        self.back_button.render(graphics, None);

        let x = graphics.renderer.get_window_size().0 / 2.0 - SETTINGS_WIDTH / 2.0;
        let mut y = gfx::SPACING;
        for setting in &self.settings {
            render_setting_ui(graphics, setting, x, y);
            y += gfx::SPACING + SETTINGS_BOX_HEIGHT;
        }

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
