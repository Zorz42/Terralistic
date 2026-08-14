use crate::client::global_settings::GlobalSettings;
use crate::client::menus::singleplayer_selector::MENU_WIDTH;
use crate::client::menus::{MainMenu, SecondaryMenu};
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::background_rect::BackgroundRect;
use super::MenuBack;

// when menu is off-screen, how much is it off-screen by
use crate::libraries::graphics::UiContext;
const INVISIBLE_PADDING: f32 = 50.0;

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
    let open_secondary_menu: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));
    let mut menus = MenuRenderer {
        main_menu: MainMenu::new(graphics, &open_secondary_menu),
        secondary_menu: SecondaryMenu::None,
        state: TitleScreenState::MainMenu,
    };

    let mut main_back_rect = MenuBack::new(graphics);

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

    while graphics.is_window_open() {
        position_back_menus(graphics, &menus.state, &mut main_back_rect, &mut secondary_back_rect);

        let window_container = gfx::Container::default(graphics);
        main_back_rect.update(graphics, &window_container);
        secondary_back_rect.update(graphics, &window_container);
        let main_back_container = main_back_rect.get_container(graphics, &window_container);
        let secondary_back_container = secondary_back_rect.get_container(graphics, &window_container);

        while let Some(event) = graphics.get_event() {
            menus.on_event(graphics, &event, &main_back_container, &secondary_back_container);
        }

        if let Some(i) = open_secondary_menu.get() {
            if menus.secondary_menu.open_secondary_menu(graphics, i, settings.clone(), global_settings.clone(), &secondary_back_rect) {
                menus.state = TitleScreenState::BothMenus;
            }
        }
        if matches!(menus.state, TitleScreenState::MainMenu) && secondary_back_container.get_absolute_rect().pos.0 > graphics.get_window_size().0 - gfx::SPACING {
            menus.secondary_menu = SecondaryMenu::None;
        }

        if let SecondaryMenu::SingleMenu(menu_stack_) = &menus.secondary_menu {
            if let Some(menu) = menu_stack_.0.get_top_menu() {
                if menu.1.starts_with("f ") {
                    menus.state = TitleScreenState::SecondaryMenu;
                } else if matches!(menus.state, TitleScreenState::SecondaryMenu) {
                    menus.state = TitleScreenState::BothMenus;
                }
            }
        }

        open_secondary_menu.set(None);

        if menus.secondary_menu.should_close() {
            menus.state = TitleScreenState::MainMenu;
        }

        if (secondary_back_rect.size.1 - graphics.get_window_size().1).abs() > f32::EPSILON {
            secondary_back_rect.size.1 = window_container.get_absolute_rect().size.1;
            secondary_back_rect.jump_to_target();
        }

        menus.update(graphics, &main_back_container, &secondary_back_container);

        main_back_rect.render(graphics, &window_container);
        secondary_back_rect.render(graphics, &window_container);
        menus.render(graphics, &main_back_container, &secondary_back_container);

        graphics.update_window();
    }
}

fn position_back_menus(graphics: &gfx::GraphicsContext, state: &TitleScreenState, main_back_rect: &mut MenuBack, secondary_back_rect: &mut gfx::RenderRect) {
    let max_width = main_back_rect.get_back_rect_width() + gfx::SPACING + secondary_back_rect.render_size.0;
    match state {
        TitleScreenState::MainMenu => {
            main_back_rect.set_x_position(0.0);
            secondary_back_rect.pos.0 = f32::midpoint(graphics.get_window_size().0, secondary_back_rect.size.0) + INVISIBLE_PADDING;
        }
        TitleScreenState::BothMenus => {
            main_back_rect.set_x_position(-max_width / 2.0 + main_back_rect.get_back_rect_width() / 2.0);
            secondary_back_rect.pos.0 = max_width / 2.0 - secondary_back_rect.render_size.0 / 2.0;
        }
        TitleScreenState::SecondaryMenu => {
            main_back_rect.set_x_position(-(graphics.get_window_size().0 + main_back_rect.get_back_rect_width()) / 2.0 - INVISIBLE_PADDING);
            secondary_back_rect.pos.0 = 0.0;
        }
    }
}
