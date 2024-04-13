use crate::client::settings::SliderSelection;
use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::rc::Rc;

const SETTINGS_WIDTH: f32 = 700.0;
const SETTINGS_BOX_HEIGHT: f32 = 70.0;
const TOGGLE_BUTTON_WIDTH: f32 = 35.0;
const TOGGLE_BOX_WIDTH: f32 = 70.0;
const TOGGLE_BOX_HEIGHT: f32 = 43.0;
const SLIDER_WIDTH: f32 = 250.0;
const SLIDER_BUTTON_WIDTH: f32 = 10.0;
const SLIDER_HEIGHT: f32 = 50.0;

enum SettingUi {
    Toggle {
        setting_id: i32,
        text: gfx::Sprite,
        toggle: gfx::Toggle,
    },
    Choice {
        setting_id: i32,
        text: gfx::Sprite,
        buttons: Vec<gfx::Button>,
        choice_rect: gfx::RenderRect,
    },
    Slider {
        setting_id: i32,
        text: gfx::Sprite,
        buttons: Vec<gfx::Button>,
        choice_rect: gfx::RenderRect,
        slider_text: gfx::Sprite,
        slider_text_string: String,
        hovered: bool,
        selected: bool,
        hovered_progress: f32,
        animation_timer: gfx::AnimationTimer,
        slider_chosen: bool,
    },
}

impl SettingUi {
    pub fn from_settings(graphics: &gfx::GraphicsContext, setting: &Setting, setting_id: i32) -> Self {
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
                Self::Toggle {
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
                    buttons.push(button);
                }

                let mut choice_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
                choice_rect.fill_color = gfx::GREY.set_a(gfx::TRANSPARENCY);
                choice_rect.smooth_factor = 30.0;
                choice_rect.orientation = gfx::RIGHT;

                Self::Choice {
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
                    buttons.push(button);
                }

                let mut choice_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
                choice_rect.fill_color = gfx::GREY.set_a(gfx::TRANSPARENCY);
                choice_rect.smooth_factor = 30.0;
                choice_rect.orientation = gfx::RIGHT;

                let mut slider_text = gfx::Sprite::new();
                slider_text.scale = 2.0;
                slider_text.orientation = gfx::RIGHT;
                slider_text.pos = gfx::FloatPos(-gfx::SPACING - SLIDER_WIDTH / 2.0, 0.0); //TODO this is shit, make it centered on the slider

                Self::Slider {
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
                    slider_chosen: false,
                }
            }
        }
    }
    const fn get_id(&self) -> i32 {
        match self {
            Self::Toggle { setting_id, .. } | Self::Choice { setting_id, .. } | Self::Slider { setting_id, .. } => *setting_id,
        }
    }

    fn update_with_setting(&mut self, graphics: &gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>, parent_container: &gfx::Container) {
        let setting_container = self.get_container(graphics, parent_container);
        match self {
            Self::Toggle { setting_id, toggle, .. } => {
                let mut setting_toggled = false;
                if let Ok(Setting::Toggle { toggled, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                    setting_toggled = *toggled;
                }
                toggle.toggled = setting_toggled;
            }
            Self::Choice { buttons, setting_id, choice_rect, .. } => {
                let mut chosen_button = 0;
                if let Ok(Setting::Choice { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                    chosen_button = *selected;
                }

                if let Some(button) = buttons.get(chosen_button as usize) {
                    choice_rect.pos = button.pos;
                    choice_rect.size = button.get_size();
                }
            }
            Self::Slider {
                buttons,
                setting_id,
                choice_rect,
                slider_text,
                slider_text_string,
                hovered,
                selected,
                slider_chosen,
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

                *slider_chosen = false;

                match choice {
                    SliderSelection::Slider(slider_choice) => {
                        choice_rect.size = gfx::FloatSize(SLIDER_BUTTON_WIDTH, SLIDER_HEIGHT);
                        let pos_x = (slider_choice - slider_val_low) as f32 * (SLIDER_WIDTH - SLIDER_BUTTON_WIDTH) / (slider_val_high - slider_val_low) as f32;
                        choice_rect.pos = gfx::FloatPos(pos_x - gfx::SPACING - SLIDER_WIDTH + SLIDER_BUTTON_WIDTH, 0.0);
                        *slider_chosen = true;

                        if slider_choice.to_string() != *slider_text_string {
                            *slider_text_string = slider_choice.to_string();
                            slider_text.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(slider_text_string, None)));
                        }
                    }
                    SliderSelection::Choice(chosen_button) => {
                        if let Some(button) = buttons.get(chosen_button as usize) {
                            choice_rect.pos = button.pos;
                            choice_rect.size = button.get_size();
                        }
                    }
                }

                let slider_container = gfx::Container::new(
                    graphics,
                    gfx::FloatPos(-gfx::SPACING, 0.0),
                    gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT),
                    gfx::RIGHT,
                    Some(&setting_container),
                );

                let slider_absolute_rect = slider_container.get_absolute_rect();
                *hovered = slider_absolute_rect.contains(graphics.get_mouse_pos());

                if *selected {
                    let mouse_x_in_rect = (graphics.get_mouse_pos().0 - slider_absolute_rect.pos.0).clamp(0.0, slider_absolute_rect.size.0);
                    let slider_val = mouse_x_in_rect / slider_absolute_rect.size.0 * (slider_val_high - slider_val_low) as f32 + slider_val_low as f32;
                    if let Ok(Setting::Slider { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                        *selected = SliderSelection::Slider(slider_val as i32);
                    }
                }
            }
        }
    }
}

