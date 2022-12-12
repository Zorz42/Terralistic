use crate::menus::background_rect::BackgroundRect;
use graphics as gfx;
use rand;
use rand::Rng;

pub fn run_main_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    let mut singleplayer_button = gfx::Button::new();
    singleplayer_button.scale = 3.0;
    singleplayer_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Singleplayer")));
    singleplayer_button.orientation = gfx::CENTER;

    let mut multiplayer_button = gfx::Button::new();
    multiplayer_button.scale = 3.0;
    multiplayer_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Multiplayer")));
    multiplayer_button.orientation = gfx::CENTER;

    let mut settings_button = gfx::Button::new();
    settings_button.scale = 3.0;
    settings_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Settings")));
    settings_button.orientation = gfx::CENTER;

    let mut mods_button = gfx::Button::new();
    mods_button.scale = 3.0;
    mods_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Mods")));
    mods_button.orientation = gfx::CENTER;

    let mut exit_button = gfx::Button::new();
    exit_button.scale = 3.0;
    exit_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Exit")));
    exit_button.orientation = gfx::CENTER;

    {
        let buttons = vec![&mut singleplayer_button, &mut multiplayer_button, &mut settings_button, &mut mods_button, &mut exit_button];

        // buttons are on top of another on the center of the screen
        // their combined height is centered on the screen

        let mut total_height = 0;
        for button in &buttons {
            total_height += button.get_height();
        }
        let mut current_y = -total_height / 2 + buttons[0].get_height();
        for button in buttons {
            button.y = current_y;
            current_y += button.get_height();
        }
    }

    while graphics.renderer.is_window_open() {
        let events = graphics.renderer.get_events();
        for event in events {
            match event {
                gfx::Event::KeyRelease(key) => {

                }
                _ => {}
            }
        }

        let buttons = vec![&mut singleplayer_button, &mut multiplayer_button, &mut settings_button, &mut mods_button, &mut exit_button];
        // get maximum width of all buttons and set background width to that
        let mut max_width = 0;
        for button in &buttons {
            if button.get_width() > max_width {
                max_width = button.get_width();
            }
        }
        menu_back.set_back_rect_width(max_width + 100);

        menu_back.render_back(graphics);

        for button in buttons {
            button.render(graphics, Some(menu_back.get_back_rect_container()));
        }

        graphics.renderer.post_render();
    }
}