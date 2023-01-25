use crate::game::game::Game;
use crate::menus::{run_loading_screen, BackgroundRect};
use graphics::GraphicsContext;
use shared_mut::SharedMut;
use std::path::PathBuf;
use terralistic_server::{Server, ServerState};

const SINGLEPLAYER_PORT: u16 = 49152;

pub fn run_private_world(
    graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect, world_path: &PathBuf,
) {
    let server_running = SharedMut::new(true);
    let server_running2 = server_running.clone();

    let server_state = SharedMut::new(ServerState::Stopped);
    let server_state2 = server_state.clone();

    let mut game = Game::new(SINGLEPLAYER_PORT, String::from("127.0.0.1"));

    let loading_text = SharedMut::new(String::from("Loading"));
    let loading_text2 = loading_text.clone();

    // start server in async thread
    let world_path2 = world_path.clone();
    let server_thread = std::thread::spawn(move || {
        let mut server = Server::new(server_running2, server_state2, SINGLEPLAYER_PORT);
        server.start(
            loading_text2,
            vec![include_bytes!("../../BaseGame/BaseGame.mod").to_vec()],
            &world_path2,
        );
    });

    run_loading_screen(graphics, menu_back, loading_text.clone());

    game.run(graphics, menu_back);

    // stop server
    *server_running.borrow() = false;

    *loading_text.borrow() = "Waiting for server".to_string();
    run_loading_screen(graphics, menu_back, loading_text.clone());

    server_thread.join().unwrap();
}
