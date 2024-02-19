use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use crate::client::game::core_client::run_game;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::run_text_input_menu;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;

use super::background_rect::BackgroundRect;
use super::{run_add_server_menu, run_choice_menu};
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
struct MultiplayerSelectorElements {
    top_rect: gfx::RenderRect,
    bottom_rect: gfx::RenderRect,
    title: gfx::Sprite,
    back_button: gfx::Button,
    new_server_button: gfx::Button,
    server_list: ServerList,
    top_height: f32,
    bottom_height: f32,
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
        icon.texture =
            gfx::Texture::load_from_surface(&gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/world_icon.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))));
        rect.size.1 = icon.get_size().1 + 2.0 * gfx::SPACING;
        icon.pos.0 = gfx::SPACING;
        icon.orientation = gfx::LEFT;

        let mut title = gfx::Sprite::new();
        title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&name, None));
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

    /// This function renders the server card on the x and y position.
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, pos: gfx::FloatPos, parent_container: Option<&gfx::Container>) {
        self.rect.pos = pos;
        /*self.rect.render(graphics, parent_container);

        let rect_container = self.rect.get_container(graphics, parent_container);
        self.icon.render(graphics, Some(&rect_container), None);
        self.title.render(graphics, Some(&rect_container), None);
        self.play_button.render(graphics, Some(&rect_container));
        self.delete_button.render(graphics, Some(&rect_container));*/
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

    /// This function returns the container of the server card.
    pub fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> gfx::Container {
        //self.rect.get_container(graphics, parent_container)
        gfx::Container::default(graphics)
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

    pub fn save(&self, file_path: PathBuf) {
        let server_infos: Vec<ServerInfo> = self
            .servers
            .iter()
            .map(|server| ServerInfo::new(server.server_info.name.clone(), server.server_info.ip.clone(), server.server_info.port))
            .collect();

        let res = std::fs::write(file_path, serde_json::to_string(&server_infos).unwrap_or_default());
        if res.is_err() {
            println!("Failed to save servers!");
        }
    }
}

pub fn run_multiplayer_selector(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect, settings: &Rc<RefCell<Settings>>, global_settings: &Rc<RefCell<GlobalSettings>>) {
    let Some(base_dirs) = BaseDirs::new() else {
        println!("Failed to get base directories!");
        return;
    };
    let servers_file = base_dirs.data_dir().join("Terralistic").join("servers.txt");

    let server_list = ServerList::new(graphics, servers_file.clone());

    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Select a server to play!", None));
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

    let mut elements = MultiplayerSelectorElements {
        top_rect,
        bottom_rect,
        title,
        back_button,
        new_server_button,
        server_list,
        top_height,
        bottom_height,
    };

    let mut top_rect_visibility = 1.0;
    while graphics.is_window_open() {
        //update_elements returns true if the loop is to be broken
        if update_elements(graphics, menu_back, &mut elements, &servers_file, settings, global_settings, &mut scrollable) {
            break;
        }
        render_elements(graphics, menu_back, &mut elements, &mut top_rect_visibility, &mut scrollable);
    }
}

fn update_elements(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    elements: &mut MultiplayerSelectorElements,
    servers_file: &Path,
    settings: &Rc<RefCell<Settings>>,
    global_settings: &Rc<RefCell<GlobalSettings>>,
    scrollable: &mut gfx::Scrollable,
) -> bool {
    while let Some(event) = graphics.get_event() {
        scrollable.on_event(graphics, &event, menu_back.get_back_rect_container());
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::MouseLeft => {
                    if elements.back_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        return true;
                    }
                    if elements.new_server_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        if let Some(server) = run_add_server_menu(graphics, menu_back, &elements.server_list.servers) {
                            elements.server_list.servers.push(ServerCard::new(graphics, server.name, server.ip, server.port));
                        }
                        elements.server_list.save(servers_file.to_path_buf());
                    }
                    for server in &elements.server_list.servers {
                        if server.play_button.is_hovered(graphics, &server.get_container(graphics, Some(menu_back.get_back_rect_container()))) {
                            let name = run_text_input_menu("Enter your name", graphics, menu_back);
                            if let Some(name) = name {
                                let game_result = run_game(graphics, menu_back, server.server_info.port, server.server_info.ip.clone(), &name, settings, global_settings);
                                if let Err(error) = game_result {
                                    println!("Game error: {error}");
                                    run_choice_menu(&format!("Game error: {error}"), graphics, menu_back, vec!["Ok"], None, None, true);
                                }
                            }
                        } else if server.delete_button.is_hovered(graphics, &server.get_container(graphics, Some(menu_back.get_back_rect_container())))
                            && run_choice_menu(
                                format!("The server \"{}\" will be deleted.\nDo you want to proceed?", server.server_info.name).as_str(),
                                graphics,
                                menu_back,
                                vec!["Back", "Proceed"],
                                Some(0),
                                Some(1),
                                false,
                            ) == 1
                        {
                            let pos = elements.server_list.servers.iter().position(|s| s.server_info.name == server.server_info.name);
                            if let Some(pos) = pos {
                                elements.server_list.servers.remove(pos);
                                elements.server_list.save(servers_file.to_path_buf());
                            }
                            break;
                        }
                    }
                }
                gfx::Key::Escape => return true,
                _ => {}
            }
        }
    }
    false
}

