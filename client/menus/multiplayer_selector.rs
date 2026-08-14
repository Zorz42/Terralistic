use crate::client::menus::choice_menu::ChoiceMenu;
use anyhow::Result;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::client::global_settings::GlobalSettings;
use crate::libraries::config::Settings;
use crate::libraries::graphics as gfx;
use crate::libraries::ui;

use super::{AddServerMenu, StartMultiplayer};
use crate::libraries::ui::Menu;

use super::background_rect::BackgroundRect;
use crate::libraries::ui::{BaseUiElement, UiElement};

pub const MENU_WIDTH: f32 = 800.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

impl ServerInfo {
    pub const fn new(name: String, ip: String, port: u16) -> Self {
        Self { name, ip, port }
    }
}

/// struct to pass around UI elements for rendering and updating
pub struct MultiplayerSelector {
    page: ui::ListPage,
    title: ui::Sprite,
    back_button: ui::Button,
    new_server_button: ui::Button,
    server_list: ServerList,
    servers_file: PathBuf,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
    close_self: bool,
    open_menu: Option<(Box<dyn Menu>, String)>,
}

impl MultiplayerSelector {
    pub fn new(graphics: &gfx::GraphicsContext, settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Result<Self> {
        let base_dirs = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("error getting base dirs"))?;
        let servers_file = base_dirs.data_dir().join("Terralistic").join("servers.txt");

        let server_list = ServerList::new(graphics, servers_file.clone());

        let mut title = ui::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Select a server to play!", None)));
        title.pos.1 = ui::SPACING;
        title.orientation = ui::TOP;

        let mut back_button = ui::Button::new(|| {});
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.pos.1 = -ui::SPACING;
        back_button.orientation = ui::BOTTOM;

        let mut new_server_button = ui::Button::new(|| {});
        new_server_button.scale = 3.0;
        new_server_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New", None));
        new_server_button.pos.1 = -ui::SPACING;
        new_server_button.pos.0 = -ui::SPACING;
        new_server_button.orientation = ui::BOTTOM_RIGHT;

        let top_height = title.get_size().1 + 2.0 * ui::SPACING;
        let bottom_height = back_button.get_size().1 + 2.0 * ui::SPACING;

        Ok(Self {
            page: ui::ListPage::new(MENU_WIDTH, top_height, bottom_height),
            title,
            back_button,
            new_server_button,
            server_list,
            servers_file,
            settings,
            global_settings,
            open_menu: None,
            close_self: false,
        })
    }
}

