use crate::client::game::tls_client;
use crate::client::menus::background_rect::BackgroundRect;
use crate::libraries::graphics as gfx;
use crate::shared::tls_client::ConnectionState::CONNECTED;
use anyhow::Result;
use directories::BaseDirs;
use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc::TryRecvError;

/// this function runs the login menu.
#[allow(clippy::too_many_lines)] // TODO: reduce the number of lines in this function
pub fn run_login_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) -> bool {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login:", None));
    title.pos.1 = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));

    let mut confirm_button = gfx::Button::new();
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
    login_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None));
    login_sprite.orientation = gfx::CENTER;
    login_sprite.pos = gfx::FloatPos(
        (-login_sprite.texture.get_texture_size().0 * login_sprite.scale - login_register_toggle.size.0) / 2.0 - gfx::SPACING,
        login_register_toggle.get_container(graphics, Some(menu_back.get_back_rect_container())).rect.pos.1,
    );

    let mut register_sprite = gfx::Sprite::new();
    register_sprite.scale = 3.0;
    register_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Register", None));
    register_sprite.orientation = gfx::CENTER;
    register_sprite.pos = gfx::FloatPos(
        (register_sprite.texture.get_texture_size().0 * register_sprite.scale + login_register_toggle.size.0) / 2.0 + gfx::SPACING,
        login_register_toggle.get_container(graphics, Some(menu_back.get_back_rect_container())).rect.pos.1,
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
        // this closure only accepts symbols allowed by the email standard
        if text.is_ascii_alphanumeric() || ['+', '-', '_', '~'].contains(&text) {
            return Some(text);
        }
        None
    }));

    let mut email_shown = false;
    //this is where the menu is drawn
    while graphics.is_window_open() {
        confirm_button.disabled = username_input.get_text().is_empty() || password_input.get_text().is_empty();

        while let Some(event) = graphics.get_event() {
            //sorts out the events
            username_input.on_event(&event, graphics, None);
            password_input.on_event(&event, graphics, None);
            if login_register_toggle.on_event(&event, graphics, Some(menu_back.get_back_rect_container())) {
                let text = if login_register_toggle.toggled { "Register" } else { "Login" };
                confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
                title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&(text.to_owned() + ":"), None));
                email_shown = login_register_toggle.toggled;
            }
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            return false;
                        }
                        if confirm_button.is_hovered(graphics, Some(&buttons_container)) {
                            save_user_data(username_input.get_text(), password_input.get_text());
                            if login_register_toggle.toggled {
                                eprintln!("{:?}", register(username_input.get_text(), password_input.get_text(), email_input.get_text()));
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
                                eprintln!("{:?}", register(username_input.get_text(), password_input.get_text(), email_input.get_text()));
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

        title.render(graphics, Some(menu_back.get_back_rect_container()), None);

        back_button.render(graphics, Some(&buttons_container));
        confirm_button.render(graphics, Some(&buttons_container));

        username_input.render(graphics, Some(menu_back.get_back_rect_container()));
        password_input.render(graphics, Some(menu_back.get_back_rect_container()));
        if email_shown {
            email_input.render(graphics, Some(menu_back.get_back_rect_container()));
        }

        login_register_toggle.render(graphics, Some(menu_back.get_back_rect_container()));
        login_sprite.render(graphics, Some(menu_back.get_back_rect_container()), None);
        register_sprite.render(graphics, Some(menu_back.get_back_rect_container()), None);

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

fn register(username: &str, password: &str, email: &str) -> Result<()> {
    //add menu rendering
    let mut manager = tls_client::TlsClient::new()?;
    while !matches!(manager.get_connection_state(), CONNECTED(_)) {
        manager.connect();
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
    eprintln!("{message}");

    Ok(())
}
