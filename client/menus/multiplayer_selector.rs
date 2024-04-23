use crate::client::menus::choice_menu::ChoiceMenu;
use anyhow::Result;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::client::game::core_client::run_game;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::run_text_input_menu;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;

use super::{AddServerMenu, Menu};

use super::background_rect::BackgroundRect;
use gfx::{BaseUiElement, UiElement};

pub const MENU_WIDTH: f32 = 800.0;

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    ip: String,
    port: u16,
}

impl ServerInfo {
    pub const fn new(name: String, ip: String, port: u16) -> Self {
        Self { name, ip, port }
    }
}

/// struct to pass around UI elements for rendering and updating
pub struct MultiplayerSelector {
    top_rect: gfx::RenderRect,
    bottom_rect: gfx::RenderRect,
    title: gfx::Sprite,
    back_button: gfx::Button,
    new_server_button: gfx::Button,
    server_list: ServerList,
    top_height: f32,
    bottom_height: f32,
    servers_file: PathBuf,
    scrollable: gfx::Scrollable,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
    top_rect_visibility: f32,
    close_self: bool,
    open_menu: Option<(Box<dyn Menu>, String)>,
}

impl MultiplayerSelector {
    pub fn new(graphics: &gfx::GraphicsContext, settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Result<Self> {
        let base_dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("error getting base dirs"))?;
        let servers_file = base_dirs.data_dir().join("Terralistic").join("servers.txt");

        let server_list = ServerList::new(graphics, servers_file.clone());

        let mut title = gfx::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Select a server to play!", None)));
        title.pos.1 = gfx::SPACING;
        title.orientation = gfx::TOP;

        let mut back_button = gfx::Button::new(|| {});
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.pos.1 = -gfx::SPACING;
        back_button.orientation = gfx::BOTTOM;

        let mut new_server_button = gfx::Button::new(|| {});
        new_server_button.scale = 3.0;
        new_server_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New", None));
        new_server_button.pos.1 = -gfx::SPACING;
        new_server_button.pos.0 = -gfx::SPACING;
        new_server_button.orientation = gfx::BOTTOM_RIGHT;

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

        Ok(Self {
            top_rect,
            bottom_rect,
            title,
            back_button,
            new_server_button,
            server_list,
            top_height,
            bottom_height,
            servers_file,
            scrollable,
            settings,
            global_settings,
            top_rect_visibility: 0.0,
            open_menu: None,
            close_self: false,
        })
    }

    fn update_server_list(&mut self, graphics: &gfx::GraphicsContext) {
        let hoverable = graphics.get_mouse_pos().1 > self.top_height && graphics.get_mouse_pos().1 < graphics.get_window_size().1 - self.bottom_height;

        for server in &mut self.server_list.servers {
            server.set_enabled(hoverable);
        }
    }

    fn update_top_bottom_rects(&mut self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container, elements_height: f32) {
        self.top_rect.size.0 = parent_container.rect.size.0;

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

        self.bottom_rect.size.0 = parent_container.rect.size.0;

        self.scrollable.scroll_size = elements_height;
        self.scrollable.rect.size.1 = graphics.get_window_size().1 - self.top_height - self.bottom_height;
    }
}

impl UiElement for MultiplayerSelector {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = Vec::new();
        elements_vec.push(&mut self.server_list);
        if self.scrollable.scroll_size > self.scrollable.rect.size.1 {
            elements_vec.push(&mut self.top_rect);
        }
        if self.scrollable.scroll_size > self.scrollable.rect.size.1 {
            elements_vec.push(&mut self.bottom_rect);
        }
        elements_vec.push(&mut self.title);
        elements_vec.push(&mut self.back_button);
        elements_vec.push(&mut self.new_server_button);
        elements_vec.push(&mut self.scrollable);

        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = Vec::new();
        elements_vec.push(&self.server_list);
        if self.scrollable.scroll_size > self.scrollable.rect.size.1 {
            elements_vec.push(&self.top_rect);
        }
        if self.scrollable.scroll_size > self.scrollable.rect.size.1 {
            elements_vec.push(&self.bottom_rect);
        }
        elements_vec.push(&self.title);
        elements_vec.push(&self.back_button);
        elements_vec.push(&self.new_server_button);
        elements_vec.push(&self.scrollable);

        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.update_server_list(graphics);

