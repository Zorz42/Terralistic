use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::Result;

use crate::client::game::core_client::run_game;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::{LoadingScreen, Menu};
use crate::client::settings::Settings;
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
    Stopped,
}

pub struct PrivateWorld {
    server_thread: Option<std::thread::JoinHandle<std::result::Result<(), anyhow::Error>>>,
    server_running: Arc<AtomicBool>,
    loading_text: Arc<Mutex<String>>,
    state: PrivateWorldState,
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
                    let res = run_game(graphics, SINGLEPLAYER_PORT, String::from("127.0.0.1"), "_", &self.settings, &self.global_settings);

                    if let Err(e) = res {
                        println!("{e}");
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
                        Err(e) => println!("{e:?}"),
                        Ok(res) => {
                            if let Err(e) = res {
                                println!("{e}");
                            }
                        }
                    }

                    self.state = PrivateWorldState::Stopped;
                }
                None
            }
            _ => None,
        }
    }

    fn on_focus(&mut self, _: &gfx::GraphicsContext) {}
}
