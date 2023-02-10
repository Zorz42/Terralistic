use super::background_rect::BackgroundRect;
use super::{run_multiplayer_selector, run_singleplayer_selector};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::SPACING;
use crate::shared::versions::VERSION;

pub fn run_main_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    let mut singleplayer_button = gfx::Button::new();
    singleplayer_button.scale = 3.0;
    singleplayer_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Singleplayer"));
    singleplayer_button.orientation = gfx::CENTER;

    let mut multiplayer_button = gfx::Button::new();
    multiplayer_button.scale = 3.0;
    multiplayer_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Multiplayer"));
    multiplayer_button.orientation = gfx::CENTER;

    let mut settings_button = gfx::Button::new();
    settings_button.scale = 3.0;
    settings_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Settings"));
    settings_button.orientation = gfx::CENTER;

    let mut mods_button = gfx::Button::new();
    mods_button.scale = 3.0;
    mods_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Mods"));
    mods_button.orientation = gfx::CENTER;

    let mut exit_button = gfx::Button::new();
    exit_button.scale = 3.0;
    exit_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Exit"));
    exit_button.orientation = gfx::CENTER;

    let mut debug_title = gfx::Sprite::new();
    debug_title.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("DEBUG MODE"));
    debug_title.color = gfx::GREY;
    debug_title.orientation = gfx::TOP;
    debug_title.scale = 2.0;
    debug_title.pos.1 = SPACING / 4.0;

    let mut title = gfx::Sprite::new();
    title.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Terralistic"));
    title.scale = 4.0;
    title.orientation = gfx::TOP;
    title.pos.1 = debug_title.pos.1 + debug_title.get_size().1 + SPACING / 2.0;

    let mut version = gfx::Sprite::new();
    version.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(VERSION));
    version.color = gfx::GREY;
    version.orientation = gfx::BOTTOM;
    version.scale = 2.0;
    version.pos.1 = -5.0;

    {
        let buttons = vec![
            &mut singleplayer_button,
            &mut multiplayer_button,
            &mut settings_button,
            &mut mods_button,
            &mut exit_button,
        ];

        // buttons are on top of another on the center of the screen
        // their combined height is centered on the screen

        let mut total_height = 0.0;
        for button in &buttons {
            total_height += button.get_size().1;
        }
        let mut current_y = -total_height / 2.0 + buttons[0].get_size().1;
        for button in buttons {
            button.pos.1 = current_y;
            current_y += button.get_size().1;
        }
    }

    while graphics.renderer.is_window_open() {
        while let Some(event) = graphics.renderer.get_event() {
            if let gfx::Event::KeyRelease(key, ..) = event {
                // check for every button if it was clicked with the left mouse button
                if key == gfx::Key::MouseLeft {
                    if singleplayer_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        run_singleplayer_selector(graphics, menu_back);
                    } else if multiplayer_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        println!("Multiplayer clicked");
                        run_multiplayer_selector(graphics, menu_back);
                    } else if settings_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        println!("Settings clicked");
                    } else if mods_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        println!("Mods clicked");
                    } else if exit_button
                        .is_hovered(graphics, Some(menu_back.get_back_rect_container()))
                    {
                        graphics.renderer.close_window();
                    }
                }
            }
        }

        let buttons = vec![
            &mut singleplayer_button,
            &mut multiplayer_button,
            &mut settings_button,
            &mut mods_button,
            &mut exit_button,
        ];
        // get maximum width of all buttons and set background width to that
        let mut max_width = 0.0;
        for button in &buttons {
            if button.get_size().0 > max_width {
                max_width = button.get_size().0;
            }
        }
        menu_back.set_back_rect_width(max_width + 100.0);

        menu_back.render_back(graphics);

        for button in buttons {
            button.render(graphics, Some(menu_back.get_back_rect_container()));
        }

        debug_title.render(graphics, Some(menu_back.get_back_rect_container()));
        title.render(graphics, Some(menu_back.get_back_rect_container()));
        version.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.renderer.update_window();
    }
}