impl UiElement for SettingUi {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        match self {
            Self::Toggle { text, toggle, .. } => {
                vec![text, toggle]
            }
            Self::Choice { text, buttons, choice_rect, .. } => {
                let mut elements_vec: Vec<&mut dyn BaseUiElement> = vec![text, choice_rect];
                for button in buttons {
                    elements_vec.push(button);
                }
                elements_vec
            }
            Self::Slider {
                text,
                buttons,
                choice_rect,
                slider_text,
                slider_chosen,
                ..
            } => {
                let mut elements_vec: Vec<&mut dyn BaseUiElement> = vec![text, choice_rect];
                if *slider_chosen {
                    elements_vec.push(slider_text);
                }
                for button in buttons {
                    elements_vec.push(button);
                }
                elements_vec
            }
        }
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        match self {
            Self::Toggle { text, toggle, .. } => {
                vec![text, toggle]
            }
            Self::Choice { text, buttons, choice_rect, .. } => {
                let mut elements_vec: Vec<&dyn BaseUiElement> = vec![text, choice_rect];
                for button in buttons {
                    elements_vec.push(button);
                }
                elements_vec
            }
            Self::Slider {
                text,
                buttons,
                choice_rect,
                slider_text,
                slider_chosen,
                ..
            } => {
                let mut elements_vec: Vec<&dyn BaseUiElement> = vec![text, choice_rect];
                if *slider_chosen {
                    elements_vec.push(slider_text);
                }
                for button in buttons {
                    elements_vec.push(button);
                }
                elements_vec
            }
        }
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let setting_container = self.get_container(graphics, parent_container);
        let mut back_rect = gfx::RenderRect::new(setting_container.rect.pos, setting_container.rect.size);
        back_rect.fill_color = gfx::BLACK.set_a(gfx::TRANSPARENCY);
        back_rect.orientation = gfx::TOP;
        back_rect.render(graphics, parent_container);
        if let Self::Slider { hovered_progress, .. } = self {
            let mut slider_rect = gfx::RenderRect::new(gfx::FloatPos(-gfx::SPACING, 0.0), gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT));
            slider_rect.fill_color = gfx::interpolate_colors(gfx::Color::new(0, 0, 0, gfx::TRANSPARENCY), gfx::Color::new(30, 30, 30, gfx::TRANSPARENCY), *hovered_progress);
            slider_rect.border_color = gfx::interpolate_colors(gfx::Color::new(0, 0, 0, 0), gfx::Color::new(50, 50, 50, 255), *hovered_progress);
            slider_rect.orientation = gfx::RIGHT;
            slider_rect.render(graphics, &self.get_container(graphics, parent_container));
        }
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let setting_container = self.get_container(graphics, parent_container);
        match self {
            Self::Toggle { toggle, .. } => {
                toggle.pos = gfx::FloatPos(-gfx::SPACING, 0.0);
                toggle.size = gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT);
                toggle.orientation = gfx::RIGHT;
            }
            Self::Choice { buttons, .. } => {
                let mut curr_x = -gfx::SPACING;
                for button in buttons {
                    button.pos = gfx::FloatPos(curr_x, 0.0);
                    curr_x -= button.get_size().0 + gfx::SPACING;
                }
            }
            Self::Slider {
                hovered,
                hovered_progress,
                animation_timer,
                selected,
                buttons,
                ..
            } => {
                let slider_container = gfx::Container::new(
                    graphics,
                    gfx::FloatPos(-gfx::SPACING, 0.0),
                    gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT),
                    gfx::RIGHT,
                    Some(&setting_container),
                );
                let slider_absolute_rect = slider_container.get_absolute_rect();
                *hovered = slider_absolute_rect.contains(graphics.get_mouse_pos());
                while animation_timer.frame_ready() {
                    let hover_progress_target = if *hovered || *selected { 1.0 } else { 0.0 };
                    *hovered_progress += (hover_progress_target - *hovered_progress) / 10.0;
                }
                let mut curr_x = -2.0 * gfx::SPACING - SLIDER_WIDTH;
                for button in buttons {
                    button.pos = gfx::FloatPos(curr_x, 0.0);
                    curr_x -= button.get_size().0 + gfx::SPACING;
                }
            }
        }
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        let y = gfx::SPACING + self.get_id() as f32 * (gfx::SPACING + SETTINGS_BOX_HEIGHT);
        gfx::Container::new(graphics, gfx::FloatPos(0.0, y), gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT), gfx::TOP, Some(parent_container))
    }
}

