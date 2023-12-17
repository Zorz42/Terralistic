use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::Result;

use crate::client::game::core_client::run_game;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::{run_loading_screen, BackgroundRect};
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use crate::server::server_core::Server;
use crate::server::server_core::SINGLEPLAYER_PORT;

pub fn run_private_world(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect, world_path: &Path, settings: &mut Settings, global_settings: &mut GlobalSettings) -> Result<()> {
    let server_running = Arc::new(AtomicBool::new(true));
    let server_running2 = server_running.clone();

    let loading_text = Arc::new(Mutex::new("Loading".to_owned()));
    let loading_text2 = loading_text.clone();

    // start server in async thread
    let world_path = world_path.to_path_buf();
    let server_thread = std::thread::spawn(move || {
        let mut server = Server::new(SINGLEPLAYER_PORT, None, None);
        let result = server.run(&server_running2, &loading_text2, vec![include_bytes!("../../base_game/base_game.mod").to_vec()], &world_path);

        if result.is_err() {
            loading_text2.lock().unwrap_or_else(PoisonError::into_inner).clear();
            server_running2.store(false, Ordering::Relaxed);
        }

        result
    });

    run_loading_screen(graphics, menu_back, &loading_text);

    if server_running.load(Ordering::Relaxed) {
        run_game(graphics, menu_back, SINGLEPLAYER_PORT, String::from("127.0.0.1"), "_", settings, global_settings)?;

        // stop server
        server_running.store(false, Ordering::Relaxed);

        *loading_text.lock().unwrap_or_else(PoisonError::into_inner) = "Waiting for server".to_owned();
        run_loading_screen(graphics, menu_back, &loading_text);
    }

    let thread_result = server_thread.join();
    thread_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Server thread panicked")))?;

    Ok(())
}
