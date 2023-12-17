use crate::client::menus::BackgroundRect;
use crate::libraries::graphics as gfx;

pub fn run_text_input_menu(menu_title: &str, graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) -> Option<String> {
    let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();

    let mut title_lines = Vec::new();
    for line in text_lines_vec {
        let mut sprite = gfx::Sprite::new();
        sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(line, None));
        sprite.scale = 3.0;
        sprite.orientation = gfx::TOP;
        sprite.pos.1 = gfx::SPACING + title_lines.len() as f32 * (sprite.get_size().1 + gfx::SPACING);
        title_lines.push(sprite);
    }

    let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

    let back_str = "Back";
    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(back_str, None));

    let confirm_str = "Continue";
    let mut confirm_button = gfx::Button::new();
    confirm_button.scale = 3.0;
    confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(confirm_str, None));
    confirm_button.pos.0 = back_button.get_size().0 + gfx::SPACING;

    buttons_container.rect.size.0 = back_button.get_size().0 + confirm_button.get_size().0 + gfx::SPACING;
    buttons_container.rect.size.1 = back_button.get_size().1;
    buttons_container.rect.pos.1 = -gfx::SPACING;

    let mut input_field = gfx::TextInput::new(graphics);
    input_field.scale = 3.0;
    input_field.orientation = gfx::CENTER;

    //this is where the menu is drawn
    while graphics.is_window_open() {
        while let Some(event) = graphics.get_event() {
            input_field.on_event(&event, graphics, Some(menu_back.get_back_rect_container()));

            //sorts out the events
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            return None;
                        }
                        if confirm_button.is_hovered(graphics, Some(&buttons_container)) {
                            return Some(input_field.get_text().clone());
                        }
                    }
                    gfx::Key::Escape => return None,
                    gfx::Key::Enter => {
                        return Some(input_field.get_text().clone());
                    }
                    _ => {}
                }
            }
        }
        menu_back.set_back_rect_width(700.0);

        menu_back.render_back(graphics);

        //render input fields

        buttons_container.update(graphics, Some(menu_back.get_back_rect_container()));

        for sprite in &mut title_lines {
            sprite.render(graphics, Some(menu_back.get_back_rect_container()), None);
        }

        back_button.render(graphics, Some(&buttons_container));

        confirm_button.render(graphics, Some(&buttons_container));

        input_field.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.update_window();
    }

    None
}
