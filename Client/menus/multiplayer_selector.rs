use super::world_creation::run_world_creation;
use crate::game::private_world::run_private_world;
use crate::menus::background_rect::BackgroundRect;
use crate::menus::run_choice_menu;

use directories::BaseDirs;
use graphics as gfx;
use graphics::GraphicsContext;
use std::fs;
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};


pub const MENU_WIDTH: i32 = 800;

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    name: String,
    ip: String,
}

impl ServerInfo {
    pub fn new(name: String, ip: String) -> Self {
        Self { name, ip }
    }
}


/**
World is a struct that contains all information to
render the server in Server selector.
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
    pub fn new(graphics: &GraphicsContext, name: String, ip: String) -> Self {

        let mut rect = gfx::RenderRect::new(0.0, 0.0, (MENU_WIDTH - 2 * gfx::SPACING) as f32, 0.0);
        rect.orientation = gfx::TOP;
        rect.fill_color.a = 100;
        rect.smooth_factor = 60.0;

        let mut icon = gfx::Sprite::new();
        icon.texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(
            &include_bytes!("../../Build/Resources/world_icon.opa").to_vec(),
        ));
        rect.h = icon.get_height() as f32 + 2.0 * gfx::SPACING as f32;
        icon.x = gfx::SPACING;
        icon.orientation = gfx::LEFT;

        let mut title = gfx::Sprite::new();
        title.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface(name.clone().as_str()));
        title.x = icon.x + icon.get_width() + gfx::SPACING;
        title.y = gfx::SPACING;
        title.scale = 3.0;

        let mut play_button = gfx::Button::new();
        play_button.texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(
            &include_bytes!("../../Build/Resources/play_button.opa").to_vec(),
        ));
        play_button.scale = 3.0;
        play_button.margin = 5;
        play_button.x = icon.x + icon.get_width() + gfx::SPACING;
        play_button.y = -gfx::SPACING;
        play_button.orientation = gfx::BOTTOM_LEFT;

        let mut delete_button = gfx::Button::new();
        delete_button.texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(
            &include_bytes!("../../Build/Resources/delete_button.opa").to_vec(),
        ));
        delete_button.scale = 3.0;
        delete_button.margin = 5;
        delete_button.x = play_button.x + play_button.get_width() + gfx::SPACING;
        delete_button.y = -gfx::SPACING;
        delete_button.orientation = gfx::BOTTOM_LEFT;

        Self {
            server_info: ServerInfo::new(name, ip),
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
        &mut self, graphics: &mut GraphicsContext, x: i32, y: i32,
        parent_container: Option<&gfx::Container>,
    ) {
        self.rect.x = x as f32;
        self.rect.y = y as f32;
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
    pub fn get_height(&self) -> i32 {
        self.rect.h as i32
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
        &self, graphics: &GraphicsContext, parent_container: Option<&gfx::Container>,
    ) -> gfx::Container {
        self.rect.get_container(graphics, parent_container)
    }
}

/**
WorldList is a struct that is used to list all worlds in the world folder
and render them in the singleplayer selector menu.
 */
pub struct ServerList {
    pub servers: Vec<ServerCard>,
}

impl ServerList {
    pub fn new(graphics: &GraphicsContext, file_path: PathBuf) -> ServerList {
        let mut world_list = ServerList { servers: Vec::new() };
        world_list.refresh(graphics, file_path);
        world_list
    }

    pub fn refresh(&mut self, graphics: &GraphicsContext, file_path: PathBuf) {
        let temp_servers;

        if !file_path.exists() {
            temp_servers = Ok(Vec::new());
            &std::fs::write(file_path, &bincode::serialize::<&Vec<ServerInfo>>(&temp_servers.as_ref().unwrap()).unwrap()).unwrap();
        } else {
            temp_servers = bincode::deserialize::<Vec<ServerInfo>>(
                &std::fs::read(file_path).unwrap(),
            );
        }

        if temp_servers.is_ok() {
            self.servers.clear();
            for server in temp_servers.unwrap() {
                self.servers.push(ServerCard::new(graphics, server.name, server.ip));
            }
        }
    }
}

