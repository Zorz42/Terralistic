use crate::client::settings::SliderSelection;
use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;

enum SettingUi {
    Toggle {
        setting_id: i32,
        text: gfx::Sprite,
        toggle_progress: f32,
        hovered_progress: f32,
        hovered: bool,
        animation_timer: gfx::AnimationTimer,
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
const TOGGLE_BUTTON_WIDTH: f32 = 30.0;
const TOGGLE_BOX_WIDTH: f32 = 70.0;
const TOGGLE_BOX_HEIGHT: f32 = 40.0;
const SLIDER_WIDTH: f32 = 250.0;
const SLIDER_BUTTON_WIDTH: f32 = 10.0;
const SLIDER_HEIGHT: f32 = 50.0;

fn setting_to_ui(graphics: &gfx::GraphicsContext, setting: &Setting, setting_id: i32) -> SettingUi {
    let text = match setting {
        Setting::Toggle { text, .. }
        | Setting::Choice { text, .. }
        | Setting::Slider { text, .. } => text,
    };

    let mut text_sprite = gfx::Sprite::new();
    text_sprite.scale = 2.0;
    text_sprite.orientation = gfx::LEFT;
    text_sprite.pos = gfx::FloatPos(gfx::SPACING, 0.0);
    text_sprite.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));

    match setting {
        Setting::Toggle { toggled, .. } => SettingUi::Toggle {
            setting_id,
            text: text_sprite,
            toggle_progress: if *toggled { 1.0 } else { 0.0 },
            hovered_progress: 0.0,
            hovered: false,
            animation_timer: gfx::AnimationTimer::new(10),
        },
        Setting::Choice { choices, .. } => {
            let mut buttons = Vec::new();

            for choice in choices {
                let mut button = gfx::Button::new();
                button.scale = 2.0;
                button.texture = gfx::Texture::load_from_surface(
                    &graphics.font.create_text_surface(choice, None),
                );
                button.orientation = gfx::RIGHT;
                buttons.push((false, button));
            }

            let mut choice_rect =
                gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
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
                let mut button = gfx::Button::new();
                button.scale = 2.0;
                button.texture = gfx::Texture::load_from_surface(
                    &graphics.font.create_text_surface(choice, None),
                );
                button.orientation = gfx::RIGHT;
                buttons.push((false, button));
            }

            let mut choice_rect =
                gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
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
fn render_setting_ui(
    graphics: &gfx::GraphicsContext,
    setting: &mut SettingUi,
    settings: &mut Settings,
    y: f32,
) {
    let mut back_rect = gfx::RenderRect::new(
        gfx::FloatPos(0.0, y),
        gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT),
    );
    back_rect.fill_color = gfx::BLACK.set_a(gfx::TRANSPARENCY);
    back_rect.orientation = gfx::TOP;
    back_rect.render(graphics, None);

    let text = match setting {
        SettingUi::Toggle { text, .. }
        | SettingUi::Choice { text, .. }
        | SettingUi::Slider { text, .. } => text,
    };

    text.render(
        graphics,
        Some(&back_rect.get_container(graphics, None)),
        None,
    );

    match setting {
        SettingUi::Toggle {
            hovered,
            setting_id,
            toggle_progress,
            hovered_progress,
            animation_timer,
            ..
        } => {
            let mut setting_toggled = false;
            if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(*setting_id) {
                setting_toggled = *toggled;
            }

            while animation_timer.frame_ready() {
                let toggle_progress_target = if setting_toggled { 1.0 } else { 0.0 };
                *toggle_progress += (toggle_progress_target - *toggle_progress) / 10.0;

                let hover_progress_target = if *hovered { 1.0 } else { 0.0 };
                *hovered_progress += (hover_progress_target - *hovered_progress) / 30.0;
            }

            let mut toggle_box_rect = gfx::RenderRect::new(
                gfx::FloatPos(-gfx::SPACING, 0.0),
                gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT),
            );
            let color = gfx::interpolate_colors(
                // turned off
                gfx::interpolate_colors(
                    // not hovered
                    gfx::Color::new(170, 0, 0, 255),
                    // hovered
                    gfx::Color::new(210, 0, 0, 255),
                    *hovered_progress,
                ),
                // turned on
                gfx::interpolate_colors(
                    // not hovered
                    gfx::Color::new(0, 170, 0, 255),
                    // hovered
                    gfx::Color::new(0, 210, 0, 255),
                    *hovered_progress,
                ),
                *toggle_progress,
            );

            let border_color = gfx::interpolate_colors(
                // turned off
                gfx::interpolate_colors(
                    // not hovered
                    gfx::Color::new(0, 0, 0, 0),
                    // hovered
                    gfx::Color::new(180, 0, 0, 255),
                    *hovered_progress,
                ),
                // turned on
                gfx::interpolate_colors(
                    // not hovered
                    gfx::Color::new(0, 0, 0, 0),
                    // hovered
                    gfx::Color::new(0, 180, 0, 255),
                    *hovered_progress,
                ),
                *toggle_progress,
            );

            toggle_box_rect.fill_color = color;
            toggle_box_rect.border_color = border_color;
            toggle_box_rect.orientation = gfx::RIGHT;
            toggle_box_rect.render(graphics, Some(&back_rect.get_container(graphics, None)));

            *hovered = toggle_box_rect
                .get_container(graphics, Some(&back_rect.get_container(graphics, None)))
                .get_absolute_rect()
                .contains(graphics.renderer.get_mouse_pos());

            let inner_spacing = (TOGGLE_BOX_HEIGHT - TOGGLE_BUTTON_WIDTH) / 2.0;

            let button_x = *toggle_progress
                * (TOGGLE_BOX_WIDTH - TOGGLE_BUTTON_WIDTH - inner_spacing)
                + (1.0 - *toggle_progress) * (inner_spacing);

            let mut toggle_button_rect = gfx::RenderRect::new(
                gfx::FloatPos(button_x, 0.0),
                gfx::FloatSize(TOGGLE_BUTTON_WIDTH, TOGGLE_BUTTON_WIDTH),
            );
            toggle_button_rect.fill_color = gfx::WHITE;
            toggle_button_rect.orientation = gfx::LEFT;
            toggle_button_rect.render(
                graphics,
                Some(
                    &toggle_box_rect
                        .get_container(graphics, Some(&back_rect.get_container(graphics, None))),
                ),
            );
        }
        SettingUi::Choice {
            buttons,
            setting_id,
            choice_rect,
            ..
        } => {
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
                *hovered =
                    button.is_hovered(graphics, Some(&back_rect.get_container(graphics, None)));
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
                selected,
                upper_limit,
                lower_limit,
                ..
            }) = settings.get_setting_mut(*setting_id)
            {
                choice = selected.clone();
                slider_val_low = *lower_limit;
                slider_val_high = *upper_limit;
            }

