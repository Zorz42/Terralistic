use super::background_rect::BackgroundRect;
use crate::libraries::graphics as gfx;

use super::multiplayer_selector::ServerCard;
use crate::server::server_core::MULTIPLAYER_PORT;

use super::multiplayer_selector::ServerInfo;
use std::net::{IpAddr, Ipv4Addr};

fn get_ip_port(server_ip_input: &str) -> (String, u16) {
    let ip = server_ip_input
        .split(':')
        .next()
        .unwrap_or("127.0.0.1")
        .to_owned();
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

/// this function runs the add server menu.
#[allow(clippy::too_many_lines)]
pub fn run_add_server_menu(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    servers_list: &Vec<ServerCard>,
) -> Option<ServerInfo> {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(
        &graphics.font.create_text_surface("Add a new server:", None),
    );
    title.pos.1 = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut buttons_container = gfx::Container::new(
        graphics,
        gfx::FloatPos(0.0, 0.0),
        gfx::FloatSize(0.0, 0.0),
        gfx::BOTTOM,
        None,
    );

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));

    let mut add_button = gfx::Button::new();
    add_button.scale = 3.0;
    add_button.darken_on_disabled = true;
    add_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Add server", None));
    add_button.pos.0 = back_button.get_size().0 + gfx::SPACING;

    buttons_container.rect.size.0 =
        back_button.get_size().0 + add_button.get_size().0 + gfx::SPACING;
    buttons_container.rect.size.1 = back_button.get_size().1;
    buttons_container.rect.pos.1 = -gfx::SPACING;

    let mut server_name_input = gfx::TextInput::new(graphics);
    server_name_input.scale = 3.0;
    server_name_input.set_hint(graphics, "server name");
    server_name_input.orientation = gfx::CENTER;
    server_name_input.selected = true;
    server_name_input.pos.1 = -(server_name_input.get_size().1 + gfx::SPACING) / 2.0;

    let mut server_ip_input = gfx::TextInput::new(graphics);
    server_ip_input.scale = 3.0;
    server_ip_input.set_hint(graphics, "ip");
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

    //this is where the menu is drawn
    'render_loop: while graphics.renderer.is_window_open() {
        add_button.disabled = server_name_input.get_text().is_empty()
            || server_ip_input.get_text().is_empty()
            || server_exists(server_name_input.get_text(), servers_list)
            || !is_valid_ip(server_ip_input.get_text());

        while let Some(event) = graphics.renderer.get_event() {
            //sorts out the events
            server_name_input.on_event(&event, graphics, None);
            server_ip_input.on_event(&event, graphics, None);
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            break 'render_loop;
                        }
                        if add_button.is_hovered(graphics, Some(&buttons_container)) {
                            let (ip, port) = get_ip_port(server_ip_input.get_text());
                            return Some(ServerInfo::new(
                                server_name_input.get_text().clone(),
                                ip,
                                port,
                            ));
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
                        if !add_button.disabled {
                            let (ip, port) = get_ip_port(server_ip_input.get_text());
                            return Some(ServerInfo::new(
                                server_name_input.get_text().clone(),
                                ip,
                                port,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        menu_back.set_back_rect_width(700.0);

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
