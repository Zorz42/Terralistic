use anyhow::Result;
use directories::BaseDirs;
use std::cell::RefCell;
use std::rc::Rc;

use crate::client::game::private_world::run_private_world;
use crate::client::global_settings::GlobalSettings;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};

fn world_name_exists(worlds_list: &Vec<String>, name: &str) -> bool {
    for world_name in worlds_list {
        if world_name.to_uppercase() == name.to_uppercase() {
            return true;
        }
    }
    false
}

pub struct WorldCreationMenu {
    title: gfx::Sprite,
    back_button: gfx::Button,
    create_button: gfx::Button,
    world_name_input: gfx::TextInput,
    world_seed_input: gfx::TextInput,
    base_dirs: BaseDirs,
    worlds_list: Vec<String>,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
    world_path: Rc<RefCell<std::path::PathBuf>>,
}

impl WorldCreationMenu {
    pub fn new(
        graphics: &gfx::GraphicsContext,
        worlds_list: Vec<String>,
        settings: Rc<RefCell<Settings>>,
        global_settings: Rc<RefCell<GlobalSettings>>,
        close_menu: Rc<RefCell<bool>>,
    ) -> Result<Self> {
        let base_dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("Failed to get base directories"))?;
        let world_path = base_dirs.data_dir().to_path_buf();
        let mut title = gfx::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Create a new world:", None)));
        title.pos.1 = gfx::SPACING;
        title.orientation = gfx::TOP;

        let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

        let cloned_close_menu = close_menu.clone();
        let mut back_button = gfx::Button::new(move || *close_menu.borrow_mut() = true);
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.orientation = gfx::BOTTOM;

        let mut create_button = gfx::Button::new(move || *cloned_close_menu.borrow_mut() = true);
        create_button.scale = 3.0;
        create_button.darken_on_disabled = true;
        create_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Create world", None));
        create_button.pos.0 = back_button.get_size().0 + gfx::SPACING;
        create_button.orientation = gfx::BOTTOM;

        back_button.pos = gfx::FloatPos(-create_button.get_size().0 / 2.0 - gfx::SPACING, -gfx::SPACING);
        create_button.pos = gfx::FloatPos(back_button.get_size().0 / 2.0 + gfx::SPACING, -gfx::SPACING);

        buttons_container.rect.size.0 = back_button.get_size().0 + create_button.get_size().0 + gfx::SPACING;
        buttons_container.rect.size.1 = back_button.get_size().1;
        buttons_container.rect.pos.1 = -gfx::SPACING;

        let mut world_name_input = gfx::TextInput::new(graphics);
        world_name_input.scale = 3.0;
        world_name_input.set_hint(graphics, "World name");
        world_name_input.orientation = gfx::CENTER;
        world_name_input.selected = true;
        world_name_input.pos.1 = -(world_name_input.get_size().1 + gfx::SPACING) / 2.0;

        let mut world_seed_input = gfx::TextInput::new(graphics);
        world_seed_input.scale = 3.0;
        world_seed_input.set_hint(graphics, "World seed");
        world_seed_input.orientation = gfx::CENTER;
        world_seed_input.pos.1 = (world_seed_input.get_size().1 + gfx::SPACING) / 2.0;

        world_name_input.text_processing = Some(Box::new(|text: char| {
            // this closure only accepts letters, numbers and _ symbol
            if text.is_alphanumeric() || text == '_' {
                return Some(text);
            }
            None
        }));

        world_seed_input.text_processing = Some(Box::new(|text: char| {
            // this closure only accepts numbers
            if text.is_numeric() {
                return Some(text);
            }
            None
        }));

        Ok(Self {
            title,
            back_button,
            create_button,
            world_name_input,
            world_seed_input,
            base_dirs,
            worlds_list,
            settings,
            global_settings,
            world_path: Rc::new(RefCell::new(world_path)),
        })
    }
}

impl UiElement for WorldCreationMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.title, &mut self.back_button, &mut self.create_button, &mut self.world_name_input, &mut self.world_seed_input]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.title, &self.back_button, &self.create_button, &self.world_name_input, &self.world_seed_input]
    }

    fn update_inner(&mut self, _grpahics: &mut gfx::GraphicsContext, _parent_container: &gfx::Container) {
        *self.world_path.borrow_mut() = self.base_dirs.data_dir().join("Terralistic").join("Worlds").join(self.world_name_input.get_text().clone() + ".world");

        self.create_button.disabled = world_name_exists(&self.worlds_list, self.world_name_input.get_text()) || self.world_name_input.get_text().is_empty();
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if self.create_button.on_event(graphics, event, &self.get_container(graphics, parent_container)) {
            let mut menu_back = crate::MenuBack::new(graphics);
            let game_result = run_private_world(graphics, &mut menu_back, &self.world_path.borrow_mut().clone(), &self.settings, &self.global_settings);
            if let Err(error) = game_result {
                println!("Game error: {error}");
            }
        }
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::Escape => {
                    if self.world_name_input.selected || self.world_seed_input.selected {
                        self.world_name_input.selected = false;
                        self.world_seed_input.selected = false;
                    } else {
                        self.back_button.press();
                    }
                }
                gfx::Key::Enter => {
                    if !self.create_button.disabled {
                        self.create_button.press();
                        self.back_button.press();
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}
