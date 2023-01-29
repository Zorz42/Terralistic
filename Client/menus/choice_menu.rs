use crate::menus::BackgroundRect;
use graphics as gfx;
use graphics::{GraphicsContext, Sprite};
use graphics::Checkbox;
use directories::BaseDirs;

pub fn run_choice_menu(
    menu_title: String, graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect, confirm_name_override: Option<String>, back_name_override: Option<String>, allow_screen_skip: bool
) -> bool {

    let menu_path = BaseDirs::new()
        .unwrap()
        .data_dir()
        .join("Terralistic")
        .join("choice_menu_skips.txt");

    if !menu_path.exists() {
        std::fs::write(menu_path.clone(), "").unwrap();
    } else {
        let menu_skips = std::fs::read_to_string(menu_path.clone()).unwrap();
        let names_vec = menu_skips.split('\n').collect::<Vec<&str>>();
        if names_vec.contains(&menu_title.replace("\n", " ").as_str()) {
            return true;
        }
    }


    let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();

    let mut title_lines = Vec::new();
    for line in text_lines_vec {
        let mut sprite = gfx::Sprite::new();
        sprite.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from(line)));
        sprite.scale = 3.0;
        sprite.orientation = gfx::TOP;
        sprite.y = gfx::SPACING + title_lines.len() as i32 * (sprite.get_height() + gfx::SPACING);
        title_lines.push(sprite);
    }

    let mut buttons_container = gfx::Container::new(0, 0, 0, 0, gfx::BOTTOM);

    let back_str = back_name_override.unwrap_or(String::from("Back"));
    let mut back_button = gfx::Button::new();
    back_button.darken_on_disabled = true;
    back_button.scale = 3.0;
    back_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface(back_str));

    let confirm_str = confirm_name_override.unwrap_or(String::from("Confirm"));
    let mut confirm_button = gfx::Button::new();
    confirm_button.scale = 3.0;
    confirm_button.texture = gfx::Texture::load_from_surface(
        &graphics.font.create_text_surface(confirm_str),
    );
    confirm_button.x = back_button.get_width() + gfx::SPACING;

    buttons_container.rect.w = back_button.get_width() + confirm_button.get_width() + gfx::SPACING;
    buttons_container.rect.h = back_button.get_height();
    buttons_container.rect.y = -gfx::SPACING;

    let mut skip_checkbox = Checkbox::new();
    skip_checkbox.scale = 3.0;
    skip_checkbox.orientation = gfx::CENTER;
    let mut skip_texture: Sprite = Sprite::new();
    skip_texture.texture = gfx::Texture::load_from_surface(
        &graphics.font.create_text_surface(String::from("do not show this screen again")),
    );
    skip_texture.scale = 3.0;
    skip_texture.orientation = gfx::CENTER;

    skip_checkbox.x = -(skip_texture.get_width()) / 2 - gfx::SPACING; ;
    skip_texture.x = (skip_checkbox.w as i32) / 2 + gfx::SPACING;

    //this is where the menu is drawn
    'render_loop: while graphics.renderer.is_window_open() {
        while let Some(event) = graphics.renderer.get_event() {
            //sorts out the events
            match event {
                gfx::Event::KeyRelease(key, ..) => match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            return false;
                        }
                        if confirm_button.is_hovered(graphics, Some(&buttons_container)) {
                            if skip_checkbox.checked && allow_screen_skip {
                                let mut menu_skips = std::fs::read_to_string(menu_path.clone()).unwrap();
                                menu_skips.push_str("\n");
                                menu_skips.push_str(&menu_title.replace("\n", " "));
                                std::fs::write(menu_path, menu_skips).unwrap();
                            }
                            return true;
                        }
                        if skip_checkbox.is_hovered(graphics, None) && allow_screen_skip{
                            skip_checkbox.toggle();
                        }
                    }
                    gfx::Key::Escape => {
                        return false;
                    }
                    gfx::Key::Enter => {
                        return true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        menu_back.set_back_rect_width(700);

        menu_back.render_back(graphics);


        back_button.disabled = skip_checkbox.checked && allow_screen_skip;
        //render input fields

        buttons_container.update(graphics, Some(&menu_back.get_back_rect_container()));

        for sprite in title_lines.iter_mut() {
            sprite.render(graphics, Some(&menu_back.get_back_rect_container()));
        }

        back_button.render(graphics, Some(&buttons_container));

        confirm_button.render(graphics, Some(&buttons_container));

        if allow_screen_skip {
            skip_texture.render(graphics, None);
            skip_checkbox.render(graphics, None);
        }

        graphics.renderer.update_window();
    }

    false
}
