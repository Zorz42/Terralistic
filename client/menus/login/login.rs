use crate::client::game::tls_client;
use crate::client::menus::background_rect::BackgroundRect;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::GraphicsContext;
use crate::shared::tls_client::ConnectionState::{CONNECTED, FAILED};
use anyhow::Result;
use directories::BaseDirs;
use gfx::{BaseUiElement, UiElement};
use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc::TryRecvError;

/// this function runs the login menu.
#[allow(clippy::too_many_lines)] // TODO: reduce the number of lines in this function
pub fn run_login_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) -> bool {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login:", None)));
    title.pos.1 = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

    let mut back_button = gfx::Button::new(|| {});
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));

    let mut confirm_button = gfx::Button::new(|| {});
    confirm_button.scale = 3.0;
    confirm_button.darken_on_disabled = true;
    confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None));
    confirm_button.pos.0 = back_button.get_size().0 + gfx::SPACING;

    buttons_container.rect.size.0 = back_button.get_size().0 + confirm_button.get_size().0 + gfx::SPACING;
    buttons_container.rect.size.1 = back_button.get_size().1;
    buttons_container.rect.pos.1 = -gfx::SPACING;

    let mut username_input = gfx::TextInput::new(graphics);
    username_input.scale = 3.0;
    username_input.set_hint(graphics, "Username");
    username_input.orientation = gfx::CENTER;
    username_input.selected = true;
    username_input.pos.1 = -(username_input.get_size().1 + gfx::SPACING);
    //username_input.pos.1 = -(username_input.get_size().1 + gfx::SPACING) / 2.0;

    let mut password_input = gfx::TextInput::new(graphics);
    password_input.scale = 3.0;
    password_input.set_hint(graphics, "Password");
    password_input.orientation = gfx::CENTER;
    //password_input.pos.1 = (password_input.get_size().1 + gfx::SPACING) / 2.0;

    let mut email_input = gfx::TextInput::new(graphics);
    email_input.scale = 3.0;
    email_input.set_hint(graphics, "Email");
    email_input.orientation = gfx::CENTER;
    email_input.pos.1 = email_input.get_size().1 + gfx::SPACING;

    let mut login_register_toggle = gfx::Toggle::new();
    login_register_toggle.pos = gfx::FloatPos(0.0, username_input.pos.1 - username_input.get_size().1 - gfx::SPACING);
    login_register_toggle.left_color = gfx::GREY;
    login_register_toggle.right_color = gfx::GREY;
    login_register_toggle.orientation = gfx::CENTER;

    let mut login_sprite = gfx::Sprite::new();
    login_sprite.scale = 3.0;
    login_sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None)));
    login_sprite.orientation = gfx::CENTER;
    login_sprite.pos = gfx::FloatPos(
        (-login_sprite.get_texture().get_texture_size().0 * login_sprite.scale - login_register_toggle.size.0) / 2.0 - gfx::SPACING,
        login_register_toggle.get_container(graphics, menu_back.get_back_rect_container()).rect.pos.1,
    );

    let mut register_sprite = gfx::Sprite::new();
    register_sprite.scale = 3.0;
    register_sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Register", None)));
    register_sprite.orientation = gfx::CENTER;
    register_sprite.pos = gfx::FloatPos(
        (register_sprite.get_texture().get_texture_size().0 * register_sprite.scale + login_register_toggle.size.0) / 2.0 + gfx::SPACING,
        login_register_toggle.get_container(graphics, menu_back.get_back_rect_container()).rect.pos.1,
    );

    username_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts letters, numbers and _ symbol
        if text.is_ascii_alphanumeric() || text == '_' {
            return Some(text);
        }
        None
    }));

    password_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts letters, numbers and special symbols
        if text.is_ascii_alphanumeric() || text.is_ascii_punctuation() {
            return Some(text);
        }
        None
    }));

    email_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts symbols allowed by the email standard.
        // This should be improved by adding better checks https://en.wikipedia.org/wiki/Email_address

        if text.is_ascii_alphanumeric() || "!#$%&'*+-/=?^_`{|}~.@\"[]\\ \t(),:;<>".contains(text) {
            return Some(text);
        }
        None
    }));

    let mut email_shown = false;
    //this is where the menu is drawn
    while graphics.is_window_open() {
        confirm_button.disabled = username_input.get_text().is_empty() || password_input.get_text().is_empty() || (login_register_toggle.toggled && !is_valid_email(email_input.get_text()));

        while let Some(event) = graphics.get_event() {
            //sorts out the events
            username_input.on_event(graphics, &event, menu_back.get_back_rect_container());
            password_input.on_event(graphics, &event, menu_back.get_back_rect_container());
            email_input.on_event(graphics, &event, menu_back.get_back_rect_container());
            login_register_toggle.on_event(graphics, &event, menu_back.get_back_rect_container());
            if login_register_toggle.changed {
                let text = if login_register_toggle.toggled { "Register" } else { "Login" };
                confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
                title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&(text.to_owned() + ":"), None)));
                email_shown = login_register_toggle.toggled;
                login_register_toggle.changed = false;
            }
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, &buttons_container) {
                            return false;
                        }
                        if confirm_button.is_hovered(graphics, &buttons_container) {
                            save_user_data(username_input.get_text(), password_input.get_text());
                            if login_register_toggle.toggled {
                                eprintln!("{:?}", register(username_input.get_text(), password_input.get_text(), email_input.get_text(), graphics, menu_back));
                            }
                            return true;
                        }
                    }
                    gfx::Key::Escape => {
                        if username_input.selected || password_input.selected {
                            username_input.selected = false;
                            password_input.selected = false;
                        } else {
                            return false;
                        }
                    }
                    gfx::Key::Enter => {
                        if !confirm_button.disabled {
                            save_user_data(username_input.get_text(), password_input.get_text());
                            if login_register_toggle.toggled {
                                eprintln!("{:?}", register(username_input.get_text(), password_input.get_text(), email_input.get_text(), graphics, menu_back));
                            }
                            return true;
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

        title.render(graphics, menu_back.get_back_rect_container());

        back_button.render(graphics, &buttons_container);
        confirm_button.render(graphics, &buttons_container);

        username_input.render(graphics, menu_back.get_back_rect_container());
        password_input.render(graphics, menu_back.get_back_rect_container());
        if email_shown {
            email_input.render(graphics, menu_back.get_back_rect_container());
        }

        login_register_toggle.render(graphics, menu_back.get_back_rect_container());
        login_sprite.render(graphics, menu_back.get_back_rect_container());
        register_sprite.render(graphics, menu_back.get_back_rect_container());

        graphics.update_window();
    }
    #[allow(clippy::needless_return)] //if we remove it it thinks it needs it, otherwise it says we don't. kinda annoying
    return false;
}

