use crate::client::global_settings::GlobalSettings;
use crate::client::menus::SettingsMenu;
use crate::client::settings::Settings;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::rc::Rc;

/// The pause menu actually does not pause the game (ironic, I know).
/// It just shows a menu with options to quit the world or go back to the game.
pub struct PauseMenu {
    open: bool,
    in_settings: bool,
    resume_button: gfx::Button,
    settings_button: gfx::Button,
    quit_button: gfx::Button,
    back_rect: gfx::RenderRect,
    settings_menu: SettingsMenu,
    rect_width: f32,
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {
            open: false,
            in_settings: false,
            resume_button: gfx::Button::new(|| {}),
            settings_button: gfx::Button::new(|| {}),
            quit_button: gfx::Button::new(|| {}),
            back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            settings_menu: SettingsMenu::new(),
            rect_width: 0.0,
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>) {
        self.resume_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Resume", None));
        self.resume_button.scale = 3.0;
        self.resume_button.pos.0 = -gfx::SPACING;
        self.resume_button.pos.1 = gfx::SPACING;
        self.resume_button.orientation = gfx::TOP_RIGHT;

        self.settings_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Settings", None));
        self.settings_button.scale = 3.0;
        self.settings_button.pos.0 = -gfx::SPACING;
        self.settings_button.pos.1 = 2.0 * gfx::SPACING + self.resume_button.get_size().1;
        self.settings_button.orientation = gfx::TOP_RIGHT;

        self.quit_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Quit", None));
        self.quit_button.scale = 3.0;
        self.quit_button.pos.0 = -gfx::SPACING;
        self.quit_button.pos.1 = 3.0 * gfx::SPACING + self.settings_button.get_size().1 + self.resume_button.get_size().1;
        self.quit_button.orientation = gfx::TOP_RIGHT;

        self.rect_width = f32::max(self.resume_button.get_size().0, f32::max(self.settings_button.get_size().0, self.quit_button.get_size().0)) + 2.0 * gfx::SPACING;

        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.border_color = gfx::BORDER_COLOR;
        self.back_rect.blur_radius = gfx::BLUR;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        self.back_rect.smooth_factor = 60.0;

        self.settings_menu.init(graphics, settings);
    }

    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>, global_settings: &Rc<RefCell<GlobalSettings>>) {
        if self.open && self.in_settings {
            self.back_rect.pos.0 = graphics.get_window_size().0 / 2.0 - self.back_rect.size.0 / 2.0;
        } else if self.open {
            self.back_rect.pos.0 = 0.0;
            self.back_rect.size.0 = self.rect_width;
        } else {
            self.back_rect.pos.0 = -self.back_rect.size.0 - 100.0;
            self.back_rect.size.0 = self.rect_width;
        }

        //self.back_rect.render(graphics, None);

        /*let back_rect = *self.back_rect.get_container(graphics, None).get_absolute_rect();
        let visible = back_rect.pos.0 + back_rect.size.0 > 0.0;*/

        if graphics.get_window_size().1 as u32 != self.back_rect.size.1 as u32 {
            self.back_rect.size.1 = graphics.get_window_size().1;
            self.back_rect.jump_to_target();
        }

        /*if visible && !self.in_settings {
            self.resume_button.render(graphics, Some(&self.back_rect.get_container(graphics, None)));
            self.settings_button.render(graphics, Some(&self.back_rect.get_container(graphics, None)));
            self.quit_button.render(graphics, Some(&self.back_rect.get_container(graphics, None)));
        }*/

        if self.in_settings {
            let width = self.settings_menu.render(graphics, settings);
            self.back_rect.size.0 = width;
            global_settings.borrow_mut().update(graphics, settings);
        }
    }

    /// returns true if the game should quit
    pub fn on_event(&mut self, event: &Event, graphics: &gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>) -> bool {
        if let Some(event) = event.downcast::<gfx::Event>() {
            if self.in_settings {
                if self.settings_menu.on_event(event, graphics, settings) {
                    self.in_settings = false;
                }
                return false;
            }

            match event {
                gfx::Event::KeyPress(key, false) => {
                    if *key == gfx::Key::Escape {
                        self.open = !self.open;
                    }
                }
                /*gfx::Event::KeyRelease(key, false) => {
                    if *key == gfx::Key::MouseLeft && self.open {
                        if self.resume_button.is_hovered(graphics, Some(&self.back_rect.get_container(graphics, None))) {
                            self.open = false;
                        } else if self.settings_button.is_hovered(graphics, Some(&self.back_rect.get_container(graphics, None))) {
                            self.in_settings = true;
                        } else if self.quit_button.is_hovered(graphics, Some(&self.back_rect.get_container(graphics, None))) {
                            return true;
                        }
                    }
                }*///TODO UI element
                _ => {}
            }
        }

        false
    }
}
