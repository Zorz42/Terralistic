use crate::game::core::Game;
use crate::menus::{run_loading_screen, BackgroundRect};
use graphics::GraphicsContext;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use terralistic_server::Server;
use terralistic_server::SINGLEPLAYER_PORT;

pub fn run_private_world(
    graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect, world_path: &PathBuf,
) {
    let server_running = Arc::new(Mutex::new(true));
    let server_running2 = server_running.clone();

    let mut game = Game::new(SINGLEPLAYER_PORT, String::from("127.0.0.1"));

    let loading_text = Arc::new(Mutex::new("Loading".to_string()));
    let loading_text2 = loading_text.clone();

    // start server in async thread
    let world_path2 = world_path.clone();
    let server_thread = std::thread::spawn(move || {
        let mut server = Server::new(SINGLEPLAYER_PORT);
        server.start(
            &server_running2,
            &loading_text2,
            vec![include_bytes!("../../BaseGame/BaseGame.mod").to_vec()],
            &world_path2,
        );
    });

    run_loading_screen(graphics, menu_back, &loading_text);

    game.run(graphics, menu_back);

    // stop server
    *server_running.lock().unwrap() = false;

    *loading_text.lock().unwrap() = "Waiting for server".to_string();
    run_loading_screen(graphics, menu_back, &loading_text);

    server_thread.join().unwrap();
}
