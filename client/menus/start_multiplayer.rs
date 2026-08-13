use std::{cell::RefCell, rc::Rc};

use crate::client::game::core_client::run_game;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::choice_menu::ChoiceMenu;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use gfx::UiElement;

use super::multiplayer_selector::ServerInfo;
use super::Menu;
use super::TextInputMenu;

#[derive(Clone, Copy)]
enum MultiplayerState {
    Nothing,
    NameMenu,
    Playing,
    ErrorMenu,
    Finished,
}

pub struct StartMultiplayer {
    state: MultiplayerState,
    player_name: Rc<RefCell<Option<String>>>,
    server: ServerInfo,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
}

impl StartMultiplayer {
    pub fn new(server: ServerInfo, settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Self {
        Self {
            state: MultiplayerState::Nothing,
            player_name: Rc::new(RefCell::new(None)),
            server,
            settings,
            global_settings,
        }
    }
}

impl UiElement for StartMultiplayer {
    fn get_container(&self, graphics: &dyn crate::libraries::graphics::UiContext, parent_container: &crate::libraries::graphics::Container) -> crate::libraries::graphics::Container {
        gfx::Container::new(graphics, parent_container.rect.pos, parent_container.rect.size, parent_container.orientation, None)
    }
}

impl Menu for StartMultiplayer {
    fn should_close(&mut self) -> bool {
        matches!(self.state, MultiplayerState::Finished)
    }

    fn open_menu(&mut self, graphics: &mut crate::libraries::graphics::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        match self.state {
            MultiplayerState::Nothing => {
                self.state = MultiplayerState::NameMenu;
                let name_input_menu = TextInputMenu::new(graphics, "Enter your username", self.player_name.clone());
                Some((Box::new(name_input_menu), "f NameInput".to_owned()))
            }
            MultiplayerState::NameMenu => {
                self.state = MultiplayerState::Playing;
                let game_result = run_game(graphics, self.server.port, self.server.ip.clone(), &self.player_name.take()?, &self.settings, &self.global_settings);
                if let Err(error) = game_result {
                    println!("Game error: {error}");
                    self.state = MultiplayerState::ErrorMenu;
                    return Some((
                        Box::new(ChoiceMenu::new(&format!("Game error: {error}"), graphics, vec![("Ok", Box::new(|| {}))], None, None)),
                        "GameError".to_owned(),
                    ));
                }
                self.state = MultiplayerState::Finished;
                None
            }
            _ => {
                self.state = MultiplayerState::Finished;
                None
            }
        }
    }
}
