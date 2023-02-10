use super::background_rect::BackgroundRect;
use super::{run_add_server_menu, run_choice_menu};
use crate::client::game::core::Game;

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use directories::BaseDirs;
use serde_derive::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

pub const MENU_WIDTH: i32 = 800;

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

/**struct to pass around UI elements for rendering and updating*/
struct MultiplayerSelectorElements {
    position: f32,
    top_rect: gfx::RenderRect,
    bottom_rect: gfx::RenderRect,
    title: gfx::Sprite,
    back_button: gfx::Button,
    new_world_button: gfx::Button,
    server_list: ServerList,
    top_height: f32,
    bottom_height: f32,
}

/**
World is a struct that contains all information to
render the server in server selector.
 */
pub struct ServerCard {
    pub server_info: ServerInfo,
    rect: gfx::RenderRect,
    play_button: gfx::Button,
    delete_button: gfx::Button,
    title: gfx::Sprite,
    icon: gfx::Sprite,
}

impl ServerCard {
    pub fn new(graphics: &GraphicsContext, name: String, ip: String, port: u16) -> Self {
        let mut rect = gfx::RenderRect::new(
            FloatPos(0.0, 0.0),
            FloatSize((MENU_WIDTH as f32 - 2.0 * gfx::SPACING) as f32, 0.0),
        );
        rect.orientation = gfx::TOP;
        rect.fill_color.a = 100;
        rect.smooth_factor = 60.0;

        let mut icon = gfx::Sprite::new();
        icon.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/world_icon.opa"
            ))
            .unwrap(),
        );
        rect.size.1 = icon.get_size().1 + 2.0 * gfx::SPACING;
        icon.pos.0 = gfx::SPACING;
        icon.orientation = gfx::LEFT;

        let mut title = gfx::Sprite::new();
        title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&name));
        title.pos.0 = icon.pos.0 + icon.get_size().1 + gfx::SPACING;
        title.pos.1 = gfx::SPACING;
        title.scale = 3.0;

        let mut play_button = gfx::Button::new();
        play_button.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/join_button.opa"
            ))
            .unwrap(),
        );
        play_button.scale = 3.0;
        play_button.padding = 5.0;
        play_button.pos.0 = icon.pos.0 + icon.get_size().0 + gfx::SPACING;
        play_button.pos.1 = -gfx::SPACING;
        play_button.orientation = gfx::BOTTOM_LEFT;

        let mut delete_button = gfx::Button::new();
        delete_button.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/remove_button.opa"
            ))
            .unwrap(),
        );
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

    /**
    This function renders the world card on the x and y position.
     */
    pub fn render(
        &mut self,
        graphics: &mut GraphicsContext,
        pos: FloatPos,
        parent_container: Option<&gfx::Container>,
    ) {
        self.rect.pos = pos;
        self.rect.render(graphics, parent_container);

        let rect_container = self.rect.get_container(graphics, parent_container);
        self.icon.render(graphics, Some(&rect_container));
        self.title.render(graphics, Some(&rect_container));
        self.play_button.render(graphics, Some(&rect_container));
        self.delete_button.render(graphics, Some(&rect_container));
    }

    /**
    This function returns height of the world card.
     */
    pub const fn get_height(&self) -> f32 {
        self.rect.size.1
    }

    /**
    This function disables/enables the world card buttons.
     */
    pub fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }

    /**
    This function returns the container of the world card.
     */
    pub fn get_container(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&gfx::Container>,
    ) -> gfx::Container {
        self.rect.get_container(graphics, parent_container)
    }
}

/**
`WorldList` is a struct that is used to list all worlds in the world folder
and render them in the singleplayer selector menu.
 */
pub struct ServerList {
    pub servers: Vec<ServerCard>,
}

impl ServerList {
    pub fn new(graphics: &GraphicsContext, file_path: PathBuf) -> Self {
        let mut world_list = Self {
            servers: Vec::new(),
        };
        world_list.refresh(graphics, file_path);
        world_list
    }

    pub fn refresh(&mut self, graphics: &GraphicsContext, file_path: PathBuf) {
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
            self.servers.push(ServerCard::new(
                graphics,
                server.name,
                server.ip,
                server.port,
            ));
        }
    }

    pub fn save(&self, file_path: PathBuf) {
        let server_infos: Vec<ServerInfo> = self
            .servers
            .iter()
            .map(|server| {
                ServerInfo::new(
                    server.server_info.name.clone(),
                    server.server_info.ip.clone(),
                    server.server_info.port,
                )
            })
            .collect();

        let res = std::fs::write(
            file_path,
            serde_json::to_string(&server_infos).unwrap_or_default(),
        );
        if res.is_err() {
            println!("Failed to save servers!");
        }
    }
}

