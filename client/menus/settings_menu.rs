use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::TRANSPARENCY;

enum SettingUi {
    Toggle {
        setting_id: i32,
        text: gfx::Texture,
        toggle_progress: f32,
        hovered: bool,
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
const TOGGLE_BUTTON_WIDTH: f32 = 30.0;
const TOGGLE_BOX_WIDTH: f32 = 70.0;
const TOGGLE_BOX_HEIGHT: f32 = 40.0;

fn setting_to_ui(graphics: &mut gfx::GraphicsContext, setting: &Setting, setting_id: i32) -> SettingUi {
    let text = match setting {
        Setting::Toggle { text, .. } | Setting::Choice { text, .. } | Setting::Slider { text, .. } => {
            text
        }
    };

    let text_texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text));

    match setting {
        Setting::Toggle { toggled, .. } => {
            SettingUi::Toggle {
                setting_id,
                text: text_texture,
                toggle_progress: if *toggled { 1.0 } else { 0.0 },
                hovered: false,
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

fn render_setting_ui(graphics: &mut gfx::GraphicsContext, setting: &mut SettingUi, settings: &mut Settings, x: f32, y: f32) {
    let back_rect = gfx::Rect::new(gfx::FloatPos(x, y), gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT));
    back_rect.render(graphics, gfx::BLACK.set_a(TRANSPARENCY));

    match setting {
        SettingUi::Toggle { text, hovered, setting_id, toggle_progress, .. } => {
            let mut setting_toggled = false;
            if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(*setting_id) {
                setting_toggled = *toggled;
            }

            let toggle_progress_target = if setting_toggled { 1.0 } else { 0.0 };
            *toggle_progress += (toggle_progress_target - *toggle_progress) / 3.0;

            let text_y = y + SETTINGS_BOX_HEIGHT / 2.0 - text.get_texture_size().1;
            text.render(&graphics.renderer, 2.0, gfx::FloatPos(x + gfx::SPACING, text_y), None, false, None);

            let box_x = x + SETTINGS_WIDTH - TOGGLE_BOX_WIDTH - gfx::SPACING;
            let box_y = y + SETTINGS_BOX_HEIGHT / 2.0 - TOGGLE_BOX_HEIGHT / 2.0;
            let toggle_box_rect = gfx::Rect::new(gfx::FloatPos(box_x, box_y), gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT));
            let color = gfx::Color::new(((1.0 - *toggle_progress) * 200.0) as u8, (*toggle_progress * 200.0) as u8, 0, 255);

            toggle_box_rect.render(graphics, color);

            *hovered = toggle_box_rect.contains(graphics.renderer.get_mouse_pos());

            let inner_spacing = (TOGGLE_BOX_HEIGHT - TOGGLE_BUTTON_WIDTH) / 2.0;

            let button_x = *toggle_progress * (box_x + TOGGLE_BOX_WIDTH - TOGGLE_BUTTON_WIDTH - inner_spacing) + (1.0 - *toggle_progress) * (box_x + inner_spacing);

            let button_y = box_y + inner_spacing;
            let toggle_button_rect = gfx::Rect::new(gfx::FloatPos(button_x, button_y), gfx::FloatSize(TOGGLE_BUTTON_WIDTH, TOGGLE_BUTTON_WIDTH));
            toggle_button_rect.render(graphics, gfx::WHITE);
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
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, settings: &mut Settings) -> f32 {
        self.back_button.render(graphics, None);

        let x = graphics.renderer.get_window_size().0 / 2.0 - SETTINGS_WIDTH / 2.0;
        let mut y = gfx::SPACING;
        for setting in &mut self.settings {
            render_setting_ui(graphics, setting, settings, x, y);
            y += gfx::SPACING + SETTINGS_BOX_HEIGHT;
        }

        SETTINGS_WIDTH + 2.0 * gfx::SPACING
    }

    /// returns true, if settings menu has been closed
    pub fn on_event(&mut self, event: &gfx::Event, graphics: &mut gfx::GraphicsContext, settings: &mut Settings) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            if self.back_button.is_hovered(graphics, None) {
                return true;
            }

            for setting in &mut self.settings {
                match setting {
                    SettingUi::Toggle { hovered, setting_id, .. } => {
                        if *hovered {
                            if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(*setting_id) {
                                *toggled = !*toggled;
                            }
                        }
                    }
                    SettingUi::Choice { .. } => {}
                    SettingUi::Slider { .. } => {}
                }
            }
        }
        false
    }
}
