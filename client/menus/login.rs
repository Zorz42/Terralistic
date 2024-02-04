use crate::client::menus::background_rect::BackgroundRect;
use crate::libraries::graphics as gfx;
use directories::BaseDirs;
use std::io::Write;

/// this function runs the world creation menu.
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
    username_input.pos.1 = -(username_input.get_size().1 + gfx::SPACING) / 2.0;

    let mut password_input = gfx::TextInput::new(graphics);
    password_input.scale = 3.0;
    password_input.set_hint(graphics, "Password");
    password_input.orientation = gfx::CENTER;
    password_input.pos.1 = (password_input.get_size().1 + gfx::SPACING) / 2.0;

    username_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts letters, numbers and _ symbol
        if text.is_alphanumeric() || text == '_' {
            return Some(text);
        }
        None
    }));

    password_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts letters, numbers and special symbols
        if text.is_alphanumeric() || text.is_ascii_punctuation() {
            return Some(text);
        }
        None
    }));

    //this is where the menu is drawn
    while graphics.is_window_open() {
        confirm_button.disabled = username_input.get_text().is_empty() || password_input.get_text().is_empty();

        while let Some(event) = graphics.get_event() {
            //sorts out the events
            username_input.on_event(&event, graphics, None);
            password_input.on_event(&event, graphics, None);
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            return false;
                        }
                        if confirm_button.is_hovered(graphics, Some(&buttons_container)) {
                            save_user_data(username_input.get_text(), password_input.get_text());
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
