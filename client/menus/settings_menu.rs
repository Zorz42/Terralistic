use crate::client::global_settings::GlobalSettings;
use crate::libraries::config::SliderSelection;
use crate::libraries::config::{Setting, Settings};
use crate::libraries::graphics as gfx;
use crate::libraries::timing;
use crate::libraries::ui;
use crate::libraries::ui::Menu;
use crate::libraries::ui::{BaseUiElement, UiElement};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::libraries::ui::UiContext;
const SETTINGS_WIDTH: f32 = 700.0;
const SETTINGS_BOX_HEIGHT: f32 = 70.0;
const SETTINGS_ROW_HEIGHT: f32 = ui::SPACING + SETTINGS_BOX_HEIGHT;
/// Where the first row starts: one row down, since the title is drawn at the top of the same
/// container.
const SETTINGS_TOP: f32 = ui::SPACING + SETTINGS_ROW_HEIGHT;
const TOGGLE_BUTTON_WIDTH: f32 = 35.0;
const TOGGLE_BOX_WIDTH: f32 = 70.0;
const TOGGLE_BOX_HEIGHT: f32 = 43.0;
const SLIDER_WIDTH: f32 = 250.0;
const SLIDER_BUTTON_WIDTH: f32 = 10.0;
const SLIDER_HEIGHT: f32 = 50.0;

enum SettingUi {
    Toggle {
        setting_id: i32,
        row: i32,
        text: ui::Sprite,
        toggle: ui::Toggle,
    },
    Choice {
        setting_id: i32,
        row: i32,
        text: ui::Sprite,
        buttons: Vec<ui::Button>,
        choice_rect: ui::RenderRect,
    },
    Slider {
        setting_id: i32,
        row: i32,
        text: ui::Sprite,
        buttons: Vec<ui::Button>,
        choice_rect: ui::RenderRect,
        slider_text: ui::Sprite,
        slider_text_string: String,
        hovered: bool,
        selected: bool,
        hovered_progress: f32,
        animation_timer: timing::FixedStep,
        slider_chosen: bool,
    },
}

