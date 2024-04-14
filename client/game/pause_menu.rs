use crate::client::global_settings::GlobalSettings;
use crate::client::menus::Menu;
use crate::client::menus::SettingsMenu;
use crate::client::menus::MENU_WIDTH;
use crate::client::settings::Settings;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::rc::Rc;

//TODO: change this into a UiElement

/// The pause menu actually does not pause the game (ironic, I know).
/// It just shows a menu with options to quit the world or go back to the game.
pub struct PauseMenu {
    open: bool,
    resume_button: gfx::Button,
    settings_button: gfx::Button,
    quit_button: gfx::Button,
    back_rect: gfx::RenderRect,
    in_settings: bool,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
    settings_menu: SettingsMenu,
    rect_width: f32,
}

impl PauseMenu {
    pub fn new(settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Self {
        Self {
            open: false,
            in_settings: false,
            resume_button: gfx::Button::new(|| {}),
            settings_button: gfx::Button::new(|| {}),
            quit_button: gfx::Button::new(|| {}),
            back_rect: gfx::RenderRect::new(gfx::FloatPos(-10000.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            settings_menu: SettingsMenu::new(settings.clone()),
            rect_width: 0.0,
            settings,
            global_settings,
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext) {
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

        self.settings_menu.init(graphics, &self.back_rect.get_container(graphics, &gfx::Container::default(graphics)));
    }

    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext) {
        self.in_settings &= self.open;
        let parent_container = self.back_rect.get_container(graphics, &gfx::Container::default(graphics));
        self.settings_menu.update(graphics, &parent_container);
        if self.settings_menu.should_close() {
            self.in_settings = false;
        }

        if self.in_settings {
            self.back_rect.size.0 = MENU_WIDTH;
            self.back_rect.pos.0 = graphics.get_window_size().0 / 2.0 - self.back_rect.size.0 / 2.0;
        } else if self.open {
            self.back_rect.pos.0 = 0.0;
            self.back_rect.size.0 = self.rect_width;
        } else {
            self.back_rect.pos.0 = -self.back_rect.size.0 - 100.0;
            self.back_rect.size.0 = self.rect_width;
        }

        let window_container = gfx::Container::default(graphics);

        self.back_rect.render(graphics, &window_container);

        let back_rect = *self.back_rect.get_container(graphics, &window_container).get_absolute_rect();
        let visible = back_rect.pos.0 + back_rect.size.0 > 0.0;

        if graphics.get_window_size().1 as u32 != self.back_rect.size.1 as u32 {
            self.back_rect.size.1 = graphics.get_window_size().1;
            self.back_rect.jump_to_target();
        }

        if visible && !self.in_settings {
            self.resume_button.render(graphics, &self.back_rect.get_container(graphics, &window_container));
            self.settings_button.render(graphics, &self.back_rect.get_container(graphics, &window_container));
            self.quit_button.render(graphics, &self.back_rect.get_container(graphics, &window_container));
        }

        if self.in_settings {
            self.settings_menu.render(graphics, &self.back_rect.get_container(graphics, &window_container));
        }
    }

    /// returns true if the game should quit
    pub fn on_event(&mut self, event: &Event, graphics: &mut gfx::GraphicsContext) -> bool {
        let parent_container = self.back_rect.get_container(graphics, &gfx::Container::default(graphics));
        if let Some(event) = event.downcast::<gfx::Event>() {
            if let gfx::Event::KeyPress(key, false) = event {
                if *key == gfx::Key::Escape && !self.in_settings {
                    self.open = !self.open;
                }
            }

            if !self.open {
                return false;
            }
            if self.in_settings {
                let updated = self.settings_menu.on_event(graphics, event, &parent_container);
                if updated {
                    self.global_settings.borrow_mut().update(graphics, &self.settings);
                }
                return false;
            }
            if self.resume_button.on_event(graphics, event, &parent_container) {
                self.open = false;
            }

            self.settings_button.on_event(graphics, event, &parent_container);
            if self.quit_button.on_event(graphics, event, &parent_container) {
                return true;
            }
        }

        false
    }
}