fn save_user_data(username: &str, password: &str) {
    let Some(base_dirs) = BaseDirs::new() else {
        println!("Failed to get base directories!");
        return;
    };
    let config_path = base_dirs.data_dir().join("Terralistic").join("user.txt");
    let mut file = match std::fs::File::create(config_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to create file: {error}");
            return;
        }
    };
    let content = format!("{username}\n{password}");
    if let Err(error) = file.write_all(content.as_bytes()) {
        println!("Failed to write to file: {error}");
    }
}

fn register(username: &str, password: &str, email: &str, graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect) -> Result<()> {
    let mut registering_text = gfx::Sprite::new();
    registering_text.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Registering", None)));
    registering_text.orientation = gfx::CENTER;
    registering_text.scale = 3.0;

    //add menu rendering
    let mut manager = tls_client::TlsClient::new()?;
    while !matches!(manager.get_connection_state(), CONNECTED(_)) {
        manager.connect();
        menu_back.render_back(graphics);
        registering_text.render(graphics, menu_back.get_back_rect_container());
        graphics.update_window();

        if matches!(manager.get_connection_state(), FAILED(_)) {
            return Err(anyhow::anyhow!("cloud server not available"));
        }
    }
    manager.write(kvptree::ValueType::LIST(HashMap::from([
        ("auth_type".to_owned(), kvptree::ValueType::STRING("register".to_owned())),
        (
            "credentials".to_owned(),
            kvptree::ValueType::LIST(HashMap::from([
                ("username".to_owned(), kvptree::ValueType::STRING(username.to_owned())),
                ("password".to_owned(), kvptree::ValueType::STRING(password.to_owned())),
                ("email".to_owned(), kvptree::ValueType::STRING(email.to_owned())),
            ])),
        ),
    ])))?;
    let message;
    loop {
        manager.connect();
        menu_back.render_back(graphics);
        registering_text.render(graphics, menu_back.get_back_rect_container());
        graphics.update_window();

        match manager.read() {
            Err(e) => {
                if matches!(e, TryRecvError::Disconnected) {
                    return Err(e.into());
                }
            }
            Ok(value) => {
                message = value;
                break;
            }
        }
    }

    if let Ok(value) = message.get_str("error") {
        let time = std::time::Instant::now() + std::time::Duration::from_secs(5);
        registering_text.set_texture(gfx::Texture::load_from_surface(
            &graphics
                .font
                .create_text_surface(&format!("error:\n{value}"), Some((menu_back.get_back_rect_width(graphics, None) / 3.5) as i32)),
        ));
        while std::time::Instant::now() < time {
            manager.connect();
            menu_back.render_back(graphics);
            registering_text.render(graphics, menu_back.get_back_rect_container());
            graphics.update_window();
        }
    }

    eprintln!("{message}");

    Ok(())
}

