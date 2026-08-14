use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::SystemTime;

use crate::client::game::private_world::PrivateWorld;
use directories::BaseDirs;

use crate::client::global_settings::GlobalSettings;
use crate::client::menus::choice_menu::ChoiceMenu;
use crate::libraries::config::Settings;
use crate::libraries::graphics as gfx;
use crate::libraries::ui;
use crate::libraries::ui::{BaseUiElement, UiElement};

use super::world_creation::WorldCreationMenu;
use super::BackgroundRect;
use crate::libraries::ui::Menu;

pub const MENU_WIDTH: f32 = 800.0;

/// This function returns formatted string "%d %B %Y %H:%M" of the time
/// that the file was last modified.
pub fn get_last_modified_time(file_path: &str) -> String {
    let metadata = fs::metadata(file_path);
    let modified_time;
    if let Ok(metadata_) = metadata {
        if let Ok(modified_time_) = metadata_.modified() {
            modified_time = modified_time_;
        } else {
            modified_time = SystemTime::now();
        }
    } else {
        modified_time = SystemTime::now();
    }
    let datetime = chrono::DateTime::<chrono::Local>::from(modified_time);
    datetime.format("%d %B %Y %H:%M").to_string()
}

/// World is a struct that contains all information to
/// render the world in singleplayer selector.
pub struct World {
    pub name: String,
    pub pos: gfx::FloatPos,
    rect: ui::RenderRect,
    play_button: ui::Button,
    delete_button: ui::Button,
    last_modified: ui::Sprite,
    title: ui::Sprite,
    icon: ui::Sprite,
    file_path: PathBuf,
}

impl World {
    pub fn new(graphics: &gfx::GraphicsContext, file_path: PathBuf, button_press: Rc<RefCell<Option<(usize, usize)>>>, index: usize) -> Self {
        let stem = file_path.file_stem();
        let name = stem.map_or("incorrect_file_path", |name_| name_.to_str().unwrap_or("invalid_text_format")).to_owned();

        let mut rect = ui::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(MENU_WIDTH - 2.0 * ui::SPACING, 0.0));
        rect.orientation = ui::TOP;
        rect.fill_color.a = 100;

        let mut icon = ui::Sprite::new();
        icon.set_texture(gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/world_icon.opa")));
        rect.size.1 = icon.get_size().1 + 2.0 * ui::SPACING;
        icon.pos.0 = ui::SPACING;
        icon.orientation = ui::LEFT;

        let mut title = ui::Sprite::new();
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&name, None)));
        title.pos.0 = icon.pos.0 + icon.get_size().0 + ui::SPACING;
        title.pos.1 = ui::SPACING;
        title.scale = 3.0;

        let temp_button_press = button_press.clone();
        let mut play_button = ui::Button::new(move || {
            *temp_button_press.borrow_mut() = Some((index, 0));
        });
        play_button.texture = gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/play_button.opa"));
        play_button.scale = 3.0;
        play_button.padding = 5.0;
        play_button.pos.0 = icon.pos.0 + icon.get_size().0 + ui::SPACING;
        play_button.pos.1 = -ui::SPACING;
        play_button.orientation = ui::BOTTOM_LEFT;

        let mut delete_button = ui::Button::new(move || {
            *button_press.borrow_mut() = Some((index, 1));
        });
        delete_button.texture = gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/delete_button.opa"));
        delete_button.scale = 3.0;
        delete_button.padding = 5.0;
        delete_button.pos.0 = play_button.pos.0 + play_button.get_size().0 + ui::SPACING;
        delete_button.pos.1 = -ui::SPACING;
        delete_button.orientation = ui::BOTTOM_LEFT;

        let mut last_modified = ui::Sprite::new();
        last_modified.set_texture(gfx::Texture::load_from_surface(
            &graphics.font.create_text_surface(get_last_modified_time(file_path.as_path().to_str().unwrap_or("")).as_str(), None),
        ));
        last_modified.color = ui::GREY;
        last_modified.orientation = ui::BOTTOM_RIGHT;
        last_modified.pos.0 = -ui::SPACING;
        last_modified.pos.1 = -ui::SPACING;
        last_modified.scale = 2.0;

        Self {
            name,
            rect,
            play_button,
            delete_button,
            last_modified,
            title,
            icon,
            file_path,
            pos: gfx::FloatPos(0.0, 0.0),
        }
    }

    /// This function returns height of the world card.
    pub const fn get_height(&self) -> f32 {
        self.rect.size.1
    }

    /// This function disables/enables the world card buttons.
    pub const fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }

    const fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

