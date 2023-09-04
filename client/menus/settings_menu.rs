use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;

enum SettingUi {
    Toggle {
        setting_id: i32,
        text: gfx::Sprite,
        toggle_progress: f32,
        hovered: bool,
    },
    Choice {
        setting_id: i32,
        text: gfx::Sprite,
        buttons: Vec<(bool, gfx::Button)>,
        choice_rect: gfx::RenderRect,
    },
    Slider {
        setting_id: i32,
        text: gfx::Sprite,
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

    let mut text_sprite = gfx::Sprite::new();
    text_sprite.scale = 2.0;
    text_sprite.orientation = gfx::LEFT;
    text_sprite.pos = gfx::FloatPos(gfx::SPACING, 0.0);
    text_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text));

    match setting {
        Setting::Toggle { toggled, .. } => {
            SettingUi::Toggle {
                setting_id,
                text: text_sprite,
                toggle_progress: if *toggled { 1.0 } else { 0.0 },
                hovered: false,
            }
        }
        Setting::Choice { choices, .. } => {
            let mut buttons = Vec::new();

            for choice in choices {
                let mut button = gfx::Button::new();
                button.scale = 2.0;
                button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(choice));
                button.orientation = gfx::RIGHT;
                buttons.push((false, button));
            }

            let mut choice_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
            choice_rect.fill_color = gfx::GREY.set_a(gfx::TRANSPARENCY);
            choice_rect.smooth_factor = 30.0;
            choice_rect.orientation = gfx::RIGHT;

            SettingUi::Choice {
                setting_id,
                text: text_sprite,
                buttons,
                choice_rect,
            }
        }
        Setting::Slider { .. } => {
            SettingUi::Slider {
                setting_id,
                text: text_sprite,
            }
        }
    }
}

fn render_setting_ui(graphics: &mut gfx::GraphicsContext, setting: &mut SettingUi, settings: &mut Settings, y: f32) {
    let mut back_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, y), gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT));
    back_rect.fill_color = gfx::BLACK.set_a(gfx::TRANSPARENCY);
    back_rect.orientation = gfx::TOP;
    back_rect.render(graphics, None);

    let text = match setting {
        SettingUi::Toggle { text, .. } | SettingUi::Choice { text, .. } | SettingUi::Slider { text, .. } => {
            text
        }
    };

    text.render(graphics, Some(&back_rect.get_container(graphics, None)));

    match setting {
        SettingUi::Toggle { hovered, setting_id, toggle_progress, .. } => {
            let mut setting_toggled = false;
            if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(*setting_id) {
                setting_toggled = *toggled;
            }

            let toggle_progress_target = if setting_toggled { 1.0 } else { 0.0 };
            *toggle_progress += (toggle_progress_target - *toggle_progress) / 3.0;

            let mut toggle_box_rect = gfx::RenderRect::new(gfx::FloatPos(-gfx::SPACING, 0.0), gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT));
            let color = gfx::Color::new(((1.0 - *toggle_progress) * 200.0) as u8, (*toggle_progress * 200.0) as u8, 0, 255);
            toggle_box_rect.fill_color = color;
            toggle_box_rect.orientation = gfx::RIGHT;
            toggle_box_rect.render(graphics, Some(&back_rect.get_container(graphics, None)));

            *hovered = toggle_box_rect.get_container(graphics, Some(&back_rect.get_container(graphics, None))).get_absolute_rect().contains(graphics.renderer.get_mouse_pos());

            let inner_spacing = (TOGGLE_BOX_HEIGHT - TOGGLE_BUTTON_WIDTH) / 2.0;

            let button_x = *toggle_progress * (TOGGLE_BOX_WIDTH - TOGGLE_BUTTON_WIDTH - inner_spacing) + (1.0 - *toggle_progress) * (inner_spacing);

            let mut toggle_button_rect = gfx::RenderRect::new(gfx::FloatPos(button_x, 0.0), gfx::FloatSize(TOGGLE_BUTTON_WIDTH, TOGGLE_BUTTON_WIDTH));
            toggle_button_rect.fill_color = gfx::WHITE;
            toggle_button_rect.orientation = gfx::LEFT;
            toggle_button_rect.render(graphics, Some(&toggle_box_rect.get_container(graphics, Some(&back_rect.get_container(graphics, None)))));
        }
        SettingUi::Choice { buttons, setting_id, choice_rect, .. } => {
            let mut chosen_button = 0;
            if let Ok(Setting::Choice { selected, .. }) = settings.get_setting_mut(*setting_id) {
                chosen_button = *selected;
            }

            if let Some((_hovered, button)) = buttons.get(chosen_button as usize) {
                choice_rect.pos = button.pos;
                choice_rect.size = button.get_size();
            }

            choice_rect.render(graphics, Some(&back_rect.get_container(graphics, None)));

            let mut curr_x = -gfx::SPACING;
            for (hovered, button) in buttons {
                button.pos = gfx::FloatPos(curr_x, 0.0);
                curr_x -= button.get_size().0 + gfx::SPACING;
                button.render(graphics, Some(&back_rect.get_container(graphics, None)));
                *hovered = button.is_hovered(graphics, Some(&back_rect.get_container(graphics, None)));
            }
        }
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

        let mut y = gfx::SPACING;
        for setting in &mut self.settings {
            render_setting_ui(graphics, setting, settings, y);
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
                    SettingUi::Choice { buttons, setting_id, .. } => {
                        for (i, (hovered, _button)) in buttons.iter().enumerate() {
                            if *hovered {
                                if let Ok(Setting::Choice { selected, .. }) = settings.get_setting_mut(*setting_id) {
                                    *selected = i as i32;
                                }
                            }
                        }
                    }
                    SettingUi::Slider { .. } => {}
                }
            }
        }
        false
    }
}
