use crate::client::game::tls_client::{AuthenticationState, TlsClient};
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::singleplayer_selector::MENU_WIDTH;
use crate::client::menus::SettingsMenu;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use crate::shared::tls_client::ConnectionState;
use crate::shared::versions::VERSION;
use gfx::UiElement;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::background_rect::BackgroundRect;
use super::{run_login_menu, MultiplayerSelector, SingleplayerSelector};
use gfx::BaseUiElement;

enum MainMenuState {
    None,
    SingleMenu((Cell<Box<dyn BaseUiElement>>, usize)),
    Transition {
        top_menu: (Cell<Box<dyn BaseUiElement>>, usize),
        bottom_menu: (Cell<Box<dyn BaseUiElement>>, usize),
        direction: bool, //true is down
        transition_state: f32,
        top_menu_container: gfx::Container,
        bottom_menu_container: gfx::Container,
    },
}

impl MainMenuState {
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        match self {
            Self::None => {}
            Self::SingleMenu((menu, _id)) => {
                menu.get_mut().render(graphics, parent_container);
            }
            Self::Transition {
                top_menu: (top_menu, _id_top),
                bottom_menu: (bottom_menu, _id_bot),
                top_menu_container,
                bottom_menu_container,
                ..
            } => {
                top_menu.get_mut().render(graphics, top_menu_container);
                bottom_menu.get_mut().render(graphics, bottom_menu_container);
            }
        }
    }

    pub fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let mut stop_transition = false;
        match self {
            Self::None => {}
            Self::SingleMenu((menu, _id)) => {
                menu.get_mut().update(graphics, parent_container);
            }
            Self::Transition {
                top_menu: (top_menu, _id_top),
                bottom_menu: (bottom_menu, _id_bot),
                direction,
                transition_state,
                top_menu_container,
                bottom_menu_container,
            } => {
                *transition_state += (1.0 - *transition_state) / 15.0;
                if *transition_state > 0.999 {
                    stop_transition = true;
                }
                let offset = if *direction {
                    -graphics.get_window_size().1 * (*transition_state)
                } else {
                    -graphics.get_window_size().1 * (1.0 - *transition_state)
                };
                top_menu_container.rect.pos.1 = parent_container.rect.pos.1 + offset;
                bottom_menu_container.rect.pos.1 = parent_container.rect.pos.1 + offset + graphics.get_window_size().1;
                top_menu_container.update(graphics, parent_container);
                bottom_menu_container.update(graphics, parent_container);
                top_menu.get_mut().update(graphics, top_menu_container);
                bottom_menu.get_mut().update(graphics, bottom_menu_container);
            }
        }
        if stop_transition {
            if let Self::Transition { top_menu, bottom_menu, direction, .. } = std::mem::replace(self, Self::None) {
                *self = Self::SingleMenu(if direction { bottom_menu } else { top_menu });
            }
        }
    }

    pub fn on_event(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) {
        match self {
            Self::None => {}
            Self::SingleMenu((menu, _id)) => {
                menu.get_mut().on_event(graphics, event, parent_container);
            }
            Self::Transition {
                top_menu: (top_menu, _id_top),
                bottom_menu: (bottom_menu, _id_bot),
                ..
            } => {
                top_menu.get_mut().on_event(graphics, event, parent_container);
                bottom_menu.get_mut().on_event(graphics, event, parent_container);
            }
        }
    }

    pub fn switch_to(&mut self, new_menu: (Cell<Box<dyn BaseUiElement>>, usize), graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) {
        match std::mem::replace(self, Self::None) {
            Self::None => {
                *self = Self::SingleMenu(new_menu);
            }
            Self::SingleMenu((menu, id)) => {
                let (top_menu, bottom_menu, direction) = if id < new_menu.1 { ((menu, id), new_menu, true) } else { (new_menu, (menu, id), false) };
                *self = Self::Transition {
                    top_menu,
                    bottom_menu,
                    transition_state: 0.0,
                    direction,
                    bottom_menu_container: gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None),
                    top_menu_container: gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None),
                }
            }
            Self::Transition { top_menu, bottom_menu, direction, .. } => {
                let switching_to = if direction { bottom_menu } else { top_menu };
                if new_menu.1 == switching_to.1 {
                    *self = Self::SingleMenu(switching_to);
                } else {
                    let (top_menu, bottom_menu, direction) = if switching_to.1 < new_menu.1 {
                        (switching_to, new_menu, true)
                    } else {
                        (new_menu, switching_to, false)
                    };
                    *self = Self::Transition {
                        top_menu,
                        bottom_menu,
                        transition_state: 0.0,
                        direction,
                        bottom_menu_container: gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None),
                        top_menu_container: gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None),
                    }
                }
            }
        }
    }
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
    let close_secondary_menu = Rc::new(RefCell::new(false));
    let mut secondary_menu_back = gfx::RenderRect::new(gfx::FloatPos(graphics.get_window_size().0, 0.0), gfx::FloatSize(MENU_WIDTH, graphics.get_window_size().1));
    secondary_menu_back.orientation = gfx::TOP;
    secondary_menu_back.blur_radius = gfx::BLUR;
    secondary_menu_back.smooth_factor = 60.0;
    secondary_menu_back.shadow_intensity = gfx::SHADOW_INTENSITY;
    secondary_menu_back.fill_color.a = gfx::TRANSPARENCY;
    secondary_menu_back.border_color = gfx::BORDER_COLOR;

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
    debug_title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("DEBUG MODE", None)));
    debug_title.color = gfx::DARK_GREY;
    debug_title.orientation = gfx::TOP;
    debug_title.scale = 2.0;
    debug_title.pos.1 = gfx::SPACING / 4.0;

    let mut title = gfx::Sprite::new();
    title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Terralistic", None)));
    title.scale = 4.0;
    title.orientation = gfx::TOP;
    title.pos.1 = debug_title.pos.1 + debug_title.get_size().1 + gfx::SPACING / 2.0;

    let mut version = gfx::Sprite::new();
    version.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface(VERSION, None)));
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

    let cloud_status_rect = gfx::Rect::new(gfx::FloatPos(10.0, 10.0), gfx::FloatSize(20.0, 20.0));

    let mut cloud_status_button = gfx::Button::new(|| {});
    cloud_status_button.color = gfx::GREY;
    cloud_status_button.pos = cloud_status_rect.pos + gfx::FloatPos(cloud_status_rect.size.0 + 5.0, -10.0);
    cloud_status_button.orientation = gfx::TOP_LEFT;
    cloud_status_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None));
    cloud_status_button.scale = 1.5;

    *menu_back.get_elements() = vec![
        Box::new(title),
        #[cfg(debug_assertions)]
        Box::new(debug_title),
        Box::new(version),
    ];

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
        let window_container = gfx::Container::new(&graphics, gfx::FloatPos(0.0, 0.0), graphics.get_window_size(), gfx::TOP_LEFT, None);

        if (secondary_menu_back.size.1 - graphics.get_window_size().1).abs() > f32::EPSILON {
            secondary_menu_back.size.1 = window_container.get_absolute_rect().size.1;
            secondary_menu_back.jump_to_target();
        }

        tls_client.as_mut().map_or_else(
            || {},
            |client| {
                client.authenticate();
            },
        );
        while let Some(event) = graphics.get_event() {
            state.on_event(graphics, &event, &secondary_menu_back.get_container(graphics, &window_container));
            global_settings.borrow_mut().update(graphics, settings);
            if *close_secondary_menu.borrow_mut() {
                state = MainMenuState::None;
                *close_secondary_menu.borrow_mut() = false;
            }

            if let gfx::Event::KeyRelease(key, ..) = event {
                // check for every button if it was clicked with the left mouse button
                if key == gfx::Key::MouseLeft {
                    if singleplayer_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        if !matches!(state, MainMenuState::SingleMenu((_, 0))) {
                            let singleplayer_menu = SingleplayerSelector::new(graphics, settings.clone(), global_settings.clone(), close_secondary_menu.clone(), menu_back_timer);
                            state.switch_to((Cell::new(Box::new(singleplayer_menu)), 0), graphics, &secondary_menu_back.get_container(graphics, &window_container));
                        }
                    } else if multiplayer_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        if !matches!(state, MainMenuState::SingleMenu((_, 1))) {
                            let multiplayer_menu = MultiplayerSelector::new(graphics, menu_back_timer, settings.clone(), global_settings.clone(), close_secondary_menu.clone());
                            state.switch_to((Cell::new(Box::new(multiplayer_menu)), 1), graphics, &secondary_menu_back.get_container(graphics, &window_container));
                        }
                    } else if settings_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        if !matches!(state, MainMenuState::SingleMenu((_, 2))) {
                            let mut settings_menu = SettingsMenu::new(close_secondary_menu.clone(), settings.clone());
                            settings_menu.init(graphics, menu_back.get_back_rect_container());
                            state.switch_to((Cell::new(Box::new(settings_menu)), 2), graphics, &secondary_menu_back.get_container(graphics, &window_container));
                        }
                    } else if mods_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                    } else if exit_button.is_hovered(graphics, menu_back.get_back_rect_container()) {
                        graphics.close_window();
                    } else if cloud_status_button.is_hovered(graphics, &gfx::Container::default(graphics)) {
                        state = MainMenuState::None;
                        menu_back.set_x_position(0.0);
                        if run_login_menu(graphics, menu_back) {
                            if let Some(client) = &mut tls_client {
                                client.reset();
                            }
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
        secondary_menu_back.pos.0 = if matches!(state, MainMenuState::None) {
            graphics.get_window_size().0
        } else {
            max_width / 2.0 - MENU_WIDTH / 2.0
        };

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

        //render secondary menu
        secondary_menu_back.render(graphics, &window_container);
        state.update(graphics, &secondary_menu_back.get_container(graphics, &window_container));
        state.render(graphics, &secondary_menu_back.get_container(graphics, &window_container));

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