        let mut server_height = 0.0;
        if let Some(server) = self.server_list.servers.first() {
            server_height = server.get_height();
        }

        let elements_height = (server_height + gfx::SPACING) * self.server_list.servers.len() as f32 - gfx::SPACING;

        self.update_top_bottom_rects(graphics, parent_container, elements_height);
        self.server_list.scrolled = self.scrollable.get_scroll_x(graphics, parent_container);
        self.server_list.top_rect_size = self.top_rect.size.1;
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        let inner_container = self.get_container(graphics, parent_container);
        if self.back_button.on_event(graphics, event, parent_container) {
            self.close_self = true;
            return true;
        }
        if let gfx::Event::KeyRelease(key, ..) = event {
            if key == &gfx::Key::Escape {
                self.close_self = true;
                return true;
            }
        }
        if self.new_server_button.on_event(graphics, event, &inner_container) {
            self.open_menu = Some((Box::new(AddServerMenu::new(graphics, self.servers_file.clone())), "AddServer".to_owned()));
        }

        for server in &mut self.server_list.servers {
            if server.play_button.on_event(graphics, event, &server.get_container(graphics, &inner_container)) {
                let mut menu_back = super::MenuBack::new(graphics);
                menu_back.set_back_rect_width(parent_container.rect.size.0, false);
                menu_back.update(graphics, &gfx::Container::default(graphics));
                menu_back.render_back(graphics);
                let name = run_text_input_menu("Enter your name", graphics, &mut menu_back);
                if let Some(name) = name {
                    let game_result = run_game(graphics, server.server_info.port, server.server_info.ip.clone(), &name, &self.settings, &self.global_settings);
                    if let Err(error) = game_result {
                        println!("Game error: {error}");
                        self.open_menu = Some((
                            Box::new(ChoiceMenu::new(&format!("Game error: {error}"), graphics, vec![("Ok", Box::new(|| {}))], None, None)),
                            "GameError".to_owned(),
                        ));
                    }
                }
            }
            if server.delete_button.on_event(graphics, event, &server.get_container(graphics, &inner_container)) {
                let path = self.servers_file.clone();
                let name_to_delete = server.server_info.name.clone();
                self.open_menu = Some((
                    Box::new(ChoiceMenu::new(
                        format!("The server \"{}\" will be deleted.\nDo you want to proceed?", server.server_info.name).as_str(),
                        graphics,
                        vec![("Back", Box::new(|| {})), ("Proceed", Box::new(move || remove_server_by_name(&name_to_delete.clone(), path.clone())))],
                        Some(0),
                        Some(1),
                    )),
                    "DeleteServer".to_owned(),
                ));
            }
        }
        false
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

fn remove_server_by_name(name: &str, file_path: PathBuf) {
    let file = std::fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());
    let mut temp_servers: Vec<ServerInfo> = serde_json::from_str(&file).unwrap_or_else(|_| Vec::new());
    temp_servers.retain(|server| server.name != name);
    let serialized = serde_json::to_string(&temp_servers).unwrap_or_default();
    let _result = std::fs::write(file_path, serialized);
}

impl super::Menu for MultiplayerSelector {
    fn should_close(&mut self) -> bool {
        let ret_val = self.close_self;
        self.close_self = false;
        ret_val
    }

    fn open_menu(&mut self, _: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        self.open_menu.take()
    }

    fn on_focus(&mut self, graphics: &gfx::GraphicsContext) {
        self.server_list.refresh(graphics, self.servers_file.clone());
    }
}