impl ui::ListRow for World {
    fn get_row_height(&self) -> f32 {
        self.get_height()
    }

    fn set_row_pos(&mut self, pos: gfx::FloatPos) {
        self.pos = pos;
    }

    fn set_row_enabled(&mut self, enabled: bool) {
        self.set_enabled(enabled);
    }
}

impl UiElement for World {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.last_modified, &mut self.delete_button, &mut self.play_button, &mut self.title, &mut self.icon]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.last_modified, &self.delete_button, &self.play_button, &self.title, &self.icon]
    }

    /// This function renders the world card on the x and y position.
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        self.rect.render(graphics, parent_container);
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        self.rect.pos = self.pos;
        self.rect.update(graphics, parent_container);
    }

    /// This function returns the container of the world card.
    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        self.rect.get_container(graphics, parent_container)
    }
}

/// `WorldList` is a struct that is used to list all worlds in the world folder
/// and render them in the singleplayer selector menu.
pub struct WorldList {
    pub worlds: Vec<World>,
}

impl WorldList {
    pub fn new(graphics: &gfx::GraphicsContext, button_press: &Rc<RefCell<Option<(usize, usize)>>>) -> Self {
        let mut world_list = Self { worlds: Vec::new() };
        world_list.refresh(graphics, button_press);
        world_list
    }

    pub fn refresh(&mut self, graphics: &gfx::GraphicsContext, button_press: &Rc<RefCell<Option<(usize, usize)>>>) {
        let base_dirs;
        if let Some(base_dirs_) = BaseDirs::new() {
            base_dirs = base_dirs_;
        } else {
            return;
        }
        let world_dir = base_dirs.data_dir().join("Terralistic").join("Worlds");
        if !world_dir.exists() {
            let res = fs::create_dir_all(&world_dir);
            if res.is_err() {
                println!("could not create world dirs");
                return;
            }
        }
        self.worlds.clear();
        if let Ok(dir) = fs::read_dir(&world_dir) {
            for (index, entry) in dir.flatten().enumerate() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if !path.is_dir() && ext == "world" {
                        self.worlds.push(World::new(graphics, path, button_press.clone(), index));
                    }
                }
            }
        }
    }
}

impl UiElement for WorldList {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut element_vec: Vec<&mut dyn BaseUiElement> = Vec::new();
        for world in &mut self.worlds {
            element_vec.push(world);
        }
        element_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut element_vec: Vec<&dyn BaseUiElement> = Vec::new();
        for world in &self.worlds {
            element_vec.push(world);
        }
        element_vec
    }

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
        //this might benefit from having its own container
    }
}

pub struct SingleplayerSelector {
    world_list: WorldList,
    title: ui::Sprite,
    back_button: ui::Button,
    new_world_button: ui::Button,
    page: ui::ListPage,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
    new_world_press: Rc<RefCell<bool>>,
    world_button_press: Rc<RefCell<Option<(usize, usize)>>>,
    close_self: bool,
    open_menu: Option<(Box<dyn Menu>, String)>,
}

