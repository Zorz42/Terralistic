use crate::libraries::graphics as gfx;
use crate::shared::versions::VERSION;
use gfx::{BaseUiElement, UiElement};
use std::cell::Cell;
use std::rc::Rc;

pub struct MainMenu {
    title: gfx::Sprite,
    debug_title: gfx::Sprite,
    version: gfx::Sprite,
    singleplayer_button: gfx::Button,
    multiplayer_button: gfx::Button,
    settings_button: gfx::Button,
    mods_button: gfx::Button,
    exit_button: gfx::Button,
}

impl MainMenu {
    pub fn new(graphics: &gfx::GraphicsContext, open_menu: &Rc<Cell<Option<usize>>>) -> Self {
        let copied_menu = open_menu.clone();
        let mut singleplayer_button = gfx::Button::new(move || copied_menu.set(Some(1)));
        singleplayer_button.scale = 3.0;
        singleplayer_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Singleplayer", None));
        singleplayer_button.orientation = gfx::CENTER;

        let copied_menu = open_menu.clone();
        let mut multiplayer_button = gfx::Button::new(move || copied_menu.set(Some(2)));
        multiplayer_button.scale = 3.0;
        multiplayer_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Multiplayer", None));
        multiplayer_button.orientation = gfx::CENTER;

        let copied_menu = open_menu.clone();
        let mut settings_button = gfx::Button::new(move || copied_menu.set(Some(3)));
        settings_button.scale = 3.0;
        settings_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Settings", None));
        settings_button.orientation = gfx::CENTER;

        let copied_menu = open_menu.clone();
        let mut mods_button = gfx::Button::new(move || copied_menu.set(Some(4)));
        mods_button.scale = 3.0;
        mods_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Mods", None));
        mods_button.orientation = gfx::CENTER;

        let copied_menu = open_menu.clone();
        let mut exit_button = gfx::Button::new(move || copied_menu.set(Some(usize::MAX)));
        exit_button.scale = 3.0;
        exit_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Exit", None));
        exit_button.orientation = gfx::CENTER;

        let mut debug_title = gfx::Sprite::new();
        debug_title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("DEBUG MODE", None)));
        debug_title.color = gfx::DARK_GREY;
        debug_title.orientation = gfx::TOP;
        debug_title.scale = 2.0;
        debug_title.pos.1 = gfx::SPACING / 4.0;

        let mut title = gfx::Sprite::new();
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Terralistic", None)));
        title.scale = 4.0;
        title.orientation = gfx::TOP;
        title.pos.1 = debug_title.pos.1 + debug_title.get_size().1 + gfx::SPACING / 2.0;

        let mut version = gfx::Sprite::new();
        version.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(VERSION, None)));
        version.color = gfx::GREY;
        version.orientation = gfx::BOTTOM;
        version.scale = 2.0;
        version.pos.1 = -5.0;

        {
            let buttons = [&mut singleplayer_button, &mut multiplayer_button, &mut settings_button, &mut mods_button, &mut exit_button];

            // buttons are on top of another on the center of the screen
            // their combined height is centered on the screen

            let mut total_height = 0.0;
            for button in &buttons {
                total_height += button.get_size().1;
            }
            let mut current_y = -total_height / 2.0 + buttons[0].get_size().1;
            for button in buttons {
                button.pos.1 = current_y;
                current_y += button.get_size().1;
            }
        }
        Self {
            title,
            debug_title,
            version,
            singleplayer_button,
            multiplayer_button,
            settings_button,
            mods_button,
            exit_button,
        }
    }
}

impl UiElement for MainMenu {
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![
            &self.title,
            #[cfg(debug_assertions)]
            &self.debug_title,
            &self.version,
            &self.singleplayer_button,
            &self.multiplayer_button,
            &self.settings_button,
            &self.mods_button,
            &self.exit_button,
        ]
    }

    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![
            &mut self.title,
            #[cfg(debug_assertions)]
            &mut self.debug_title,
            &mut self.version,
            &mut self.singleplayer_button,
            &mut self.multiplayer_button,
            &mut self.settings_button,
            &mut self.mods_button,
            &mut self.exit_button,
        ]
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}
