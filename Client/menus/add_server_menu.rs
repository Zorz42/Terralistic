use crate::menus::background_rect::BackgroundRect;
use graphics as gfx;
use graphics::GraphicsContext;
use std::path::PathBuf;
use terralistic_server::MULTIPLAYER_PORT;

use super::multiplayer_selector::ServerInfo;
use std::net::{IpAddr, Ipv4Addr};

fn is_valid_ip(ip: &str) -> bool {
    if ip.contains(':') && ip.contains('.') {
        //port specified
        let ip_port: Vec<&str> = ip.split(':').collect();
        if ip_port.len() != 2 {
            return false;
        }
        let ip = ip_port[0];
        let port = ip_port[1];
        if port.parse::<u16>().is_err() {
            return false;
        }
        if ip.parse::<Ipv4Addr>().is_err() {
            return false;
        }
        return true;
    }
    ip.parse::<IpAddr>().is_ok()
}

/**this function runs the add server menu.*/
pub fn run_add_server_menu(
    graphics: &mut GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    _file_path: PathBuf, //will be used to not allow 2 servers with the same name to exist
) -> Option<ServerInfo> {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Add a new server:"));
    title.y = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut buttons_container = gfx::Container::new(0, 0, 0, 0, gfx::BOTTOM);

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));

    let mut add_button = gfx::Button::new();
    add_button.scale = 3.0;
    add_button.darken_on_disabled = true;
    add_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Add server"));
    add_button.x = back_button.get_width() + gfx::SPACING;

    buttons_container.rect.w = back_button.get_width() + add_button.get_width() + gfx::SPACING;
    buttons_container.rect.h = back_button.get_height();
    buttons_container.rect.y = -gfx::SPACING;

    let mut server_name_input = gfx::TextInput::new(graphics);
    server_name_input.scale = 3.0;
    server_name_input.set_hint(graphics, "Server name");
    server_name_input.orientation = gfx::CENTER;
    server_name_input.selected = true;
    server_name_input.y = -(server_name_input.get_height() + gfx::SPACING) / 2;

    let mut server_ip_input = gfx::TextInput::new(graphics);
    server_ip_input.scale = 3.0;
    server_ip_input.set_hint(graphics, "ip");
    server_ip_input.orientation = gfx::CENTER;
    server_ip_input.y = (server_ip_input.get_height() + gfx::SPACING) / 2;

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

    //this is where the menu is drawn
    'render_loop: while graphics.renderer.is_window_open() {
        add_button.disabled = server_name_input.text.is_empty() || server_ip_input.text.is_empty();

        while let Some(event) = graphics.renderer.get_event() {
            //sorts out the events
            server_name_input.on_event(&event, graphics, None);
            server_ip_input.on_event(&event, graphics, None);
            match event {
                gfx::Event::KeyRelease(key, ..) => match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            break 'render_loop;
                        }
                        if add_button.is_hovered(graphics, Some(&buttons_container))
                            && is_valid_ip(&server_ip_input.text)
                        {
                            let ip = server_ip_input.text.split(':').next().unwrap().to_string();
                            let port = if server_ip_input.text.contains(':') {
                                server_ip_input
                                    .text
                                    .split(':')
                                    .last()
                                    .unwrap()
                                    .to_string()
                                    .parse::<u16>()
                                    .unwrap()
                            } else {
                                MULTIPLAYER_PORT
                            };
                            return Some(ServerInfo::new(server_name_input.text.clone(), ip, port));
                        }
                    }
                    gfx::Key::Escape => {
                        if server_name_input.selected || server_ip_input.selected {
                            server_name_input.selected = false;
                            server_ip_input.selected = false;
                        } else {
                            break 'render_loop;
                        }
                    }
                    gfx::Key::Enter => {
                        if !server_name_input.text.is_empty()
                            && !server_ip_input.text.is_empty()
                            && is_valid_ip(&server_ip_input.text)
                        {
                            let ip = server_ip_input.text.split(':').next().unwrap().to_string();
                            let port = if server_ip_input.text.contains(':') {
                                server_ip_input
                                    .text
                                    .split(':')
                                    .last()
                                    .unwrap()
                                    .to_string()
                                    .parse::<u16>()
                                    .unwrap()
                            } else {
                                MULTIPLAYER_PORT
                            };
                            return Some(ServerInfo::new(server_name_input.text.clone(), ip, port));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        menu_back.set_back_rect_width(700);

        menu_back.render_back(graphics);

        //render input fields

        buttons_container.update(graphics, Some(menu_back.get_back_rect_container()));

        title.render(graphics, Some(menu_back.get_back_rect_container()));
        back_button.render(graphics, Some(&buttons_container));

        add_button.render(graphics, Some(&buttons_container));

        server_name_input.render(graphics, Some(menu_back.get_back_rect_container()));
        server_ip_input.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.renderer.update_window();
    }
    None
}
