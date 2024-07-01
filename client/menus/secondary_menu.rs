use super::MenuStack;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::{Menu, SettingsMenu};
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};
use std::cell::RefCell;
use std::rc::Rc;

use super::{LoginMenu, MultiplayerSelector, SingleplayerSelector};

pub enum SecondaryMenu {
    None,
    SingleMenu((MenuStack, usize)),
    Transition {
        top_menu: (MenuStack, usize),
        bottom_menu: (MenuStack, usize),
        direction: bool, //true is down
        transition_state: f32,
        top_menu_container: gfx::Container,
        bottom_menu_container: gfx::Container,
    },
}

impl SecondaryMenu {
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        match self {
            Self::None => {}
            Self::SingleMenu((menu, _id)) => {
                menu.render(graphics, parent_container);
            }
            Self::Transition {
                top_menu: (top_menu, _id_top),
                bottom_menu: (bottom_menu, _id_bot),
                top_menu_container,
                bottom_menu_container,
                ..
            } => {
                top_menu.render(graphics, top_menu_container);
                bottom_menu.render(graphics, bottom_menu_container);
            }
        }
    }

    pub fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let mut stop_transition = false;
        match self {
            Self::None => {}
            Self::SingleMenu((menu, _id)) => {
                menu.update(graphics, parent_container);
            }
            Self::Transition {
                top_menu: (top_menu, _id_top),
                bottom_menu: (bottom_menu, _id_bot),
                direction,
                transition_state,
                top_menu_container,
                bottom_menu_container,
            } => {
                *transition_state += (1.0 - *transition_state) / 5.0;
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
                top_menu.update(graphics, top_menu_container);
                bottom_menu.update(graphics, bottom_menu_container);
            }
        }
        if stop_transition {
            if let Self::Transition { top_menu, bottom_menu, direction, .. } = std::mem::replace(self, Self::None) {
                *self = Self::SingleMenu(if direction { bottom_menu } else { top_menu });
            }
        }
    }

    pub fn on_event(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        match self {
            Self::None => false,
            Self::SingleMenu((menu, _id)) => menu.on_event(graphics, event, parent_container),
            Self::Transition {
                top_menu: (top_menu, _id_top),
                bottom_menu: (bottom_menu, _id_bot),
                ..
            } => top_menu.on_event(graphics, event, parent_container) || bottom_menu.on_event(graphics, event, parent_container),
        }
    }

    pub fn switch_to(&mut self, new_menu: ((Box<dyn Menu>, String), usize), graphics: &gfx::GraphicsContext, parent_container: &gfx::Container) {
        match std::mem::replace(self, Self::None) {
            Self::None => {
                let mut stack = MenuStack::new();
                stack.add_menu(new_menu.0);
                *self = Self::SingleMenu((stack, new_menu.1));
            }
            Self::SingleMenu((menu, id)) => {
                let (top_menu, bottom_menu, direction) = if id < new_menu.1 {
                    let mut stack = MenuStack::new();
                    stack.add_menu(new_menu.0);
                    ((menu, id), (stack, new_menu.1), true)
                } else {
                    let mut stack = MenuStack::new();
                    stack.add_menu(new_menu.0);
                    ((stack, new_menu.1), (menu, id), false)
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
            Self::Transition {
                top_menu,
                bottom_menu,
                direction,
                transition_state,
                top_menu_container,
                bottom_menu_container,
            } => {
                let switching_to = if direction { bottom_menu.1 } else { top_menu.1 };
                let switching_from = if direction { top_menu.1 } else { bottom_menu.1 };
                if new_menu.1 == switching_to {
                    *self = Self::Transition {
                        top_menu,
                        bottom_menu,
                        transition_state,
                        direction,
                        bottom_menu_container,
                        top_menu_container,
                    }
                } else if new_menu.1 == switching_from {
                    *self = Self::Transition {
                        top_menu,
                        bottom_menu,
                        transition_state: 1.0 - transition_state,
                        direction: !direction,
                        bottom_menu_container,
                        top_menu_container,
                    }
                } else {
                    let (top_menu, bottom_menu, direction) = if switching_to < new_menu.1 {
                        let mut stack = MenuStack::new();
                        stack.add_menu(new_menu.0);
                        (if direction { bottom_menu } else { top_menu }, (stack, new_menu.1), true)
                    } else {
                        let mut stack = MenuStack::new();
                        stack.add_menu(new_menu.0);
                        ((stack, new_menu.1), if direction { bottom_menu } else { top_menu }, false)
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

    pub fn should_close(&mut self) -> bool {
        match self {
            Self::None => false,
            Self::SingleMenu((menu, _id)) => menu.should_close(),
            Self::Transition { top_menu, bottom_menu, .. } => top_menu.0.should_close() || bottom_menu.0.should_close(),
        }
    }

    pub fn open_secondary_menu(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        menu_index: usize,
        settings: Rc<RefCell<Settings>>,
        global_settings: Rc<RefCell<GlobalSettings>>,
        menu_back: &dyn UiElement,
    ) -> bool {
        let menu: (Box<dyn Menu>, String) = match menu_index {
            0 => (Box::new(LoginMenu::new(graphics)), "LoginMenu".to_owned()),
            1 => (Box::new(SingleplayerSelector::new(graphics, settings, global_settings)), "SingleplayerSelector".to_owned()),
            2 => {
                let res = MultiplayerSelector::new(graphics, settings, global_settings);
                if let Ok(menu) = res {
                    (Box::new(menu), "MultiplayerSelector".to_owned())
                } else {
                    *self = Self::None;
                    return false;
                }
            }
            3 => {
                let mut menu = SettingsMenu::new(graphics, settings, global_settings);
                menu.init(graphics, &menu_back.get_container(graphics, &gfx::Container::default(graphics)));
                (Box::new(menu), "Settings".to_owned())
            }
            usize::MAX => {
                graphics.close_window();
                return false;
            }
            _ => {
                println!("menu doesn't exist");
                return false;
            }
        };

        if !matches!(self, Self::SingleMenu((_, curr_menu_index)) if *curr_menu_index == menu_index) {
            self.switch_to((menu, menu_index), graphics, &menu_back.get_container(graphics, &gfx::Container::default(graphics)));
        }

        true
    }
}