pub struct SettingsMenu {
    back_button: gfx::Button,
    settings_ui: Vec<SettingUi>,
    settings: Rc<RefCell<Settings>>,
    close_self: bool,
}

impl SettingsMenu {
    #[must_use]
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            back_button: gfx::Button::new(|| {}),
            settings_ui: Vec::new(),
            settings,
            close_self: false,
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext, _: &gfx::Container) {
        self.back_button.scale = 3.0;
        self.back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        self.back_button.pos.1 = -gfx::SPACING;
        self.back_button.orientation = gfx::BOTTOM;

        let binding = self.settings.borrow_mut();
        let mut keys: Vec<&i32> = binding.get_all_settings().keys().collect();
        keys.sort();
        for id in keys {
            if let Ok(setting) = binding.get_setting(*id) {
                self.settings_ui.push(SettingUi::from_settings(graphics, setting, *id));
            }
        }
    }
}

impl UiElement for SettingsMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = vec![&mut self.back_button];
        for element in &mut self.settings_ui {
            elements_vec.push(element);
        }
        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = vec![&self.back_button];
        for element in &self.settings_ui {
            elements_vec.push(element);
        }
        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        for element in &mut self.settings_ui {
            element.update_with_setting(graphics, &self.settings, parent_container);
        }
    }

    /// returns true, if settings menu has been closed
    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            for setting in &mut self.settings_ui {
                let setting_container = setting.get_container(graphics, parent_container);
                match setting {
                    SettingUi::Toggle { toggle, setting_id, .. } => {
                        if toggle.hovered {
                            if let Ok(Setting::Toggle { toggled, .. }) = self.settings.borrow_mut().get_setting_mut(*setting_id) {
                                *toggled = !*toggled;
                            }
                        }
                    }
                    SettingUi::Choice { buttons, setting_id, .. } => {
                        for (i, button) in buttons.iter().enumerate() {
                            if button.is_hovered(graphics, &setting_container) {
                                if let Ok(Setting::Choice { selected, .. }) = self.settings.borrow_mut().get_setting_mut(*setting_id) {
                                    *selected = i as i32;
                                }
                            }
                        }
                    }
                    SettingUi::Slider { buttons, setting_id, selected, .. } => {
                        *selected = false;
                        for (i, button) in buttons.iter().enumerate() {
                            if button.is_hovered(graphics, &setting_container) {
                                if let Ok(Setting::Slider { selected, .. }) = self.settings.borrow_mut().get_setting_mut(*setting_id) {
                                    *selected = SliderSelection::Choice(i as i32);
                                }
                            }
                        }
                    }
                }
            }
        } else if let gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) = event {
            for setting in &mut self.settings_ui {
                if let SettingUi::Slider { hovered, selected, .. } = setting {
                    *selected = *hovered;
                }
            }
        } else if let gfx::Event::KeyRelease(key, ..) = event {
            if key == &gfx::Key::Escape {
                self.close_self = true;
                return true;
            }
        }
        if self.back_button.on_event(graphics, event, parent_container) {
            self.close_self = true;
            return true;
        }
        false
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

impl super::Menu for SettingsMenu {
    fn should_close(&self) -> bool {
        self.close_self
    }
}
