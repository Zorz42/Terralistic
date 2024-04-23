use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::SystemTime;

use directories::BaseDirs;

use crate::client::global_settings::GlobalSettings;
use crate::client::menus::choice_menu::ChoiceMenu;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};

use super::world_creation::WorldCreationMenu;
use super::{BackgroundRect, Menu};

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
    rect: gfx::RenderRect,
    play_button: gfx::Button,
    delete_button: gfx::Button,
    last_modified: gfx::Sprite,
    title: gfx::Sprite,
    icon: gfx::Sprite,
    file_path: PathBuf,
}

impl World {
    pub fn new(graphics: &gfx::GraphicsContext, file_path: PathBuf, button_press: Rc<RefCell<Option<(usize, usize)>>>, index: usize) -> Self {
        let stem = file_path.file_stem();
        let name = stem.map_or("incorrect_file_path", |name_| name_.to_str().unwrap_or("invalid_text_format")).to_owned();

        let mut rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(MENU_WIDTH - 2.0 * gfx::SPACING, 0.0));
        rect.orientation = gfx::TOP;
        rect.fill_color.a = 100;

        let mut icon = gfx::Sprite::new();
        icon.set_texture(gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/world_icon.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))),
        ));
        rect.size.1 = icon.get_size().1 + 2.0 * gfx::SPACING;
        icon.pos.0 = gfx::SPACING;
        icon.orientation = gfx::LEFT;

        let mut title = gfx::Sprite::new();
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&name, None)));
        title.pos.0 = icon.pos.0 + icon.get_size().0 + gfx::SPACING;
        title.pos.1 = gfx::SPACING;
        title.scale = 3.0;

        let temp_button_press = button_press.clone();
        let mut play_button = gfx::Button::new(move || {
            *temp_button_press.borrow_mut() = Some((index, 0));
        });
        play_button.texture =
            gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/play_button.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))));
        play_button.scale = 3.0;
        play_button.padding = 5.0;
        play_button.pos.0 = icon.pos.0 + icon.get_size().0 + gfx::SPACING;
        play_button.pos.1 = -gfx::SPACING;
        play_button.orientation = gfx::BOTTOM_LEFT;

        let mut delete_button = gfx::Button::new(move || {
            *button_press.borrow_mut() = Some((index, 1));
        });
        delete_button.texture =
            gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/delete_button.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))));
        delete_button.scale = 3.0;
        delete_button.padding = 5.0;
        delete_button.pos.0 = play_button.pos.0 + play_button.get_size().0 + gfx::SPACING;
        delete_button.pos.1 = -gfx::SPACING;
        delete_button.orientation = gfx::BOTTOM_LEFT;

        let mut last_modified = gfx::Sprite::new();
        last_modified.set_texture(gfx::Texture::load_from_surface(
            &graphics.font.create_text_surface(get_last_modified_time(file_path.as_path().to_str().unwrap_or("")).as_str(), None),
        ));
        last_modified.color = gfx::GREY;
        last_modified.orientation = gfx::BOTTOM_RIGHT;
        last_modified.pos.0 = -gfx::SPACING;
        last_modified.pos.1 = -gfx::SPACING;
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
    pub fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }

    const fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

impl UiElement for World {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.last_modified, &mut self.delete_button, &mut self.play_button, &mut self.title, &mut self.icon]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.last_modified, &self.delete_button, &self.play_button, &self.title, &self.icon]
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.rect.pos = self.pos;
        self.rect.update(graphics, parent_container);
    }

    /// This function renders the world card on the x and y position.
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.rect.render(graphics, parent_container);
    }

    /// This function returns the container of the world card.
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        self.rect.get_container(graphics, parent_container)
    }
}

/// `WorldList` is a struct that is used to list all worlds in the world folder
/// and render them in the singleplayer selector menu.
pub struct WorldList {
    pub worlds: Vec<World>,
    pub scrolled: f32,
    pub top_rect_size: f32,
}

impl WorldList {
    pub fn new(graphics: &gfx::GraphicsContext, button_press: &Rc<RefCell<Option<(usize, usize)>>>) -> Self {
        let mut world_list = Self {
            worlds: Vec::new(),
            scrolled: 0.0,
            top_rect_size: 0.0,
        };
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

    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &gfx::Container) {
        let mut current_y = gfx::SPACING + self.scrolled + self.top_rect_size;
        for world in &mut self.worlds {
            world.pos = gfx::FloatPos(0.0, current_y);
            current_y += world.get_height() + gfx::SPACING;
        }
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
        //this might benefit from having its own container
    }
}

pub struct SingleplayerSelector {
    world_list: WorldList,
    title: gfx::Sprite,
    back_button: gfx::Button,
    new_world_button: gfx::Button,
    top_rect: gfx::RenderRect,
    bottom_rect: gfx::RenderRect,
    scrollable: gfx::Scrollable,
    top_rect_visibility: f32,
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
        let mut title = gfx::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Select a world to play!", None)));
        title.pos.1 = gfx::SPACING;
        title.orientation = gfx::TOP;

        let mut back_button = gfx::Button::new(|| {});
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.pos.1 = -gfx::SPACING;
        back_button.orientation = gfx::BOTTOM;