impl UiElement for MultiplayerSelector {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = Vec::new();
        let scrollable = self.page.is_scrollable();
        elements_vec.push(&mut self.server_list);
        if scrollable {
            elements_vec.push(&mut self.page.top_rect);
        }
        if scrollable {
            elements_vec.push(&mut self.page.bottom_rect);
        }
        elements_vec.push(&mut self.title);
        elements_vec.push(&mut self.back_button);
        elements_vec.push(&mut self.new_server_button);
        elements_vec.push(&mut self.page.scrollable);

        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = Vec::new();
        let scrollable = self.page.is_scrollable();
        elements_vec.push(&self.server_list);
        if scrollable {
            elements_vec.push(&self.page.top_rect);
        }
        if scrollable {
            elements_vec.push(&self.page.bottom_rect);
        }
        elements_vec.push(&self.title);
        elements_vec.push(&self.back_button);
        elements_vec.push(&self.new_server_button);
        elements_vec.push(&self.page.scrollable);

        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        let mut rows: Vec<&mut dyn ui::ListRow> = self.server_list.servers.iter_mut().map(|server| server as &mut dyn ui::ListRow).collect();
        self.page.update(graphics, parent_container, &mut rows);
    }

    fn on_event_inner(&mut self, graphics: &mut dyn ui::UiContext, event: &gfx::Event, parent_container: &ui::Container) -> bool {
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
            // Opening a menu builds its text textures, so it only happens with a real
            // graphics context. See `UiContext::as_graphics_context`.
            if let Some(graphics) = graphics.as_graphics_context() {
                self.open_menu = Some((Box::new(AddServerMenu::new(graphics, self.servers_file.clone())), "AddServer".to_owned()));
            }
        }

        for server in &mut self.server_list.servers {
            let server_container = server.get_container(graphics, &inner_container);
            if server.play_button.on_event(graphics, event, &server_container) {
                if let Some(graphics) = graphics.as_graphics_context() {
                    let mut menu_back = super::MenuBack::new(graphics);
                    menu_back.set_back_rect_width(parent_container.rect.size.0, false);
                    let default_container = ui::Container::default(graphics);
                    menu_back.update(graphics, &default_container);
                    menu_back.render_back(graphics);
                }
                self.open_menu = Some((
                    Box::new(StartMultiplayer::new(server.server_info.clone(), self.settings.clone(), self.global_settings.clone())),
                    "f StartMultiplayer".to_owned(),
                ));
            }
            if server.delete_button.on_event(graphics, event, &server_container) {
                let path = self.servers_file.clone();
                let name_to_delete = server.server_info.name.clone();
                if let Some(graphics) = graphics.as_graphics_context() {
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
        }
        false
    }

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

fn remove_server_by_name(name: &str, file_path: PathBuf) {
    let file = std::fs::read_to_string(&file_path).unwrap_or_else(|_| String::new());
    let mut temp_servers: Vec<ServerInfo> = serde_json::from_str(&file).unwrap_or_else(|_| Vec::new());
    temp_servers.retain(|server| server.name != name);
    let serialized = serde_json::to_string(&temp_servers).unwrap_or_default();
    let _result = std::fs::write(file_path, serialized);
}

impl Menu for MultiplayerSelector {
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
    rect: ui::RenderRect,
    play_button: ui::Button,
    delete_button: ui::Button,
    title: ui::Sprite,
    icon: ui::Sprite,
}

impl ServerCard {
    pub fn new(graphics: &gfx::GraphicsContext, name: String, ip: String, port: u16) -> Self {
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
        title.pos.0 = icon.pos.0 + icon.get_size().1 + ui::SPACING;
        title.pos.1 = ui::SPACING;
        title.scale = 3.0;

        let mut play_button = ui::Button::new(|| {});
        play_button.texture = gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/join_button.opa"));
        play_button.scale = 3.0;
        play_button.padding = 5.0;
        play_button.pos.0 = icon.pos.0 + icon.get_size().0 + ui::SPACING;
        play_button.pos.1 = -ui::SPACING;
        play_button.orientation = ui::BOTTOM_LEFT;

        let mut delete_button = ui::Button::new(|| {});
        delete_button.texture = gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/remove_button.opa"));
        delete_button.scale = 3.0;
        delete_button.padding = 5.0;
        delete_button.pos.0 = play_button.pos.0 + play_button.get_size().0 + ui::SPACING;
        delete_button.pos.1 = -ui::SPACING;
        delete_button.orientation = ui::BOTTOM_LEFT;

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
    pub const fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }

    pub const fn set_pos(&mut self, pos: gfx::FloatPos) {
        self.rect.pos = pos;
    }
}

impl ui::ListRow for ServerCard {
    fn get_row_height(&self) -> f32 {
        self.get_height()
    }

    fn set_row_pos(&mut self, pos: gfx::FloatPos) {
        self.set_pos(pos);
    }

    fn set_row_enabled(&mut self, enabled: bool) {
        self.set_enabled(enabled);
    }
}

impl UiElement for ServerCard {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.icon, &mut self.title, &mut self.play_button, &mut self.delete_button]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.icon, &self.title, &self.play_button, &self.delete_button]
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        self.rect.render(graphics, parent_container);
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        self.rect.update(graphics, parent_container);
    }

    /// This function returns the container of the server card.
    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        self.rect.get_container(graphics, parent_container)
    }
}

/// `ServerList` is a struct that is used to list all servers in the server folder
/// and render them in the singleplayer selector menu.
pub struct ServerList {
    pub servers: Vec<ServerCard>,
}

impl ServerList {
    pub fn new(graphics: &gfx::GraphicsContext, file_path: PathBuf) -> Self {
        let mut server_list = Self { servers: Vec::new() };
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

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}
