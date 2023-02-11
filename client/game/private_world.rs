use super::core::Game;
use crate::client::menus::{run_loading_screen, BackgroundRect};
use crate::libraries::graphics::GraphicsContext;
use crate::server::server_impl::Server;
use crate::server::server_impl::SINGLEPLAYER_PORT;
use anyhow::Result;
use core::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;
use std::sync::{Mutex, PoisonError};
extern crate alloc;
use alloc::sync::Arc;

/// # Errors
/// Returns an error if the server thread panics or if the server thread returns an error.
pub fn run_private_world(
    graphics: &mut GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    world_path: &Path,
) -> Result<()> {
    let server_running = Arc::new(AtomicBool::new(true));
    let server_running2 = server_running.clone();

    let mut game = Game::new(SINGLEPLAYER_PORT, String::from("127.0.0.1"));

    let loading_text = Arc::new(Mutex::new("Loading".to_owned()));
    let loading_text2 = loading_text.clone();

    // start server in async thread
    let world_path = world_path.to_path_buf();
    let server_thread = std::thread::spawn(move || {
        let mut server = Server::new(SINGLEPLAYER_PORT);
        let result = server.start(
            &server_running2,
            &loading_text2,
            vec![include_bytes!("../../base_game/base_game.mod").to_vec()],
            &world_path,
        );

        if result.is_err() {
            server_running2.store(false, Ordering::Relaxed);
        }

        result
    });

    run_loading_screen(graphics, menu_back, &loading_text);

    game.run(graphics, menu_back)?;

    // stop server
    server_running.store(false, Ordering::Relaxed);

    *loading_text.lock().unwrap_or_else(PoisonError::into_inner) = "Waiting for server".to_owned();
    run_loading_screen(graphics, menu_back, &loading_text);

    let thread_result = server_thread.join();
    thread_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Server thread panicked")))?;

    Ok(())
}
