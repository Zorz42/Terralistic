use std::sync::Mutex;
use graphics::GraphicsContext;
use shared_mut::SharedMut;
use terralistic_server::{Server, ServerState};
use crate::game::game::Game;

const SINGLEPLAYER_PORT: u16 = 49152;

pub fn run_private_world(graphics: &mut GraphicsContext) {
    let server_running = SharedMut::new(true);
    let server_running2 = server_running.duplicate();

    let server_state = SharedMut::new(ServerState::Stopped);
    let server_state2 = server_state.duplicate();

    // start server in async thread
    let server_thread = std::thread::spawn(|| {
        let mut server = Server::new(server_running2, server_state2, SINGLEPLAYER_PORT);
        server.start();
    });

    while *server_state.borrow() != ServerState::Running {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // start client
    let mut game = Game::new(SINGLEPLAYER_PORT, String::from("127.0.0.1"));
    game.run(graphics);

    // stop server
    *server_running.borrow() = false;

    server_thread.join().unwrap();
}