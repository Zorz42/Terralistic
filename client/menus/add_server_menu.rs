use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::libraries::graphics as gfx;
use crate::server::server_core::MULTIPLAYER_PORT;
use gfx::{BaseUiElement, UiElement};

use super::background_rect::BackgroundRect;
use super::multiplayer_selector::ServerCard;
use super::multiplayer_selector::ServerInfo;

fn get_ip_port(server_ip_input: &str) -> (String, u16) {
    let ip = server_ip_input.split(':').next().unwrap_or("127.0.0.1").to_owned();
    let port = if server_ip_input.contains(':') {
        server_ip_input
            .split(':')
            .last()
            .unwrap_or("-1") //-1 so it fails on next unwrap and always defaults to MULTIPLAYER_PORT
            .to_owned()
            .parse::<u16>()
            .unwrap_or(MULTIPLAYER_PORT)
    } else {
        MULTIPLAYER_PORT
    };
    (ip, port)
}

fn is_valid_ip(ip_: &str) -> bool {
    if ip_.contains(':') && ip_.contains('.') {
        //port specified
        let ip_port: Vec<&str> = ip_.split(':').collect();
        if ip_port.len() != 2 {
            return false;
        }
        let Some(ip) = ip_port.first() else {
            return false;
        };
        let Some(port) = ip_port.get(1) else {
            return false;
        };
        if port.parse::<u16>().is_err() {
            return false;
        }
        if ip.parse::<Ipv4Addr>().is_err() {
            return false;
        }
        return true;
    }
    ip_.parse::<IpAddr>().is_ok()
}

fn server_exists(name: &str, servers_list: &Vec<ServerCard>) -> bool {
    for server in servers_list {
        if server.server_info.name == name {
            return true;
        }
    }
    false
}

pub struct AddServerMenu {
    title: gfx::Sprite,
    back_button: gfx::Button,
    add_button: gfx::Button,
    server_name_input: gfx::TextInput,
    server_ip_input: gfx::TextInput,
    server_file: PathBuf,
    pub close: bool,
}

impl AddServerMenu {
    pub fn new(graphics: &gfx::GraphicsContext, server_file: PathBuf) -> Self {
        let mut title = gfx::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Add a new server:", None)));
        title.pos.1 = gfx::SPACING;
        title.orientation = gfx::TOP;

        let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

        let mut back_button = gfx::Button::new(|| {});
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.orientation = gfx::BOTTOM;

        let mut add_button = gfx::Button::new(|| {});
        add_button.scale = 3.0;
        add_button.darken_on_disabled = true;
        add_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Add server", None));
        add_button.pos.0 = back_button.get_size().0 + gfx::SPACING;
        add_button.orientation = gfx::BOTTOM;

        back_button.pos = gfx::FloatPos(-add_button.get_size().0 / 2.0 - gfx::SPACING, -gfx::SPACING);
        add_button.pos = gfx::FloatPos(back_button.get_size().0 / 2.0 + gfx::SPACING, -gfx::SPACING);

        buttons_container.rect.size.0 = back_button.get_size().0 + add_button.get_size().0 + gfx::SPACING;
        buttons_container.rect.size.1 = back_button.get_size().1;
        buttons_container.rect.pos.1 = -gfx::SPACING;

        let mut server_name_input = gfx::TextInput::new(graphics);
        server_name_input.scale = 3.0;
        server_name_input.set_hint(graphics, "World name");
        server_name_input.orientation = gfx::CENTER;
        server_name_input.selected = true;
        server_name_input.pos.1 = -(server_name_input.get_size().1 + gfx::SPACING) / 2.0;

        let mut server_ip_input = gfx::TextInput::new(graphics);
        server_ip_input.scale = 3.0;
        server_ip_input.set_hint(graphics, "World seed");
        server_ip_input.orientation = gfx::CENTER;
        server_ip_input.pos.1 = (server_ip_input.get_size().1 + gfx::SPACING) / 2.0;

        server_name_input.text_processing = Some(Box::new(|text: char| {
            // this closure only accepts letters, numbers and _ symbol
            if text.is_alphanumeric() || text == '_' {
                return Some(text);
            }
            None
        }));

        server_ip_input.text_processing = Some(Box::new(|text: char| {
            // this closure only accepts numbers, : and . symbols
            if text.is_numeric() || text == '.' || text == ':' {
                return Some(text);
            }
            None
        }));

        Self {
            title,
            back_button,
            add_button,
            server_name_input,
            server_ip_input,
            server_file,
            close: false,
        }
    }

    fn add_server(&self) {
        let (ip, port) = get_ip_port(self.server_ip_input.get_text());
        let file = std::fs::read_to_string(self.server_file.clone()).unwrap_or_else(|_| String::new());
        let mut servers: Vec<ServerInfo> = serde_json::from_str(&file).unwrap_or_else(|_| Vec::new());
        servers.push(ServerInfo::new(self.server_name_input.get_text().to_string(), ip, port));
        let file = serde_json::to_string(&servers).unwrap_or_else(|_| String::new());
        std::fs::write(self.server_file.clone(), file);
    }
}

impl UiElement for AddServerMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.title, &mut self.back_button, &mut self.add_button, &mut self.server_name_input, &mut self.server_ip_input]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.title, &self.back_button, &self.add_button, &self.server_name_input, &self.server_ip_input]
    }

    fn update_inner(&mut self, _grpahics: &mut gfx::GraphicsContext, _parent_container: &gfx::Container) {
        self.add_button.disabled = self.server_name_input.get_text().is_empty() || std::net::IpAddr::from_str(self.server_ip_input.get_text()).is_err();
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if self.add_button.on_event(graphics, event, &self.get_container(graphics, parent_container)) {
            self.add_server();
            self.close = true;
        }
        if self.back_button.on_event(graphics, event, &self.get_container(graphics, parent_container)) {
            self.close = true;
        }
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::Escape => {
                    if self.server_ip_input.selected || self.server_name_input.selected {
                        self.server_ip_input.selected = false;
                        self.server_name_input.selected = false;
                    } else {
                        self.close = true;
                    }
                }
                gfx::Key::Enter => {
                    if !self.add_button.disabled {
                        self.add_server();
                        self.close = true;
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