pub(super) fn is_valid_email(email: &str) -> bool {
    let (user, domain) = email.rsplit_once('@').unwrap_or_default();
    if user.is_empty() {
        //empty user or unsuccessful split
        return false;
    }
    if user.len() > 64 {
        return false;
    }
    let user_ok = if user.starts_with('"') && user.ends_with('"') {
        if user.len() < 2 {
            return false;
        }
        let mut user = user.to_owned();
        user.remove(0);
        user.remove(user.as_bytes().len() - 1);
        while let Some(n) = user.find("\\\\") {
            user.remove(n);
            user.remove(n);
        }
        while let Some(n) = user.find("\\\"") {
            user.remove(n);
            user.remove(n);
        }
        is_quoted_user_email_valid(&user)
    } else {
        is_user_email_valid(user)
    };

    let domain_ok = if domain.starts_with('[') && domain.ends_with(']') {
        let mut domain = domain.to_owned();
        domain.remove(0);
        domain.remove(domain.as_bytes().len() - 1);
        is_ip_email_domain_valid(domain)
    } else {
        is_email_domain_valid(domain)
    };

    user_ok && domain_ok
}

fn is_user_email_valid(user: &str) -> bool {
    for c in user.chars() {
        if !(c.is_ascii_alphanumeric() || "!#$%&'*+-/=?^_`{|}~.".contains(c)) {
            return false;
        }
    }
    let pos = user.find("..");
    pos.is_none() //as .. is not allowed in an unquoted string
}

fn is_quoted_user_email_valid(user: &str) -> bool {
    for c in user.chars() {
        if (!c.is_ascii_graphic() || "\\\"".contains(c)) && !" \t".contains(c) {
            return false;
        }
    }
    true
}

fn is_email_domain_valid(domain: &str) -> bool {
    let parts: Vec<&str> = domain.split('.').collect();
    for part in parts {
        let mut contains_letter = false;
        for c in part.chars() {
            if c.is_ascii_alphabetic() {
                contains_letter = true;
            }
            if !(c.is_ascii_alphanumeric() || c == '-') {
                return false;
            }
        }
        if !contains_letter {
            return false;
        }
    }
    true
}

fn is_ip_email_domain_valid(mut domain: String) -> bool {
    if domain.starts_with("IPv6:") {
        domain.remove(0); //I
        domain.remove(0); //P
        domain.remove(0); //v
        domain.remove(0); //6
        domain.remove(0); //:
    }
    let res = std::net::IpAddr::from_str(&domain);
    res.map_or(false, |_| true)
}