impl SingleplayerSelector {
    #[must_use]
    pub fn new(graphics: &gfx::GraphicsContext, settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Self {
        let world_button_press = Rc::new(RefCell::new(None));
        let world_list = WorldList::new(graphics, &world_button_press);
        let mut title = ui::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Select a world to play!", None)));
        title.pos.1 = ui::SPACING;
        title.orientation = ui::TOP;

        let mut back_button = ui::Button::new(|| {});
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.pos.1 = -ui::SPACING;
        back_button.orientation = ui::BOTTOM;

        let new_world_press = Rc::new(RefCell::new(false));
        let temp_new_world_press = new_world_press.clone();
        let mut new_world_button = ui::Button::new(move || {
            *temp_new_world_press.borrow_mut() = true;
        });
        new_world_button.scale = 3.0;
        new_world_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New", None));
        new_world_button.pos.0 = -ui::SPACING;
        new_world_button.pos.1 = -ui::SPACING;
        new_world_button.orientation = ui::BOTTOM_RIGHT;

        let top_height = title.get_size().1 + 2.0 * ui::SPACING;
        let bottom_height = back_button.get_size().1 + 2.0 * ui::SPACING;

        Self {
            world_list,
            title,
            back_button,
            new_world_button,
            page: ui::ListPage::new(MENU_WIDTH, top_height, bottom_height),
            settings,
            global_settings,
            new_world_press,
            world_button_press,
            close_self: false,
            open_menu: None,
        }
    }

    fn do_world_action(&mut self, graphics: &mut gfx::GraphicsContext, world: usize, action: usize, parent_container: &ui::Container) -> Option<()> {
        if action == 0 {
            let mut menu_back = super::MenuBack::new(graphics);
            menu_back.set_back_rect_width(parent_container.rect.size.0, false);
            menu_back.update(graphics, &ui::Container::default(graphics));
            menu_back.render_back(graphics);
            if let Ok(menu) = PrivateWorld::new(self.world_list.worlds.get(world)?.get_file_path(), self.settings.clone(), self.global_settings.clone()) {
                self.open_menu = Some((Box::new(menu), "f LoadingScreen".to_owned()));
            }
        } else if action == 1 {
            let path = self.world_list.worlds.get(world)?.get_file_path().clone();
            let menu = ChoiceMenu::new(
                &format!("The world \"{}\" will be deleted.\nDo you want to proceed?", self.world_list.worlds.get(world)?.name),
                graphics,
                vec![
                    ("Back", Box::new(|| {})),
                    (
                        "Proceed",
                        Box::new(move || {
                            if let Err(e) = fs::remove_file(path.clone()) {
                                println!("{e}");
                            }
                        }),
                    ),
                ],
                Some(0),
                Some(1),
            );
            self.open_menu = Some((Box::new(menu), "DeleteWorld".to_owned()));
        }

        Some(())
    }
}

impl UiElement for SingleplayerSelector {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = Vec::new();
        let (top_visible, scrollable) = (self.page.is_top_rect_visible(), self.page.is_scrollable());
        elements_vec.push(&mut self.world_list);
        if top_visible {
            elements_vec.push(&mut self.page.top_rect);
        }
        if scrollable {
            elements_vec.push(&mut self.page.bottom_rect);
        }
        elements_vec.push(&mut self.title);
        elements_vec.push(&mut self.back_button);
        elements_vec.push(&mut self.new_world_button);
        elements_vec.push(&mut self.page.scrollable);
        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = Vec::new();
        elements_vec.push(&self.world_list);
        if self.page.is_top_rect_visible() {
            elements_vec.push(&self.page.top_rect);
        }
        if self.page.is_scrollable() {
            elements_vec.push(&self.page.bottom_rect);
        }
        elements_vec.push(&self.back_button);
        elements_vec.push(&self.new_world_button);
        elements_vec.push(&self.page.scrollable);
        elements_vec.push(&self.title);
        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        if *self.new_world_press.borrow_mut() {
            let mut names_vec = Vec::new();
            for world in &self.world_list.worlds {
                names_vec.push(world.name.clone());
            }
            if let Ok(world_creation_menu) = WorldCreationMenu::new(graphics, names_vec, self.settings.clone(), self.global_settings.clone()) {
                self.open_menu = Some((Box::new(world_creation_menu), "CreateWorld".to_owned()));
            }
        }
        *self.new_world_press.borrow_mut() = false;

        let mut rows: Vec<&mut dyn ui::ListRow> = self.world_list.worlds.iter_mut().map(|world| world as &mut dyn ui::ListRow).collect();
        self.page.update(graphics, parent_container, &mut rows);

        let res = *self.world_button_press.borrow_mut();
        if let Some((world, action)) = res {
            self.do_world_action(graphics, world, action, parent_container);
        }
        *self.world_button_press.borrow_mut() = None;
    }

    fn on_event_inner(&mut self, graphics: &mut dyn ui::UiContext, event: &gfx::Event, parent_container: &ui::Container) -> bool {
        if let gfx::Event::KeyRelease(key, ..) = event {
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

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

impl Menu for SingleplayerSelector {
    fn should_close(&mut self) -> bool {
        let ret_val = self.close_self;
        self.close_self = false;
        ret_val
    }
    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        self.open_menu.take()
    }

    fn on_focus(&mut self, graphics: &gfx::GraphicsContext) {
        self.world_list.refresh(graphics, &self.world_button_press);
    }
}
