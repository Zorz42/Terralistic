use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::Result;

use crate::client::game::core_client::run_game;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::choice_menu::ChoiceMenu;
use crate::client::menus::{LoadingScreen, Menu};
use crate::libraries::config::Settings;
use crate::libraries::graphics as gfx;
use crate::server::server_core::Server;
use crate::server::server_core::{BindAddress, SINGLEPLAYER_PORT};

/// Marks the server as gone and empties the loading text, however the server thread ends.
///
/// The loading screen closes when the text empties and nothing else can close it, so a server
/// thread that ends without emptying it leaves the player watching a loading screen for good.
/// A `Drop` covers a panic as well as an error return, which an `if result.is_err()` does not.
///
/// The flag is cleared *before* the text, because clearing the text is what lets the menu move
/// on: the other order leaves a window in which the menu sees the loading screen finish and the
/// server still marked as running, and tries to join a world that is not there.
struct ServerEndedGuard {
    server_running: Arc<AtomicBool>,
    loading_text: Arc<Mutex<String>>,
}

impl Drop for ServerEndedGuard {
    fn drop(&mut self) {
        self.server_running.store(false, Ordering::Relaxed);
        self.loading_text.lock().unwrap_or_else(PoisonError::into_inner).clear();
    }
}

fn start_private_world_server(world_path: &Path) -> Result<(std::thread::JoinHandle<std::result::Result<(), anyhow::Error>>, Arc<AtomicBool>, Arc<Mutex<String>>)> {
    let server_running = Arc::new(AtomicBool::new(true));
    let loading_text = Arc::new(Mutex::new("Loading".to_owned()));

    let guard = ServerEndedGuard {
        server_running: server_running.clone(),
        loading_text: loading_text.clone(),
    };
    let loading_text2 = loading_text.clone();
    let server_running2 = server_running.clone();

    let world_path = world_path.to_owned();

    let server_thread = std::thread::Builder::new().name("Private server".to_owned()).spawn(move || {
        let _guard = guard;
        // loopback only: a singleplayer world must not be reachable from the network
        let mut server = Server::new(SINGLEPLAYER_PORT, BindAddress::Loopback, None, None);
        server.run(&server_running2, &loading_text2, vec![include_bytes!("../../base_game/base_game.mod").to_vec()], &world_path)
    })?;

    Ok((server_thread, server_running, loading_text))
}

#[derive(Clone, Copy)]
enum PrivateWorldState {
    StartingServer,
    Loading,
    Playing,
    StoppingServer,
    ShowingError,
    Stopped,
}

pub struct PrivateWorld {
    server_thread: Option<std::thread::JoinHandle<std::result::Result<(), anyhow::Error>>>,
    server_running: Arc<AtomicBool>,
    loading_text: Arc<Mutex<String>>,
    state: PrivateWorldState,
    /// What went wrong, to be shown once the server has stopped.
    ///
    /// A singleplayer world that fails used to print to the terminal and drop the player back
    /// on the menu with nothing said, which is how a broken world looks exactly like a menu
    /// button that did nothing. Multiplayer has always shown its errors - see
    /// `start_multiplayer.rs` - and this is the same `ChoiceMenu`.
    error: Option<String>,
    settings: Rc<RefCell<Settings>>,
    global_settings: Rc<RefCell<GlobalSettings>>,
}

impl PrivateWorld {
    pub fn new(world_path: &Path, settings: Rc<RefCell<Settings>>, global_settings: Rc<RefCell<GlobalSettings>>) -> Result<Self> {
        let (server_thread, server_running, loading_text) = start_private_world_server(world_path)?;
        Ok(Self {
            server_thread: Some(server_thread),
            server_running,
            loading_text,
            state: PrivateWorldState::StartingServer,
            error: None,
            settings,
            global_settings,
        })
    }
}

impl gfx::UiElement for PrivateWorld {
    fn get_container(&self, graphics: &dyn gfx::UiContext, _: &gfx::Container) -> gfx::Container {
        gfx::Container::default(graphics)
    }
}

impl Menu for PrivateWorld {
    fn should_close(&mut self) -> bool {
        matches!(self.state, PrivateWorldState::Stopped)
    }

    fn open_menu(&mut self, graphics: &mut gfx::GraphicsContext) -> Option<(Box<dyn Menu>, String)> {
        let state = self.state;
        match state {
            //every time we advance the state by one. This is because this won't be called
            //when loading menus are open and we know things are ready to advance when
            //they close
            PrivateWorldState::StartingServer => {
                self.state = PrivateWorldState::Loading;
                Some((Box::new(LoadingScreen::new(self.loading_text.clone())), "f LoadingScreen".to_owned()))
            }
            PrivateWorldState::Loading => {
                self.state = PrivateWorldState::Playing;
                if self.server_running.load(Ordering::Relaxed) {
                    let res = run_game(
                        graphics,
                        SINGLEPLAYER_PORT,
                        String::from("127.0.0.1"),
                        "_",
                        &self.settings,
                        &self.global_settings,
                        Some(&self.server_running),
                    );

                    if let Err(e) = res {
                        println!("{e}");
                        self.error = Some(e.to_string());
                    }

                    // stop server
                    self.server_running.store(false, Ordering::Relaxed);

                    "Waiting for server".clone_into(&mut self.loading_text.lock().unwrap_or_else(PoisonError::into_inner));
                    self.state = PrivateWorldState::StoppingServer;
                    return Some((Box::new(LoadingScreen::new(self.loading_text.clone())), "f LoadingScreen".to_owned()));
                }
                self.state = PrivateWorldState::StoppingServer;
                None
            }
            PrivateWorldState::StoppingServer => {
                if let Some(thread) = self.server_thread.take() {
                    let thread_result = thread.join();

                    match thread_result {
                        Err(e) => {
                            println!("{e:?}");
                            // a panic's payload is not worth showing, but the fact of it is
                            self.error = Some("the world's server crashed - see the console".to_owned());
                        }
                        Ok(res) => {
                            if let Err(e) = res {
                                println!("{e}");
                                // the server's own error is the cause of whatever the client
                                // then made of it, so it wins over one from `run_game`
                                self.error = Some(e.to_string());
                            }
                        }
                    }
                }

                let Some(error) = self.error.take() else {
                    self.state = PrivateWorldState::Stopped;
                    return None;
                };

                self.state = PrivateWorldState::ShowingError;
                Some((
                    Box::new(ChoiceMenu::new(
                        &format!("Could not play this world:\n{error}"),
                        graphics,
                        vec![("Ok", Box::new(|| {}))],
                        Some(0),
                        Some(0),
                    )),
                    "f WorldError".to_owned(),
                ))
            }
            PrivateWorldState::ShowingError => {
                self.state = PrivateWorldState::Stopped;
                None
            }
            _ => None,
        }
    }

    fn on_focus(&mut self, _: &gfx::GraphicsContext) {}
}
