use super::main_menu::{MainMenu, SecondaryMenu};
use crate::client::game::tls_client::{AuthenticationState, TlsClient};
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::singleplayer_selector::MENU_WIDTH;
use crate::client::menus::{Menu, SettingsMenu};
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use crate::shared::tls_client::ConnectionState;
use crate::shared::versions::VERSION;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::rc::Rc;

use super::background_rect::BackgroundRect;
use super::{LoginMenu, MultiplayerSelector, SingleplayerSelector};

enum TitleScreenState {
    MainMenu,
    BothMenus,
    SecondaryMenu,
}

struct MenuRenderer {
    main_menu: MainMenu,
    secondary_menu: SecondaryMenu,
    state: TitleScreenState,
}

impl MenuRenderer {
    fn on_event(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, main_back_container: &gfx::Container, secondary_back_container: &gfx::Container) -> bool {
        match self.state {
            TitleScreenState::MainMenu => self.main_menu.on_event(graphics, event, main_back_container),
            TitleScreenState::BothMenus => self.main_menu.on_event(graphics, event, main_back_container) || self.secondary_menu.on_event(graphics, event, secondary_back_container),
            TitleScreenState::SecondaryMenu => self.secondary_menu.on_event(graphics, event, secondary_back_container),
        }
    }

    fn update(&mut self, graphics: &mut gfx::GraphicsContext, main_back_container: &gfx::Container, secondary_back_container: &gfx::Container) {
        match self.state {
            TitleScreenState::MainMenu => self.main_menu.update(graphics, main_back_container),
            TitleScreenState::BothMenus => {
                self.main_menu.update(graphics, main_back_container);
                self.secondary_menu.update(graphics, secondary_back_container);
            }
            TitleScreenState::SecondaryMenu => self.secondary_menu.update(graphics, secondary_back_container),
        }
    }

    fn render(&mut self, graphics: &mut gfx::GraphicsContext, main_back_container: &gfx::Container, secondary_back_container: &gfx::Container) {
        match self.state {
            TitleScreenState::MainMenu => self.main_menu.render(graphics, main_back_container),
            TitleScreenState::BothMenus => {
                self.main_menu.render(graphics, main_back_container);
                self.secondary_menu.render(graphics, secondary_back_container);
            }
            TitleScreenState::SecondaryMenu => self.secondary_menu.render(graphics, secondary_back_container),
        }
    }
}

pub fn run_title_screen(graphics: &mut gfx::GraphicsContext, settings: &Rc<RefCell<Settings>>, global_settings: &Rc<RefCell<GlobalSettings>>) {
    let open_secondary_menu: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(None));
    let mut menus = MenuRenderer {
        main_menu: MainMenu::new(graphics, &open_secondary_menu),
        secondary_menu: SecondaryMenu::None,
        state: TitleScreenState::MainMenu,
    };

    let mut main_back_rect = super::MenuBack::new(graphics);

    let mut max_width = 0.0;
    for button in menus.main_menu.get_sub_elements() {
        if button.get_container(graphics, &gfx::Container::default(graphics)).rect.size.0 > max_width {
            max_width = button.get_container(graphics, &gfx::Container::default(graphics)).rect.size.0;
        }
    }
    main_back_rect.set_back_rect_width(max_width + 100.0, true);

    let mut secondary_back_rect = gfx::RenderRect::new(gfx::FloatPos(graphics.get_window_size().0, 0.0), gfx::FloatSize(MENU_WIDTH, graphics.get_window_size().1));
    secondary_back_rect.orientation = gfx::TOP;
    secondary_back_rect.blur_radius = gfx::BLUR;
    secondary_back_rect.smooth_factor = 60.0;
    secondary_back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    secondary_back_rect.fill_color.a = gfx::TRANSPARENCY;
    secondary_back_rect.border_color = gfx::BORDER_COLOR;

    let cloud_status_rect = gfx::Rect::new(gfx::FloatPos(10.0, 10.0), gfx::FloatSize(20.0, 20.0));
    let copied_open_secondary_menu = open_secondary_menu.clone();
    let mut cloud_status_button = gfx::Button::new(move || *RefCell::borrow_mut(&copied_open_secondary_menu) = Some(0));
    cloud_status_button.color = gfx::GREY;
    cloud_status_button.pos = cloud_status_rect.pos + gfx::FloatPos(cloud_status_rect.size.0 + 5.0, -10.0);
    cloud_status_button.orientation = gfx::TOP_LEFT;
    cloud_status_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Login", None));
    cloud_status_button.scale = 1.5;

    let tls_client = match TlsClient::new() {
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
        let window_container = gfx::Container::default(graphics);
        let main_back_container = main_back_rect.get_container(graphics, &window_container);
        let secondary_back_container = secondary_back_rect.get_container(graphics, &window_container);

        while let Some(event) = graphics.get_event() {
            menus.on_event(graphics, &event, &main_back_container, &secondary_back_container);
        }

        if let Some(i) = *open_secondary_menu.borrow() {
            if menus
                .secondary_menu
                .open_secondary_menu(graphics, i, settings.clone(), global_settings.clone(), main_back_rect.get_timer(), &secondary_back_rect)
            {
                menus.state = TitleScreenState::BothMenus;
            }
        }
        if matches!(menus.state, TitleScreenState::MainMenu) && secondary_back_container.get_absolute_rect().pos.0 > graphics.get_window_size().0 {
            menus.secondary_menu = SecondaryMenu::None;
        }

        *open_secondary_menu.borrow_mut() = None;

        if menus.secondary_menu.should_close() {
            menus.state = TitleScreenState::MainMenu;
        }

        if (secondary_back_rect.size.1 - graphics.get_window_size().1).abs() > f32::EPSILON {
            secondary_back_rect.size.1 = window_container.get_absolute_rect().size.1;
            secondary_back_rect.jump_to_target();
        }

        main_back_rect.update(graphics, &window_container);
        secondary_back_rect.update(graphics, &window_container);
        menus.update(graphics, &main_back_container, &secondary_back_container);

        main_back_rect.render(graphics, &window_container);
        secondary_back_rect.render(graphics, &window_container);
        menus.render(graphics, &main_back_container, &secondary_back_container);

        let color = get_tls_status_color(&tls_client);
        cloud_status_rect.render(graphics, color);
        cloud_status_button.render(graphics, &gfx::Container::default(graphics));

        let max_width = main_back_container.rect.size.0 + gfx::SPACING + secondary_back_container.rect.size.0;
        match menus.state {
            TitleScreenState::MainMenu => {
                main_back_rect.set_x_position(0.0);
                secondary_back_rect.pos.0 = graphics.get_window_size().0 + secondary_back_rect.size.0;
            }
            TitleScreenState::BothMenus => {
                main_back_rect.set_x_position(-max_width / 2.0 + main_back_container.rect.size.0 / 2.0);
                secondary_back_rect.pos.0 = max_width / 2.0 - secondary_back_container.rect.size.0 / 2.0;
            }
            TitleScreenState::SecondaryMenu => {
                main_back_rect.set_x_position(-(graphics.get_window_size().0 + main_back_container.rect.size.0));
                secondary_back_rect.pos.0 = 0.0;
            }
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