        let new_world_press = Rc::new(RefCell::new(false));
        let temp_new_world_press = new_world_press.clone();
        let mut new_world_button = gfx::Button::new(move || {
            *temp_new_world_press.borrow_mut() = true;
        });
        new_world_button.scale = 3.0;
        new_world_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New", None));
        new_world_button.pos.0 = -gfx::SPACING;
        new_world_button.pos.1 = -gfx::SPACING;
        new_world_button.orientation = gfx::BOTTOM_RIGHT;

        let top_height = title.get_size().1 + 2.0 * gfx::SPACING;
        let bottom_height = back_button.get_size().1 + 2.0 * gfx::SPACING;

        let mut top_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, top_height));
        top_rect.orientation = gfx::TOP;

        let mut bottom_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, bottom_height));
        bottom_rect.fill_color.a = gfx::TRANSPARENCY / 2;
        bottom_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        bottom_rect.blur_radius = gfx::BLUR;
        bottom_rect.orientation = gfx::BOTTOM;

        let mut scrollable = gfx::Scrollable::new();
        scrollable.rect.pos.1 = gfx::SPACING;
        scrollable.rect.size.0 = MENU_WIDTH;
        scrollable.scroll_smooth_factor = 100.0;
        scrollable.boundary_smooth_factor = 40.0;
        scrollable.orientation = gfx::TOP;

        Self {
            world_list,
            title,
            back_button,
            new_world_button,
            top_rect,
            bottom_rect,
            scrollable,
            top_rect_visibility: 0.0,
            settings,
            global_settings,
            new_world_press,
            world_button_press,
            close_self: false,
            open_menu: None,
        }
    }

    fn do_world_action(&mut self, graphics: &mut gfx::GraphicsContext, world: usize, action: usize, parent_container: &gfx::Container) -> Option<()> {
        if action == 0 {
            let mut menu_back = super::MenuBack::new(graphics);
            menu_back.set_back_rect_width(parent_container.rect.size.0, false);
            menu_back.update(graphics, &gfx::Container::default(graphics));
            menu_back.render_back(graphics);
            //let game_result = run_private_world(graphics, &mut menu_back, self.world_list.worlds.get(world)?.get_file_path(), &self.settings, &self.global_settings);
            /*if let Err(error) = game_result {
                println!("Game error: {error}");
                let menu = ChoiceMenu::new(&format!("Game error: {error}"), graphics, vec![("Ok", Box::new(|| {}))], Some(0), Some(0));
                self.open_menu = Some(Box::new(menu));
            }*/
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
        elements_vec.push(&mut self.world_list);
        if self.top_rect_visibility > 0.0 {
            elements_vec.push(&mut self.top_rect);
        }
        if self.scrollable.scroll_size > self.scrollable.rect.size.1 {
            elements_vec.push(&mut self.bottom_rect);
        }
        elements_vec.push(&mut self.title);
        elements_vec.push(&mut self.back_button);
        elements_vec.push(&mut self.new_world_button);
        elements_vec.push(&mut self.scrollable);
        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = Vec::new();
        elements_vec.push(&self.world_list);
        if self.top_rect_visibility > 0.0 {
            elements_vec.push(&self.top_rect);
        }
        if self.scrollable.scroll_size > self.scrollable.rect.size.1 {
            elements_vec.push(&self.bottom_rect);
        }
        elements_vec.push(&self.back_button);
        elements_vec.push(&self.new_world_button);
        elements_vec.push(&self.scrollable);
        elements_vec.push(&self.title);
        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
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

        let hoverable = graphics.get_mouse_pos().1 > self.top_rect.size.1 && graphics.get_mouse_pos().1 < graphics.get_window_size().1 - self.bottom_rect.size.1;

        for world in &mut self.world_list.worlds {
            world.set_enabled(hoverable);
        }

        self.world_list.scrolled = self.scrollable.get_scroll_x(graphics, parent_container);
        self.world_list.top_rect_size = self.top_rect.size.1;

        self.top_rect.size.0 = parent_container.get_absolute_rect().size.0;

        self.top_rect_visibility += ((if self.scrollable.get_scroll_pos() > 5.0 { 1.0 } else { 0.0 }) - self.top_rect_visibility) / 20.0;

        if self.top_rect_visibility < 0.01 {
            self.top_rect_visibility = 0.0;
        }

        if self.top_rect_visibility > 0.99 {
            self.top_rect_visibility = 1.0;
        }

        self.top_rect.fill_color.a = (self.top_rect_visibility * gfx::TRANSPARENCY as f32 / 2.0) as u8;
        self.top_rect.blur_radius = (self.top_rect_visibility * gfx::BLUR as f32) as i32;
        self.top_rect.shadow_intensity = (self.top_rect_visibility * gfx::SHADOW_INTENSITY as f32) as i32;

        self.bottom_rect.size.0 = parent_container.get_absolute_rect().size.0;

        let mut world_height = 0.0;
        if let Some(world) = self.world_list.worlds.first() {
            world_height = world.get_height();
        }
        self.scrollable.scroll_size = (world_height + gfx::SPACING) * self.world_list.worlds.len() as f32 - gfx::SPACING;
        self.scrollable.rect.size.1 = graphics.get_window_size().1 - self.top_rect.size.1 - self.bottom_rect.size.1;

        let res = *self.world_button_press.borrow_mut();
        if let Some((world, action)) = res {
            self.do_world_action(graphics, world, action, parent_container);
        }
        *self.world_button_press.borrow_mut() = None;
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
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

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

impl super::menu::Menu for SingleplayerSelector {
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
