use crate::client::game::tls_client;
use crate::client::menus::background_rect::BackgroundRect;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::GraphicsContext;
use crate::shared::tls_client::ConnectionState::{CONNECTED, FAILED};
use anyhow::Result;
use directories::BaseDirs;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::TryRecvError;

pub struct LoginMenu {
    title: gfx::Sprite,
    back_button: gfx::Button,
    confirm_button: gfx::Button,
    username_input: gfx::TextInput,
    password_input: gfx::TextInput,
    email_input: gfx::TextInput,
    login_register_toggle: gfx::Toggle,
    login_sprite: gfx::Sprite,
    register_sprite: gfx::Sprite,
}

impl LoginMenu {
    #[must_use]
    pub fn new(graphics: &GraphicsContext, close_menu: Rc<RefCell<bool>>) -> Self {
        let mut title = gfx::Sprite::new();
        title.scale = 3.0;
        title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login:", None)));
        title.pos.1 = gfx::SPACING;
        title.orientation = gfx::TOP;

        let mut back_button = gfx::Button::new(move || {
            *close_menu.borrow_mut() = true;
        });
        back_button.scale = 3.0;
        back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));
        back_button.orientation = gfx::BOTTOM;

        let mut confirm_button = gfx::Button::new(|| {});
        confirm_button.scale = 3.0;
        confirm_button.darken_on_disabled = true;
        confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None));
        confirm_button.orientation = gfx::BOTTOM;

        back_button.pos = gfx::FloatPos(-confirm_button.get_size().0 / 2.0 - gfx::SPACING, -gfx::SPACING);
        confirm_button.pos = gfx::FloatPos(back_button.get_size().0 / 2.0 + gfx::SPACING, -gfx::SPACING);

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

        let mut register_sprite = gfx::Sprite::new();
        register_sprite.scale = 3.0;
        register_sprite.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Register", None)));
        register_sprite.orientation = gfx::CENTER;

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

        Self {
            title,
            back_button,
            confirm_button,
            username_input,
            password_input,
            email_input,
            login_register_toggle,
            login_sprite,
            register_sprite,
        }
    }
}

impl UiElement for LoginMenu {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        let email_shown = self.login_register_toggle.toggled;
        let mut elements_vec: Vec<&mut dyn BaseUiElement> = vec![
            &mut self.title,
            &mut self.back_button,
            &mut self.confirm_button,
            &mut self.username_input,
            &mut self.password_input,
            &mut self.login_register_toggle,
            &mut self.login_sprite,
            &mut self.register_sprite,
        ];

        if email_shown {
            elements_vec.push(&mut self.email_input);
        }

        elements_vec
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        let mut elements_vec: Vec<&dyn BaseUiElement> = vec![
            &self.title,
            &self.back_button,
            &self.confirm_button,
            &self.username_input,
            &self.password_input,
            &self.login_register_toggle,
            &self.login_sprite,
            &self.register_sprite,
        ];

        if self.login_register_toggle.toggled {
            elements_vec.push(&self.email_input);
        }

        elements_vec
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.login_sprite.pos = gfx::FloatPos(
            (-self.login_sprite.get_texture().get_texture_size().0 * self.login_sprite.scale - self.login_register_toggle.size.0) / 2.0 - gfx::SPACING,
            self.login_register_toggle.get_container(graphics, parent_container).rect.pos.1,
        );

        self.register_sprite.pos = gfx::FloatPos(
            (self.register_sprite.get_texture().get_texture_size().0 * self.register_sprite.scale + self.login_register_toggle.size.0) / 2.0 + gfx::SPACING,
            self.login_register_toggle.get_container(graphics, parent_container).rect.pos.1,
        );

        if self.login_register_toggle.changed {
            let text = if self.login_register_toggle.toggled { "Register" } else { "Login" };
            self.confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
            self.title
                .set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&(text.to_owned() + ":"), None)));
            self.login_register_toggle.changed = false;
        }

        self.confirm_button.disabled =
            self.username_input.get_text().is_empty() || self.password_input.get_text().is_empty() || (self.login_register_toggle.toggled && !is_valid_email(self.email_input.get_text()));
    }

    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(key, ..) = event {
            match key {
                gfx::Key::MouseLeft => {
                    if self.back_button.is_hovered(graphics, parent_container) {
                        return false;
                    }
                    if self.confirm_button.is_hovered(graphics, parent_container) {
                        save_user_data(self.username_input.get_text(), self.password_input.get_text());
                        if self.login_register_toggle.toggled {
                            eprintln!("{:?}", register(self.username_input.get_text(), self.password_input.get_text(), self.email_input.get_text(), graphics));
                        }
                        return true;
                    }
                }
                gfx::Key::Escape => {
                    if self.username_input.selected || self.password_input.selected {
                        self.username_input.selected = false;
                        self.password_input.selected = false;
                    } else {
                        return false;
                    }
                }
                gfx::Key::Enter => {
                    if !self.confirm_button.disabled {
                        save_user_data(self.username_input.get_text(), self.password_input.get_text());
                        if self.login_register_toggle.toggled {
                            eprintln!("{:?}", register(self.username_input.get_text(), self.password_input.get_text(), self.email_input.get_text(), graphics));
                        }
                        return true;
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

fn register(username: &str, password: &str, email: &str, graphics: &mut GraphicsContext) -> Result<()> {
    let mut menu_back = crate::client::menus::menu_back::MenuBack::new(graphics);
    let window_container = gfx::Container::default(graphics);
    let mut registering_text = gfx::Sprite::new();
    registering_text.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Registering", None)));
    registering_text.orientation = gfx::CENTER;
    registering_text.scale = 3.0;

    //add menu rendering
    let mut manager = tls_client::TlsClient::new()?;
    while !matches!(manager.get_connection_state(), CONNECTED(_)) {
        manager.connect();
        menu_back.render_back(graphics);
        registering_text.render(graphics, &menu_back.get_container(graphics, &window_container));
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
        registering_text.render(graphics, &menu_back.get_container(graphics, &window_container));
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
            registering_text.render(graphics, &menu_back.get_container(graphics, &window_container));
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
