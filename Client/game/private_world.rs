use std::borrow::{Borrow, BorrowMut};
use graphics::GraphicsContext;
use shared_mut::SharedMut;
use terralistic_server::Server;
use crate::game::game::Game;

pub fn run_private_world(graphics: &mut GraphicsContext) {
    let port = 49152;

    let server_running = SharedMut::new(true);
    let server_running2 = server_running.duplicate();
    // start server in async thread
    let server_thread = std::thread::spawn(|| {
        let mut server = Server::new(server_running2);
        server.start();
    });

    // start client
    let mut game = Game::new(port, String::from("127.0.0.1"));
    game.run(graphics);

    // stop server
    *server_running.borrow() = false;

    server_thread.join().unwrap();
}