/// `ServerCard` is a struct that contains all information to
/// render the server in server selector.
pub struct ServerCard {
    pub server_info: ServerInfo,
    rect: gfx::RenderRect,
    play_button: gfx::Button,
    delete_button: gfx::Button,
    title: gfx::Sprite,
    icon: gfx::Sprite,
}

impl ServerCard {
    pub fn new(graphics: &gfx::GraphicsContext, name: String, ip: String, port: u16) -> Self {
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
        title.pos.0 = icon.pos.0 + icon.get_size().1 + gfx::SPACING;
        title.pos.1 = gfx::SPACING;
        title.scale = 3.0;

        let mut play_button = gfx::Button::new(|| {});
        play_button.texture =
            gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/join_button.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))));
        play_button.scale = 3.0;
        play_button.padding = 5.0;
        play_button.pos.0 = icon.pos.0 + icon.get_size().0 + gfx::SPACING;
        play_button.pos.1 = -gfx::SPACING;
        play_button.orientation = gfx::BOTTOM_LEFT;

        let mut delete_button = gfx::Button::new(|| {});
        delete_button.texture =
            gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/remove_button.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))));
        delete_button.scale = 3.0;
        delete_button.padding = 5.0;
        delete_button.pos.0 = play_button.pos.0 + play_button.get_size().0 + gfx::SPACING;
        delete_button.pos.1 = -gfx::SPACING;
        delete_button.orientation = gfx::BOTTOM_LEFT;

        Self {
            server_info: ServerInfo::new(name, ip, port),
            rect,
            play_button,
            delete_button,
            title,
            icon,
        }
    }

    /// This function returns height of the server card.
    pub const fn get_height(&self) -> f32 {
        self.rect.size.1
    }

    /// This function disables/enables the server card buttons.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }

    pub fn set_pos(&mut self, pos: gfx::FloatPos) {
        self.rect.pos = pos;
    }
}

impl UiElement for ServerCard {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.icon, &mut self.title, &mut self.play_button, &mut self.delete_button]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.icon, &self.title, &self.play_button, &self.delete_button]
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.rect.update(graphics, parent_container);
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.rect.render(graphics, parent_container);
    }

    /// This function returns the container of the server card.
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        self.rect.get_container(graphics, parent_container)
    }
}

/// `ServerList` is a struct that is used to list all servers in the server folder
/// and render them in the singleplayer selector menu.
pub struct ServerList {
    pub servers: Vec<ServerCard>,
    pub scrolled: f32,
    pub top_rect_size: f32,
}

impl ServerList {
    pub fn new(graphics: &gfx::GraphicsContext, file_path: PathBuf) -> Self {
        let mut server_list = Self {
            servers: Vec::new(),
            scrolled: 0.0,
            top_rect_size: 0.0,
        };
        server_list.refresh(graphics, file_path);
        server_list
    }

    pub fn refresh(&mut self, graphics: &gfx::GraphicsContext, file_path: PathBuf) {
        let temp_servers: Vec<ServerInfo>;

        if file_path.exists() {
            let file = std::fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
            temp_servers = serde_json::from_str(&file).unwrap_or_else(|_| Vec::new());
        } else {
            temp_servers = Vec::new();
            let serial = serde_json::to_string(&temp_servers).unwrap_or_default();
            let res = std::fs::write(file_path, serial);
            if res.is_err() {
                println!("Failed to create a server file!");
            }
        }

        self.servers.clear();
        for server in temp_servers {
            self.servers.push(ServerCard::new(graphics, server.name, server.ip, server.port));
        }
    }
}

impl UiElement for ServerList {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = Vec::new();
        for element in &mut self.servers {
            elements_vec.push(element);
        }
        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = Vec::new();
        for element in &self.servers {
            elements_vec.push(element);
        }
        elements_vec
    }

    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &gfx::Container) {
        let mut current_y = gfx::SPACING + self.scrolled + self.top_rect_size;
        for server in &mut self.servers {
            server.set_pos(gfx::FloatPos(0.0, current_y));
            current_y += server.get_height() + gfx::SPACING;
        }
    }

    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}