            let mut slider_chosen = false;

            match choice {
                SliderSelection::Slider(slider_choice) => {
                    choice_rect.size = gfx::FloatSize(SLIDER_BUTTON_WIDTH, SLIDER_HEIGHT);
                    let pos_x = (slider_choice - slider_val_low) as f32
                        * (SLIDER_WIDTH - SLIDER_BUTTON_WIDTH)
                        / (slider_val_high - slider_val_low) as f32;
                    choice_rect.pos = gfx::FloatPos(
                        pos_x - gfx::SPACING - SLIDER_WIDTH + SLIDER_BUTTON_WIDTH,
                        0.0,
                    );
                    slider_chosen = true;

                    if slider_choice.to_string() != *slider_text_string {
                        *slider_text_string = slider_choice.to_string();
                        slider_text.texture = gfx::Texture::load_from_surface(
                            &graphics.font.create_text_surface(slider_text_string, None),
                        );
                    }
                }
                SliderSelection::Choice(chosen_button) => {
                    if let Some((_hovered, button)) = buttons.get(chosen_button as usize) {
                        choice_rect.pos = button.pos;
                        choice_rect.size = button.get_size();
                    }
                }
            }

            let mut slider_rect = gfx::RenderRect::new(
                gfx::FloatPos(-gfx::SPACING, 0.0),
                gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT),
            );
            slider_rect.fill_color = gfx::interpolate_colors(
                gfx::Color::new(0, 0, 0, gfx::TRANSPARENCY),
                gfx::Color::new(30, 30, 30, gfx::TRANSPARENCY),
                *hovered_progress,
            );
            slider_rect.border_color = gfx::interpolate_colors(
                gfx::Color::new(0, 0, 0, 0),
                gfx::Color::new(50, 50, 50, 255),
                *hovered_progress,
            );
            slider_rect.orientation = gfx::RIGHT;
            slider_rect.render(graphics, Some(&back_rect.get_container(graphics, None)));

            let slider_container =
                slider_rect.get_container(graphics, Some(&back_rect.get_container(graphics, None)));
            let slider_absolute_rect = slider_container.get_absolute_rect();
            *hovered = slider_absolute_rect.contains(graphics.renderer.get_mouse_pos());

            if *selected {
                let mouse_x_in_rect = (graphics.renderer.get_mouse_pos().0
                    - slider_absolute_rect.pos.0)
                    .clamp(0.0, slider_absolute_rect.size.0);
                let slider_val = mouse_x_in_rect / slider_absolute_rect.size.0
                    * (slider_val_high - slider_val_low) as f32
                    + slider_val_low as f32;
                if let Ok(Setting::Slider { selected, .. }) = settings.get_setting_mut(*setting_id)
                {
                    *selected = SliderSelection::Slider(slider_val as i32);
                }
            }

            while animation_timer.frame_ready() {
                let hover_progress_target = if *hovered || *selected { 1.0 } else { 0.0 };
                *hovered_progress += (hover_progress_target - *hovered_progress) / 10.0;
            }

            choice_rect.render(graphics, Some(&back_rect.get_container(graphics, None)));

            if slider_chosen {
                slider_text.render(
                    graphics,
                    Some(
                        &slider_rect.get_container(
                            graphics,
                            Some(&back_rect.get_container(graphics, None)),
                        ),
                    ),
                    None,
                );
            }

            let mut curr_x = -2.0 * gfx::SPACING - SLIDER_WIDTH;
            for (hovered, button) in buttons {
                button.pos = gfx::FloatPos(curr_x, 0.0);
                curr_x -= button.get_size().0 + gfx::SPACING;
                button.render(graphics, Some(&back_rect.get_container(graphics, None)));
                *hovered =
                    button.is_hovered(graphics, Some(&back_rect.get_container(graphics, None)));
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
            back_button: gfx::Button::new(),
            settings: Vec::new(),
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext, settings: &Settings) {
        self.back_button.scale = 3.0;
        self.back_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        self.back_button.pos.1 = -gfx::SPACING;
        self.back_button.orientation = gfx::BOTTOM;

        let mut keys: Vec<&i32> = settings.get_all_settings().keys().collect();
        keys.sort();
        for id in keys {
            if let Ok(setting) = settings.get_setting(*id) {
                self.settings.push(setting_to_ui(graphics, setting, *id));
            }
        }
    }

    /// returns the required width of background container
    pub fn render(&mut self, graphics: &gfx::GraphicsContext, settings: &mut Settings) -> f32 {
        self.back_button.render(graphics, None);

        let mut y = gfx::SPACING;
        for setting in &mut self.settings {
            render_setting_ui(graphics, setting, settings, y);
            y += gfx::SPACING + SETTINGS_BOX_HEIGHT;
        }

        SETTINGS_WIDTH + 2.0 * gfx::SPACING
    }

    /// returns true, if settings menu has been closed
    pub fn on_event(
        &mut self,
        event: &gfx::Event,
        graphics: &gfx::GraphicsContext,
        settings: &mut Settings,
    ) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            if self.back_button.is_hovered(graphics, None) {
                return true;
            }

            for setting in &mut self.settings {
                match setting {
                    SettingUi::Toggle {
                        hovered,
                        setting_id,
                        ..
                    } => {
                        if *hovered {
                            if let Ok(Setting::Toggle { toggled, .. }) =
                                settings.get_setting_mut(*setting_id)
                            {
                                *toggled = !*toggled;
                            }
                        }
                    }
                    SettingUi::Choice {
                        buttons,
                        setting_id,
                        ..
                    } => {
                        for (i, (hovered, _button)) in buttons.iter().enumerate() {
                            if *hovered {
                                if let Ok(Setting::Choice { selected, .. }) =
                                    settings.get_setting_mut(*setting_id)
                                {
                                    *selected = i as i32;
                                }
                            }
                        }
                    }
                    SettingUi::Slider {
                        buttons,
                        setting_id,
                        selected,
                        ..
                    } => {
                        *selected = false;
                        for (i, (hovered, _button)) in buttons.iter().enumerate() {
                            if *hovered {
                                if let Ok(Setting::Slider { selected, .. }) =
                                    settings.get_setting_mut(*setting_id)
                                {
                                    *selected = SliderSelection::Choice(i as i32);
                                }
                            }
                        }
                    }
                }
            }
        } else if let gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) = event {
            for setting in &mut self.settings {
                if let SettingUi::Slider {
                    hovered, selected, ..
                } = setting
                {
                    *selected = *hovered;
                }
            }
        }
        false
    }
}