pub fn run_multiplayer_selector(
    graphics: &mut GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
) {
    let Some(base_dirs) = BaseDirs::new() else {
        println!("Failed to get base directories!");
        return;
    };
    let servers_file = base_dirs.data_dir().join("Terralistic").join("servers.txt");

    let server_list = ServerList::new(graphics, servers_file.clone());

    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(
        &graphics
            .font
            .create_text_surface("Select a server to play!"),
    );
    title.pos.1 = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));
    back_button.pos.1 = -gfx::SPACING;
    back_button.orientation = gfx::BOTTOM;

    let mut new_world_button = gfx::Button::new();
    new_world_button.scale = 3.0;
    new_world_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New"));
    new_world_button.pos.1 = -gfx::SPACING;
    new_world_button.pos.0 = -gfx::SPACING;
    new_world_button.orientation = gfx::BOTTOM_RIGHT;

    let top_height = title.get_size().1 + 2.0 * gfx::SPACING;
    let bottom_height = back_button.get_size().1 + 2.0 * gfx::SPACING;

    let mut top_rect = gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, top_height as f32));
    top_rect.orientation = gfx::TOP;

    let mut bottom_rect =
        gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, bottom_height as f32));
    bottom_rect.fill_color.a = gfx::TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    bottom_rect.blur_radius = gfx::BLUR;
    bottom_rect.orientation = gfx::BOTTOM;

    let position: f32 = 0.0;

    let mut elements = MultiplayerSelectorElements {
        position,
        top_rect,
        bottom_rect,
        title,
        back_button,
        new_world_button,
        server_list,
        top_height,
        bottom_height,
    };

    while graphics.renderer.is_window_open() {
        //update_elements returns true if the loop is to be broken
        if update_elements(graphics, menu_back, &mut elements, &servers_file) {
            break;
        }
        render_elements(graphics, menu_back, &mut elements);
    }
}
fn update_elements(
    graphics: &mut GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    elements: &mut MultiplayerSelectorElements,
    servers_file: &Path,
) -> bool {
    while let Some(event) = graphics.renderer.get_event() {
        match event {
            gfx::Event::KeyRelease(key, ..) => match key {
                gfx::Key::MouseLeft => {
                    if elements
                        .back_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        return true;
                    }
                    if elements
                        .new_world_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        if let Some(server) =
                            run_add_server_menu(graphics, menu_back, &elements.server_list.servers)
                        {
                            elements.server_list.servers.push(ServerCard::new(
                                graphics,
                                server.name,
                                server.ip,
                                server.port,
                            ));
                        }
                        elements.server_list.save(servers_file.to_path_buf());
                    }
                    for server in &elements.server_list.servers {
                        if server.play_button.is_hovered(
                            graphics,
                            Some(&server.get_container(
                                graphics,
                                Some(menu_back.get_back_rect_container()),
                            )),
                        ) {
                            let mut game =
                                Game::new(server.server_info.port, server.server_info.ip.clone());
                            game.run(graphics, menu_back);
                        } else if server.delete_button.is_hovered(
                            graphics,
                            Some(&server.get_container(
                                graphics,
                                Some(menu_back.get_back_rect_container()),
                            )),
                        ) && run_choice_menu(
                            format!(
                                "The world \"{}\" will be deleted.\nDo you want to proceed?",
                                server.server_info.name
                            )
                            .as_str(),
                            graphics,
                            menu_back,
                            None,
                            None,
                        ) {
                            let pos = elements
                                .server_list
                                .servers
                                .iter()
                                .position(|s| s.server_info.name == server.server_info.name);
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
            },
            gfx::Event::MouseScroll(delta) => {
                elements.position += delta * 2.0;
            }
            _ => {}
        }
    }
    false
}

fn render_elements(
    graphics: &mut GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    elements: &mut MultiplayerSelectorElements,
) {
    let mut top_rect_visibility = 1.0;

    menu_back.set_back_rect_width(MENU_WIDTH as f32);

    menu_back.render_back(graphics);

    let hoverable = graphics.renderer.get_mouse_pos().1 > elements.top_height
        && graphics.renderer.get_mouse_pos().1
            < graphics.renderer.get_window_size().1 - elements.bottom_height;

    for server in &mut elements.server_list.servers {
        server.set_enabled(hoverable);
    }

    let mut current_y = gfx::SPACING;
    for server in &mut elements.server_list.servers {
        server.render(
            graphics,
            FloatPos(
                0.0,
                current_y + elements.top_height + elements.position as f32,
            ),
            Some(menu_back.get_back_rect_container()),
        );
        current_y += server.get_height() + gfx::SPACING;
    }

    elements.top_rect.size.0 = menu_back.get_back_rect_width(graphics, None) as f32;
    top_rect_visibility +=
        ((if elements.position < -5.0 { 1.0 } else { 0.0 }) - top_rect_visibility) / 20.0;

    if top_rect_visibility < 0.01 {
        top_rect_visibility = 0.0;
    }

    if top_rect_visibility > 0.99 {
        top_rect_visibility = 1.0;
    }

    elements.top_rect.fill_color.a = (top_rect_visibility * gfx::TRANSPARENCY as f32 / 2.0) as u8;
    elements.top_rect.blur_radius = (top_rect_visibility * gfx::BLUR as f32) as i32;
    elements.top_rect.shadow_intensity =
        (top_rect_visibility * gfx::SHADOW_INTENSITY as f32) as i32;
    if top_rect_visibility > 0.0 {
        elements
            .top_rect
            .render(graphics, Some(menu_back.get_back_rect_container()));
    }

    elements.bottom_rect.size.0 = menu_back.get_back_rect_width(graphics, None) as f32;
    let mut scroll_limit = current_y - graphics.renderer.get_window_size().1
        + elements.top_height
        + elements.bottom_height;
    if scroll_limit < 0.0 {
        scroll_limit = 0.0;
    }

    if scroll_limit > 0.0 {
        elements
            .bottom_rect
            .render(graphics, Some(menu_back.get_back_rect_container()));
    }

    if elements.position > 0.0 {
        elements.position -= elements.position / 20.0;
    }

    if elements.position < -scroll_limit as f32 {
        elements.position -= (elements.position + scroll_limit as f32) / 20.0;
    }

    elements
        .title
        .render(graphics, Some(menu_back.get_back_rect_container()));
    elements
        .back_button
        .render(graphics, Some(menu_back.get_back_rect_container()));

    elements
        .new_world_button
        .render(graphics, Some(menu_back.get_back_rect_container()));

    graphics.renderer.update_window();
}