fn render_elements(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    elements: &mut MultiplayerSelectorElements,
    top_rect_visibility: &mut f32,
    scrollable: &mut gfx::Scrollable,
) {
    menu_back.set_back_rect_width(MENU_WIDTH);

    menu_back.render_back(graphics);

    let hoverable = graphics.get_mouse_pos().1 > elements.top_height && graphics.get_mouse_pos().1 < graphics.get_window_size().1 - elements.bottom_height;

    for server in &mut elements.server_list.servers {
        server.set_enabled(hoverable);
    }

    let mut current_y = gfx::SPACING + scrollable.get_scroll_x(graphics, menu_back.get_back_rect_container()) + elements.top_height;
    let mut elements_height = 0.0;

    for server in &mut elements.server_list.servers {
        server.render(graphics, gfx::FloatPos(0.0, current_y), Some(menu_back.get_back_rect_container()));
        current_y += server.get_height() + gfx::SPACING;
        elements_height += server.get_height() + gfx::SPACING;
    }

    elements.top_rect.size.0 = menu_back.get_back_rect_width(graphics, None);

    *top_rect_visibility += ((if scrollable.get_scroll_pos() > 5.0 { 1.0 } else { 0.0 }) - *top_rect_visibility) / 20.0;

    if *top_rect_visibility < 0.01 {
        *top_rect_visibility = 0.0;
    }

    if *top_rect_visibility > 0.99 {
        *top_rect_visibility = 1.0;
    }

    elements.top_rect.fill_color.a = (*top_rect_visibility * gfx::TRANSPARENCY as f32 / 2.0) as u8;
    elements.top_rect.blur_radius = (*top_rect_visibility * gfx::BLUR as f32) as i32;
    elements.top_rect.shadow_intensity = (*top_rect_visibility * gfx::SHADOW_INTENSITY as f32) as i32;
    if *top_rect_visibility > 0.0 {
        elements.top_rect.render(graphics, menu_back.get_back_rect_container());
    }

    elements.bottom_rect.size.0 = menu_back.get_back_rect_width(graphics, None);

    if scrollable.scroll_size > scrollable.rect.size.1 {
        elements.bottom_rect.render(graphics, menu_back.get_back_rect_container());
    }

    elements.title.render(graphics, Some(menu_back.get_back_rect_container()), None);
    elements.back_button.render(graphics, menu_back.get_back_rect_container());

    elements.new_server_button.render(graphics, menu_back.get_back_rect_container());

    scrollable.scroll_size = elements_height;
    scrollable.rect.size.1 = graphics.get_window_size().1 - elements.top_height - elements.bottom_height;
    scrollable.render(graphics, menu_back.get_back_rect_container());

    graphics.update_window();
}