pub fn run_multiplayer_selector(
    graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect,
) {

    let base_dirs = BaseDirs::new().unwrap();
    let servers_file = base_dirs.data_dir().join("Terralistic").join("servers.txt");

    let mut server_list = ServerList::new(graphics, servers_file.clone());

    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(
        &graphics
            .font
            .create_text_surface("Select a server to play!"),
    );
    title.y = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));
    back_button.y = -gfx::SPACING;
    back_button.orientation = gfx::BOTTOM;

    let mut new_world_button = gfx::Button::new();
    new_world_button.scale = 3.0;
    new_world_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New"));
    new_world_button.y = -gfx::SPACING;
    new_world_button.x = -gfx::SPACING;
    new_world_button.orientation = gfx::BOTTOM_RIGHT;

    let top_height = title.get_height() + 2 * gfx::SPACING;
    let bottom_height = back_button.get_height() + 2 * gfx::SPACING;

    let mut top_rect = gfx::RenderRect::new(0.0, 0.0, 0.0, top_height as f32);
    top_rect.orientation = gfx::TOP;

    let mut bottom_rect = gfx::RenderRect::new(0.0, 0.0, 0.0, bottom_height as f32);
    bottom_rect.fill_color.a = gfx::TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    bottom_rect.blur_radius = gfx::BLUR;
    bottom_rect.orientation = gfx::BOTTOM;

    let mut top_rect_visibility = 1.0;
    let mut position: f32 = 0.0;

    'render_loop: while graphics.renderer.is_window_open() {
        while let Some(event) = graphics.renderer.get_event() {
            match event {
                gfx::Event::KeyRelease(key, ..) => {
                    match key {
                        gfx::Key::MouseLeft => {
                            if back_button
                                .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                            {
                                break 'render_loop;
                            }
                            if new_world_button
                                .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                            {
                                //run_world_creation(graphics, menu_back, &mut world_list.worlds);//TODO: server creation
                                //world_list.refresh(graphics);
                            }

                            let mut needs_refresh = false;
                            for server in &mut server_list.servers {
                                if server.play_button.is_hovered(
                                    graphics,
                                    Some(&server.get_container(
                                        graphics,
                                        Some(menu_back.get_back_rect_container()),
                                    )),
                                ) {
                                    //run_private_world(graphics, menu_back, world.get_file_path());//TODO: join server
                                } else if server.delete_button.is_hovered(
                                    graphics,
                                    Some(&server.get_container(
                                        graphics,
                                        Some(menu_back.get_back_rect_container()),
                                    )),
                                ) && run_choice_menu(format!("The world \"{}\" will be deleted.\nDo you want to proceed?", server.server_info.name), graphics, menu_back, None, None) {
                                    //fs::remove_file(server.get_file_path()).unwrap();//TODO: remove server
                                    needs_refresh = true;
                                }
                            }
                            if needs_refresh {
                                server_list.refresh(graphics, servers_file.clone());
                            }
                        }
                        gfx::Key::Escape => {
                            break 'render_loop;
                        }
                        _ => {}
                    }
                }
                gfx::Event::MouseScroll(delta) => {
                    position += delta * 2.0;
                }
                _ => {}
            }
        }
        menu_back.set_back_rect_width(MENU_WIDTH);

        menu_back.render_back(graphics);

        let hoverable = graphics.renderer.get_mouse_y() as i32 > top_height
            && (graphics.renderer.get_mouse_y() as i32)
            < graphics.renderer.get_window_height() as i32 - bottom_height;

        for server in &mut server_list.servers {
            server.set_enabled(hoverable);
        }

        let mut current_y = gfx::SPACING;
        for i in 0..server_list.servers.len() {
            server_list.servers[i].render(
                graphics,
                0,
                current_y + top_height + position as i32,
                Some(menu_back.get_back_rect_container()),
            );
            current_y += server_list.servers[i].get_height() + gfx::SPACING;
        }

        top_rect.w = menu_back.get_back_rect_width(graphics, None) as f32;
        top_rect_visibility +=
            ((if position < -5.0 { 1.0 } else { 0.0 }) - top_rect_visibility) / 20.0;

        if top_rect_visibility < 0.01 {
            top_rect_visibility = 0.0;
        }

        if top_rect_visibility > 0.99 {
            top_rect_visibility = 1.0;
        }

        top_rect.fill_color.a = (top_rect_visibility * gfx::TRANSPARENCY as f32 / 2.0) as u8;
        top_rect.blur_radius = (top_rect_visibility * gfx::BLUR as f32) as i32;
        top_rect.shadow_intensity = (top_rect_visibility * gfx::SHADOW_INTENSITY as f32) as i32;
        if top_rect_visibility > 0.0 {
            top_rect.render(graphics, Some(menu_back.get_back_rect_container()));
        }

        bottom_rect.w = menu_back.get_back_rect_width(graphics, None) as f32;
        let mut scroll_limit =
            current_y - graphics.renderer.get_window_height() as i32 + top_height + bottom_height;
        if scroll_limit < 0 {
            scroll_limit = 0;
        }

        if scroll_limit > 0 {
            bottom_rect.render(graphics, Some(menu_back.get_back_rect_container()));
        }

        if position > 0.0 {
            position -= position / 20.0;
        }

        if position < -scroll_limit as f32 {
            position -= (position + scroll_limit as f32) / 20.0;
        }

        title.render(graphics, Some(menu_back.get_back_rect_container()));
        back_button.render(graphics, Some(menu_back.get_back_rect_container()));

        new_world_button.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.renderer.update_window();
    }
}