impl SettingUi {
    /// `row` is the setting's place in the menu, which is **not** its id: ids come from a
    /// counter that never reuses a number, so the in-game lights setting - registered when a
    /// world loads and removed when it closes - is a higher id every time a world is opened.
    /// Laying out by id left it a row further down the screen on each one.
    pub fn from_settings(graphics: &gfx::GraphicsContext, setting: &Setting, setting_id: i32, row: i32) -> Self {
        let text = match setting {
            Setting::Toggle { text, .. } | Setting::Choice { text, .. } | Setting::Slider { text, .. } => text,
        };

        let mut text_sprite = ui::Sprite::new();
        text_sprite.scale = 2.0;
        text_sprite.orientation = ui::LEFT;
        text_sprite.pos = gfx::FloatPos(ui::SPACING, 0.0);
        text_sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None)));

        match setting {
            Setting::Toggle { toggled, .. } => {
                let mut temp_toggle = ui::Toggle::new();
                temp_toggle.toggled = *toggled;
                temp_toggle.size = gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT);
                temp_toggle.padding = (TOGGLE_BOX_HEIGHT - TOGGLE_BUTTON_WIDTH) / 2.0;
                Self::Toggle {
                    setting_id,
                    row,
                    text: text_sprite,
                    toggle: temp_toggle,
                }
            }
            Setting::Choice { choices, .. } => {
                let mut buttons = Vec::new();

                for choice in choices {
                    let mut button = ui::Button::new(|| {});
                    button.scale = 2.0;
                    button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(choice, None));
                    button.orientation = ui::RIGHT;
                    buttons.push(button);
                }

                let mut choice_rect = ui::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
                choice_rect.fill_color = ui::GREY.set_a(ui::TRANSPARENCY);
                choice_rect.smooth_factor = 30.0;
                choice_rect.orientation = ui::RIGHT;

                Self::Choice {
                    setting_id,
                    row,
                    text: text_sprite,
                    buttons,
                    choice_rect,
                }
            }
            Setting::Slider { choices, .. } => {
                let mut buttons = Vec::new();

                for choice in choices {
                    let mut button = ui::Button::new(|| {});
                    button.scale = 2.0;
                    button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(choice, None));
                    button.orientation = ui::RIGHT;
                    buttons.push(button);
                }

                let mut choice_rect = ui::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
                choice_rect.fill_color = ui::GREY.set_a(ui::TRANSPARENCY);
                choice_rect.smooth_factor = 30.0;
                choice_rect.orientation = ui::RIGHT;

                let mut slider_text = ui::Sprite::new();
                slider_text.scale = 2.0;
                slider_text.orientation = ui::RIGHT;
                slider_text.pos = gfx::FloatPos(-ui::SPACING - SLIDER_WIDTH / 2.0, 0.0); //TODO this is shit, make it centered on the slider

                Self::Slider {
                    setting_id,
                    row,
                    text: text_sprite,
                    buttons,
                    choice_rect,
                    slider_text,
                    slider_text_string: String::new(),
                    hovered: false,
                    selected: false,
                    hovered_progress: 0.0,
                    animation_timer: timing::FixedStep::for_animation(10),
                    slider_chosen: false,
                }
            }
        }
    }
    const fn get_row(&self) -> i32 {
        match self {
            Self::Toggle { row, .. } | Self::Choice { row, .. } | Self::Slider { row, .. } => *row,
        }
    }

    fn update_with_setting(&mut self, graphics: &gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>, parent_container: &ui::Container) {
        let setting_container = self.get_container(graphics, parent_container);
        match self {
            Self::Toggle { setting_id, toggle, .. } => {
                let setting_toggled = if let Ok(Setting::Toggle { toggled, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                    *toggled
                } else {
                    false
                };
                toggle.toggled = setting_toggled;
            }
            Self::Choice { buttons, setting_id, choice_rect, .. } => {
                let chosen_button = if let Ok(Setting::Choice { selected, .. }) = settings.borrow_mut().get_setting_mut(*setting_id) {
                    *selected
                } else {
                    0
                };

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
                let (choice, slider_val_low, slider_val_high) = if let Ok(Setting::Slider {
                    selected, upper_limit, lower_limit, ..
                }) = settings.borrow_mut().get_setting_mut(*setting_id)
                {
                    (selected.clone(), *lower_limit, *upper_limit)
                } else {
                    (SliderSelection::Choice(0), 0, 0)
                };

                *slider_chosen = false;

                match choice {
                    SliderSelection::Slider(slider_choice) => {
                        choice_rect.size = gfx::FloatSize(SLIDER_BUTTON_WIDTH, SLIDER_HEIGHT);
                        let pos_x = (slider_choice - slider_val_low) as f32 * (SLIDER_WIDTH - SLIDER_BUTTON_WIDTH) / (slider_val_high - slider_val_low) as f32;
                        choice_rect.pos = gfx::FloatPos(pos_x - ui::SPACING - SLIDER_WIDTH + SLIDER_BUTTON_WIDTH, 0.0);
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

                let slider_container = ui::Container::new(
                    graphics,
                    gfx::FloatPos(-ui::SPACING, 0.0),
                    gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT),
                    ui::RIGHT,
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

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        let setting_container = self.get_container(graphics, parent_container);
        let mut back_rect = ui::RenderRect::new(setting_container.rect.pos, setting_container.rect.size);
        back_rect.fill_color = ui::BLACK.set_a(ui::TRANSPARENCY);
        back_rect.orientation = ui::TOP;
        back_rect.render(graphics, parent_container);
        if let Self::Slider { hovered_progress, .. } = self {
            let mut slider_rect = ui::RenderRect::new(gfx::FloatPos(-ui::SPACING, 0.0), gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT));
            slider_rect.fill_color = gfx::interpolate_colors(gfx::Color::new(0, 0, 0, ui::TRANSPARENCY), gfx::Color::new(30, 30, 30, ui::TRANSPARENCY), *hovered_progress);
            slider_rect.border_color = gfx::interpolate_colors(gfx::Color::new(0, 0, 0, 0), gfx::Color::new(50, 50, 50, 255), *hovered_progress);
            slider_rect.orientation = ui::RIGHT;
            slider_rect.render(graphics, &self.get_container(graphics, parent_container));
        }
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        let setting_container = self.get_container(graphics, parent_container);
        match self {
            Self::Toggle { toggle, .. } => {
                toggle.pos = gfx::FloatPos(-ui::SPACING, 0.0);
                toggle.size = gfx::FloatSize(TOGGLE_BOX_WIDTH, TOGGLE_BOX_HEIGHT);
                toggle.orientation = ui::RIGHT;
            }
            Self::Choice { buttons, .. } => {
                let mut curr_x = -ui::SPACING;
                for button in buttons {
                    button.pos = gfx::FloatPos(curr_x, 0.0);
                    curr_x -= button.get_size().0 + ui::SPACING;
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
                let slider_container = ui::Container::new(
                    graphics,
                    gfx::FloatPos(-ui::SPACING, 0.0),
                    gfx::FloatSize(SLIDER_WIDTH, SLIDER_HEIGHT),
                    ui::RIGHT,
                    Some(&setting_container),
                );
                let slider_absolute_rect = slider_container.get_absolute_rect();
                *hovered = slider_absolute_rect.contains(graphics.get_mouse_pos());
                while animation_timer.step() {
                    let hover_progress_target = if *hovered || *selected { 1.0 } else { 0.0 };
                    *hovered_progress = ui::approach(*hovered_progress, hover_progress_target, 10.0, 0.001);
                }
                let mut curr_x = -2.0 * ui::SPACING - SLIDER_WIDTH;
                for button in buttons {
                    button.pos = gfx::FloatPos(curr_x, 0.0);
                    curr_x -= button.get_size().0 + ui::SPACING;
                }
            }
        }
    }

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        let y = SETTINGS_TOP + self.get_row() as f32 * SETTINGS_ROW_HEIGHT;
        ui::Container::new(graphics, gfx::FloatPos(0.0, y), gfx::FloatSize(SETTINGS_WIDTH, SETTINGS_BOX_HEIGHT), ui::TOP, Some(parent_container))
    }
}

pub struct SettingsMenu {
    title: ui::Sprite,
    back_button: ui::Button,
    settings_ui: Vec<SettingUi>,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
    close_self: bool,
}

impl SettingsMenu {
    #[must_use]
    pub fn new(graphics: &gfx::GraphicsContext, settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Self {
        let mut title = ui::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Settings", None)));
        title.pos.1 = ui::SPACING;
        title.orientation = ui::TOP;

        Self {
            title,
            back_button: ui::Button::new(|| {}),
            settings_ui: Vec::new(),
            settings,
            global_settings,
            close_self: false,
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext, _: &ui::Container) {
        self.back_button.scale = 3.0;
        self.back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        self.back_button.pos.1 = -ui::SPACING;
        self.back_button.orientation = ui::BOTTOM;

        let binding = self.settings.borrow_mut();
        let mut keys: Vec<&i32> = binding.get_all_settings().keys().collect();
        keys.sort();
        for (row, id) in keys.into_iter().enumerate() {
            if let Ok(setting) = binding.get_setting(*id) {
                self.settings_ui.push(SettingUi::from_settings(graphics, setting, *id, row as i32));
            }
        }
    }
}

impl UiElement for SettingsMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = vec![&mut self.back_button, &mut self.title];
        for element in &mut self.settings_ui {
            elements_vec.push(element);
        }
        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = vec![&self.back_button, &self.title];
        for element in &self.settings_ui {
            elements_vec.push(element);
        }
        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        for element in &mut self.settings_ui {
            element.update_with_setting(graphics, &self.settings, parent_container);
        }
    }

    /// returns true, if settings menu has been closed
    fn on_event_inner(&mut self, graphics: &mut dyn ui::UiContext, event: &gfx::Event, parent_container: &ui::Container) -> bool {
        if let gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) = event {
            for setting in &mut self.settings_ui {
                let setting_container = setting.get_container(graphics, parent_container);
                match setting {
                    // The toggle is a sub-element, so it has already seen this release and
                    // decided for itself - a click being a press *and* a release on it. Reading
                    // the answer back beats deciding again from the hover, which flipped the
                    // setting for any release that happened to land on the toggle, whatever the
                    // press before it had been aimed at.
                    SettingUi::Toggle { toggle, setting_id, .. } => {
                        if let Ok(Setting::Toggle { toggled, .. }) = self.settings.borrow_mut().get_setting_mut(*setting_id) {
                            *toggled = toggle.toggled;
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

        if let Some(graphics) = graphics.as_graphics_context() {
            self.global_settings.borrow_mut().update(graphics, self.settings.borrow());
        }

        false
    }

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

impl Menu for SettingsMenu {
    fn should_close(&mut self) -> bool {
        let ret_val = self.close_self;
        self.close_self = false;
        ret_val
    }

    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        None
    }
}
