use crate::libraries::graphics as gfx;

use super::BackgroundRect;

pub fn run_choice_menu(menu_title: &str, graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect, buttons: Vec<&str>, esc_choice: Option<usize>, enter_choice: Option<usize>) -> usize {
    let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();

    let mut title_lines = Vec::new();
    let mut curr_y = 0.0;
    for line in text_lines_vec {
        let mut sprite = gfx::Sprite::new();
        sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(line, Some(200)));
        sprite.scale = 3.0;
        sprite.orientation = gfx::TOP;
        sprite.pos.1 = curr_y;
        curr_y += sprite.get_size().1 + gfx::SPACING;
        title_lines.push(sprite);
    }

    let mut lines_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::Orientation { x: 0.5, y: 0.3 }, None);

    let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

    let mut buttons_vec = Vec::new();
    let mut buttons_width = 0.0;
    let mut max_button_height: f32 = 0.0;
    for button_text in buttons {
        let mut button_sprite = gfx::Button::new();
        button_sprite.scale = 3.0;
        button_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(button_text, None));
        button_sprite.pos.0 = buttons_width;
        buttons_width += button_sprite.get_size().0 + gfx::SPACING;
        max_button_height = max_button_height.max(button_sprite.get_size().1);
        buttons_vec.push(button_sprite);
    }

    buttons_container.rect.size = gfx::FloatSize(buttons_width, max_button_height);
    buttons_container.rect.pos.1 = -gfx::SPACING;

    while graphics.renderer.is_window_open() {
        while let Some(event) = graphics.renderer.get_event() {
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        for (i, button) in &mut buttons_vec.iter().enumerate() {
                            if button.is_hovered(graphics, Some(&buttons_container)) {
                                return i;
                            }
                        }
                    }
                    gfx::Key::Escape => {
                        if let Some(choice) = esc_choice {
                            return choice;
                        }
                    }
                    gfx::Key::Enter => {
                        if let Some(choice) = enter_choice {
                            return choice;
                        }
                    }
                    _ => {}
                }
            }
        }
        menu_back.set_back_rect_width(700.0);

        menu_back.render_back(graphics);

        buttons_container.update(graphics, Some(menu_back.get_back_rect_container()));
        lines_container.update(graphics, Some(menu_back.get_back_rect_container()));

        for sprite in &mut title_lines {
            sprite.render(graphics, Some(&lines_container), None);
        }

        for sprite in &mut buttons_vec {
            sprite.render(graphics, Some(&buttons_container));
        }

        graphics.renderer.update_window();
    }

    0
}
