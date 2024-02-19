use crate::client::game::tls_client::{AuthenticationState, TlsClient};
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::singleplayer_selector::MENU_WIDTH;
use crate::client::menus::SettingsMenu;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use crate::shared::tls_client::ConnectionState;
use crate::shared::versions::VERSION;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::background_rect::BackgroundRect;
use super::{run_login_menu, run_multiplayer_selector, SingleplayerSelector};
use gfx::BaseUiElement;

enum MainMenuState {
    None,
    SingleplayerSelector(Box<Cell<SingleplayerSelector>>),
}

#[allow(clippy::too_many_lines)] // TODO: split this function up
#[allow(clippy::cognitive_complexity)]
pub fn run_main_menu(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    settings: &Rc<RefCell<Settings>>,
    global_settings: &Rc<RefCell<GlobalSettings>>,
    menu_back_timer: std::time::Instant,
) {
    let mut state = MainMenuState::None;
    let mut secondary_menu_back = crate::client::menus::MenuBack::new(graphics);
    secondary_menu_back.set_x_position(graphics.get_window_size().0);
    secondary_menu_back.main_back_menu = false;
    secondary_menu_back.set_back_rect_width(MENU_WIDTH);
    let close_secondary_menu = Rc::new(RefCell::new(false));

    let mut singleplayer_button = gfx::Button::new(|| {});
    singleplayer_button.scale = 3.0;
    singleplayer_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Singleplayer", None));
    singleplayer_button.orientation = gfx::CENTER;

    let mut multiplayer_button = gfx::Button::new(|| {});
    multiplayer_button.scale = 3.0;
    multiplayer_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Multiplayer", None));
    multiplayer_button.orientation = gfx::CENTER;

    let mut settings_button = gfx::Button::new(|| {});
    settings_button.scale = 3.0;
    settings_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Settings", None));
    settings_button.orientation = gfx::CENTER;

    let mut mods_button = gfx::Button::new(|| {});
    mods_button.scale = 3.0;
    mods_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Mods", None));
    mods_button.orientation = gfx::CENTER;

    let mut exit_button = gfx::Button::new(|| {});
    exit_button.scale = 3.0;
    exit_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Exit", None));
    exit_button.orientation = gfx::CENTER;

    let mut debug_title = gfx::Sprite::new();
    debug_title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("DEBUG MODE", None));
    debug_title.color = gfx::DARK_GREY;
    debug_title.orientation = gfx::TOP;
    debug_title.scale = 2.0;
    debug_title.pos.1 = gfx::SPACING / 4.0;

    let mut title = gfx::Sprite::new();
    title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Terralistic", None));
    title.scale = 4.0;
    title.orientation = gfx::TOP;
    title.pos.1 = debug_title.pos.1 + debug_title.get_size().1 + gfx::SPACING / 2.0;

    let mut version = gfx::Sprite::new();
    version.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(VERSION, None));
    version.color = gfx::GREY;
    version.orientation = gfx::BOTTOM;
    version.scale = 2.0;
    version.pos.1 = -5.0;

    {
        let buttons = [&mut singleplayer_button, &mut multiplayer_button, &mut settings_button, &mut mods_button, &mut exit_button];

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

    let mut in_settings = false;
    let mut settings_menu = SettingsMenu::new();
    settings_menu.init(graphics, settings);

    let cloud_status_rect = gfx::Rect::new(gfx::FloatPos(10.0, 10.0), gfx::FloatSize(20.0, 20.0));

    let mut cloud_status_button = gfx::Button::new(|| {});
    cloud_status_button.color = gfx::GREY;
    cloud_status_button.pos = cloud_status_rect.pos + gfx::FloatPos(cloud_status_rect.size.0 + 5.0, -10.0);
    cloud_status_button.orientation = gfx::TOP_LEFT;
    cloud_status_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None));
    cloud_status_button.scale = 1.5;

    let mut tls_client = match TlsClient::new() {
        Err(e) => {
            eprintln!("error getting tls client:\n{e}\n\nbacktrace:\n{}", e.backtrace());
            None
        }
        Ok(mut client) => {
            client.connect();
            Some(client)
        }
    };

    while graphics.is_window_open() {
        tls_client.as_mut().map_or_else(
            || {},
            |client| {
                //client.print_state();
                client.authenticate();
            },
        );
        while let Some(event) = graphics.get_event() {
            if in_settings {
                if settings_menu.on_event(&event, graphics, settings) {
                    in_settings = false;
                }
                continue;
            }

            match state {
                MainMenuState::None => (),
                MainMenuState::SingleplayerSelector(ref mut menu) => {
                    let _ = menu.get_mut().on_event(graphics, &event, secondary_menu_back.get_back_rect_container());
                }
            }
            if *close_secondary_menu.borrow_mut() {
                state = MainMenuState::None;
                *close_secondary_menu.borrow_mut() = false;
            }

            if let gfx::Event::KeyRelease(key, ..) = event {
                // check for every button if it was clicked with the left mouse button
                if key == gfx::Key::MouseLeft {
                    if singleplayer_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        if !matches!(state, MainMenuState::SingleplayerSelector(_)) {
                            let singleplayer_menu = SingleplayerSelector::new(graphics, settings.clone(), global_settings.clone(), close_secondary_menu.clone(), menu_back_timer);
                            state = MainMenuState::SingleplayerSelector(Box::new(Cell::new(singleplayer_menu)));
                        }
                    } else if multiplayer_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        //run_multiplayer_selector(graphics, menu_back, settings, global_settings);
                    } else if settings_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        in_settings = true;
                    } else if mods_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                    } else if exit_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        graphics.close_window();
                    } else if cloud_status_button.is_hovered(graphics, &gfx::Container::default(graphics)) && run_login_menu(graphics, menu_back) {
                        if let Some(client) = &mut tls_client {
                            client.reset();
                        }
                    }
                }
            }
        }

        let max_width = MENU_WIDTH + menu_back.get_back_rect_width(graphics, None) + gfx::SPACING;
        menu_back.set_x_position(if matches!(state, MainMenuState::None) {
            0.0
        } else {
            -max_width / 2.0 + menu_back.get_back_rect_width(graphics, None) / 2.0
        });
        secondary_menu_back.set_x_position(if matches!(state, MainMenuState::None) {
            graphics.get_window_size().0
        } else {
            max_width / 2.0 - MENU_WIDTH / 2.0
        });

        if in_settings {
            menu_back.render_back(graphics);
            let width = settings_menu.render(graphics, settings);
            menu_back.set_back_rect_width(width);
            graphics.update_window();
            global_settings.borrow_mut().update(graphics, settings);
            continue;
        }

        let buttons = vec![&mut singleplayer_button, &mut multiplayer_button, &mut settings_button, &mut mods_button, &mut exit_button];
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
            button.render(graphics, menu_back.get_back_rect_container());
        }

        let color = get_tls_status_color(&tls_client);
        cloud_status_rect.render(graphics, color);
        cloud_status_button.render(graphics, &gfx::Container::default(graphics));

        #[cfg(debug_assertions)]
        debug_title.render(graphics, Some(menu_back.get_back_rect_container()), None);
        title.render(graphics, Some(menu_back.get_back_rect_container()), None);
        version.render(graphics, Some(menu_back.get_back_rect_container()), None);

        //render secondary menu
        secondary_menu_back.render_back(graphics);
        match state {
            MainMenuState::None => {}
            MainMenuState::SingleplayerSelector(ref mut menu) => menu.get_mut().render(graphics, secondary_menu_back.get_back_rect_container()),
        }

        graphics.update_window();
    }
}

fn get_tls_status_color(tls_client: &Option<TlsClient>) -> gfx::Color {
    tls_client.as_ref().map_or_else(
        || gfx::Color::new(255, 0, 0, 255),
        |client| match &client.get_connection_state() {
            ConnectionState::CONNECTING(_) => gfx::Color::new(255, 255, 0, 255),
            _ => match client.get_authentication_state() {
                AuthenticationState::AUTHENTICATING => gfx::Color::new(255, 255, 0, 255),
                AuthenticationState::AUTHENTICATED => gfx::Color::new(0, 255, 0, 255),
                _ => gfx::Color::new(255, 0, 0, 255),
            },
        },
    )
}
