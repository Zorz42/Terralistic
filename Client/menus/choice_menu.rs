use graphics::GraphicsContext;
use graphics as gfx;
use crate::menus::BackgroundRect;

pub fn run_choice_menu(menu_title: String, graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect) -> bool {
    let text_lines_vec = menu_title.split('\n').collect::<Vec<&str>>();

    let mut title_lines = Vec::new();
    for line in text_lines_vec {
        let mut sprite = gfx::Sprite::new();
        sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from(line)));
        sprite.scale = 3.0;
        sprite.orientation = gfx::TOP;
        sprite.y = gfx::SPACING + title_lines.len() as i32 * (sprite.get_height() + gfx::SPACING);
        title_lines.push(sprite);
    }

    let mut buttons_container = gfx::Container::new(0, 0, 0, 0,  gfx::BOTTOM);

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Back")));

    let mut confirm_button = gfx::Button::new();
    confirm_button.scale = 3.0;
    confirm_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Confirm")));
    confirm_button.x = back_button.get_width() + gfx::SPACING;

    buttons_container.rect.w = back_button.get_width() + confirm_button.get_width() + gfx::SPACING;
    buttons_container.rect.h = back_button.get_height();
    buttons_container.rect.y = -gfx::SPACING;


    //this is where the menu is drawn
    'render_loop: while graphics.renderer.is_window_open() {

        while let Some(event) = graphics.renderer.get_event() {//sorts out the events
            match event {
                gfx::Event::KeyRelease(key, ..) => {
                    match key {
                        gfx::Key::MouseLeft => {
                            if back_button.is_hovered(graphics, Some(&buttons_container)) {
                                return false;
                            }
                            if confirm_button.is_hovered(graphics, Some(&buttons_container)) {
                                return true;
                            }
                        }
                        gfx::Key::Escape => {
                            return false;
                        }
                        gfx::Key::Enter => {
                            return true;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        menu_back.set_back_rect_width(700);

        menu_back.render_back(graphics);

        //render input fields

        buttons_container.update(graphics, Some(&menu_back.get_back_rect_container()));

        for sprite in title_lines.iter_mut() {
            sprite.render(graphics, Some(&menu_back.get_back_rect_container()));
        }

        back_button.render(graphics, Some(&buttons_container));

        confirm_button.render(graphics, Some(&buttons_container));

        graphics.renderer.update_window();

    }

    false
}