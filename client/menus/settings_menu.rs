use crate::client::settings::SliderSelection;
use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::rc::Rc;

enum SettingUi {
    Toggle {
        setting_id: i32,
        text: gfx::Sprite,
        toggle: gfx::Toggle,
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
        buttons: Vec<(bool, gfx::Button)>,
        choice_rect: gfx::RenderRect,
        slider_text: gfx::Sprite,
        slider_text_string: String,
        hovered: bool,
        selected: bool,
        hovered_progress: f32,
        animation_timer: gfx::AnimationTimer,
    },
}

const SETTINGS_WIDTH: f32 = 700.0;
const SETTINGS_BOX_HEIGHT: f32 = 70.0;
const TOGGLE_BUTTON_WIDTH: f32 = 35.0;
const TOGGLE_BOX_WIDTH: f32 = 70.0;
const TOGGLE_BOX_HEIGHT: f32 = 43.0;
const SLIDER_WIDTH: f32 = 250.0;
const SLIDER_BUTTON_WIDTH: f32 = 10.0;
const SLIDER_HEIGHT: f32 = 50.0;

fn setting_to_ui(graphics: &gfx::GraphicsContext, setting: &Setting, setting_id: i32) -> SettingUi {
    let text = match setting {
        Setting::Toggle { text, .. } | Setting::Choice { text, .. } | Setting::Slider { text, .. } => text,
    };

    let mut text_sprite = gfx::Sprite::new();
    text_sprite.scale = 2.0;
    text_sprite.orientation = gfx::LEFT;
    text_sprite.pos = gfx::FloatPos(gfx::SPACING, 0.0);
    text_sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None)));

    match setting {
        Setting::Toggle { toggled, .. } => {
            let mut temp_toggle = gfx::Toggle::new();
            temp_toggle.toggled = *toggled;
            temp_toggle.size = gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT);
            temp_toggle.padding = (TOGGLE_BOX_HEIGHT - TOGGLE_BUTTON_WIDTH) / 2.0;
            SettingUi::Toggle {
                setting_id,
                text: text_sprite,
                toggle: temp_toggle,
            }
        }
        Setting::Choice { choices, .. } => {
            let mut buttons = Vec::new();

            for choice in choices {
                let mut button = gfx::Button::new(|| {});
                button.scale = 2.0;
                button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(choice, None));
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
        Setting::Slider { choices, .. } => {
            let mut buttons = Vec::new();

            for choice in choices {
                let mut button = gfx::Button::new(|| {});
                button.scale = 2.0;
                button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(choice, None));
                button.orientation = gfx::RIGHT;
                buttons.push((false, button));
            }

            let mut choice_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
            choice_rect.fill_color = gfx::GREY.set_a(gfx::TRANSPARENCY);
            choice_rect.smooth_factor = 30.0;
            choice_rect.orientation = gfx::RIGHT;

            let mut slider_text = gfx::Sprite::new();
            slider_text.scale = 2.0;
            slider_text.orientation = gfx::CENTER;

            SettingUi::Slider {
                setting_id,
                text: text_sprite,
                buttons,
                choice_rect,
                slider_text,
                slider_text_string: String::new(),
                hovered: false,
                selected: false,
                hovered_progress: 0.0,
                animation_timer: gfx::AnimationTimer::new(10),
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn render_setting_ui(graphics: &mut gfx::GraphicsContext, setting: &mut SettingUi, settings: &Rc<RefCell<Settings>>, y: f32) {
    let parent_container = gfx::Container::default(graphics);
    let mut back_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, y), gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT));
    back_rect.fill_color = gfx::BLACK.set_a(gfx::TRANSPARENCY);
    back_rect.orientation = gfx::TOP;
    back_rect.render(graphics, &parent_container);

    let text = match setting {
        SettingUi::Toggle { text, .. } | SettingUi::Choice { text, .. } | SettingUi::Slider { text, .. } => text,
    };

    text.render(graphics, &back_rect.get_container(graphics, &parent_container));

    match setting {
        SettingUi::Toggle { setting_id, toggle, .. } => {
            let mut setting_toggled = false;
            if let Ok(Setting::Toggle { toggled, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                setting_toggled = *toggled;
            }

            toggle.pos = gfx::FloatPos(-gfx::SPACING, 0.0);
            toggle.size = gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT);
            toggle.orientation = gfx::RIGHT;

            toggle.toggled = setting_toggled;

            toggle.render(graphics, &back_rect.get_container(graphics, &parent_container));
        }
        SettingUi::Choice { buttons, setting_id, choice_rect, .. } => {
            let mut chosen_button = 0;
            if let Ok(Setting::Choice { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                chosen_button = *selected;
            }

            if let Some((_hovered, button)) = buttons.get(chosen_button as usize) {
                choice_rect.pos = button.pos;
                choice_rect.size = button.get_size();
            }

            choice_rect.render(graphics, &back_rect.get_container(graphics, &parent_container));

            let mut curr_x = -gfx::SPACING;
            for (hovered, button) in buttons {
                button.pos = gfx::FloatPos(curr_x, 0.0);
                curr_x -= button.get_size().0 + gfx::SPACING;
                button.render(graphics, &back_rect.get_container(graphics, &parent_container));
                *hovered = button.is_hovered(graphics, &back_rect.get_container(graphics, &parent_container));
            }
        }
        SettingUi::Slider {
            buttons,
            setting_id,
            choice_rect,
            slider_text,
            slider_text_string,
            hovered,
            selected,
            hovered_progress,
            animation_timer,
            ..
        } => {
            let mut choice = SliderSelection::Choice(0);
            let mut slider_val_low = 0;
            let mut slider_val_high = 0;
            if let Ok(Setting::Slider {
                selected, upper_limit, lower_limit, ..
            }) = settings.borrow_mut().get_setting_mut(*setting_id)
            {
                choice = selected.clone();
                slider_val_low = *lower_limit;
                slider_val_high = *upper_limit;
            }

            let mut slider_chosen = false;

            match choice {
                SliderSelection::Slider(slider_choice) => {
                    choice_rect.size = gfx::FloatSize(SLIDER_BUTTON_WIDTH, SLIDER_HEIGHT);
                    let pos_x = (slider_choice - slider_val_low) as f32 * (SLIDER_WIDTH - SLIDER_BUTTON_WIDTH) / (slider_val_high - slider_val_low) as f32;
                    choice_rect.pos = gfx::FloatPos(pos_x - gfx::SPACING - SLIDER_WIDTH + SLIDER_BUTTON_WIDTH, 0.0);
                    slider_chosen = true;

                    if slider_choice.to_string() != *slider_text_string {
                        *slider_text_string = slider_choice.to_string();
                        slider_text.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(slider_text_string, None)));
                    }
                }
                SliderSelection::Choice(chosen_button) => {
                    if let Some((_hovered, button)) = buttons.get(chosen_button as usize) {
                        choice_rect.pos = button.pos;
                        choice_rect.size = button.get_size();
                    }
                }
            }

            let mut slider_rect = gfx::RenderRect::new(gfx::FloatPos(-gfx::SPACING, 0.0), gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT));
            slider_rect.fill_color = gfx::interpolate_colors(gfx::Color::new(0, 0, 0, gfx::TRANSPARENCY), gfx::Color::new(30, 30, 30, gfx::TRANSPARENCY), *hovered_progress);
            slider_rect.border_color = gfx::interpolate_colors(gfx::Color::new(0, 0, 0, 0), gfx::Color::new(50, 50, 50, 255), *hovered_progress);
            slider_rect.orientation = gfx::RIGHT;
            slider_rect.render(graphics, &back_rect.get_container(graphics, &parent_container));

            let slider_container = slider_rect.get_container(graphics, &back_rect.get_container(graphics, &parent_container));
            let slider_absolute_rect = slider_container.get_absolute_rect();
            *hovered = slider_absolute_rect.contains(graphics.get_mouse_pos());

            if *selected {
                let mouse_x_in_rect = (graphics.get_mouse_pos().0 - slider_absolute_rect.pos.0).clamp(0.0, slider_absolute_rect.size.0);
                let slider_val = mouse_x_in_rect / slider_absolute_rect.size.0 * (slider_val_high - slider_val_low) as f32 + slider_val_low as f32;
                if let Ok(Setting::Slider { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                    *selected = SliderSelection::Slider(slider_val as i32);
                }
            }

            while animation_timer.frame_ready() {
                let hover_progress_target = if *hovered || *selected { 1.0 } else { 0.0 };
                *hovered_progress += (hover_progress_target - *hovered_progress) / 10.0;
            }

            choice_rect.render(graphics, &back_rect.get_container(graphics, &parent_container));

            if slider_chosen {
                slider_text.render(graphics, &slider_rect.get_container(graphics, &back_rect.get_container(graphics, &parent_container)));
            }

            let mut curr_x = -2.0 * gfx::SPACING - SLIDER_WIDTH;
            for (hovered, button) in buttons {
                button.pos = gfx::FloatPos(curr_x, 0.0);
                curr_x -= button.get_size().0 + gfx::SPACING;
                button.render(graphics, &back_rect.get_container(graphics, &parent_container));
                *hovered = button.is_hovered(graphics, &back_rect.get_container(graphics, &parent_container));
            }
        }
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
            back_button: gfx::Button::new(|| {}),
            settings: Vec::new(),
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>) {
        self.back_button.scale = 3.0;
        self.back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        self.back_button.pos.1 = -gfx::SPACING;
        self.back_button.orientation = gfx::BOTTOM;

        let binding = settings.borrow_mut();
        let mut keys: Vec<&i32> = binding.get_all_settings().keys().collect();
        keys.sort();
        for id in keys {
            if let Ok(setting) = binding.get_setting(*id) {
                self.settings.push(setting_to_ui(graphics, setting, *id));
            }
        }
    }

    /// returns the required width of background container
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>) -> f32 {
        self.back_button.render(graphics, &gfx::Container::default(graphics));

        let mut y = gfx::SPACING;
        for setting in &mut self.settings {
            render_setting_ui(graphics, setting, settings, y);
            y += gfx::SPACING + SETTINGS_BOX_HEIGHT;
        }

        SETTINGS_WIDTH + 2.0 * gfx::SPACING
    }

    /// returns true, if settings menu has been closed
    pub fn on_event(&mut self, event: &gfx::Event, graphics: &gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            if self.back_button.is_hovered(graphics, &gfx::Container::default(graphics)) {
                return true;
            }

            for setting in &mut self.settings {
                match setting {
                    SettingUi::Toggle { toggle, setting_id, .. } => {
                        if toggle.hovered {
                            if let Ok(Setting::Toggle { toggled, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                                *toggled = !*toggled;
                            }
                        }
                    }
                    SettingUi::Choice { buttons, setting_id, .. } => {
                        for (i, (hovered, _button)) in buttons.iter().enumerate() {
                            if *hovered {
                                if let Ok(Setting::Choice { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                                    *selected = i as i32;
                                }
                            }
                        }
                    }
                    SettingUi::Slider { buttons, setting_id, selected, .. } => {
                        *selected = false;
                        for (i, (hovered, _button)) in buttons.iter().enumerate() {
                            if *hovered {
                                if let Ok(Setting::Slider { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                                    *selected = SliderSelection::Choice(i as i32);
                                }
                            }
                        }
                    }
                }
            }
        } else if let gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) = event {
            for setting in &mut self.settings {
                if let SettingUi::Slider { hovered, selected, .. } = setting {
                    *selected = *hovered;
                }
            }
        }
        false
    }
}
