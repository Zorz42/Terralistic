use std::borrow::{Borrow, BorrowMut};
use shared_mut::SharedMut;
use terralistic_server::Server;
use crate::game::game::Game;

pub fn run_private_world() {
    let server_running = SharedMut::new(true);
    let server_running2 = server_running.duplicate();
    // start server in async thread
    let server_thread = std::thread::spawn(|| {
        let mut server = Server::new(server_running2);
        server.start();
    });

    // start client
    let mut game = Game::new();
    //game.run();

    // wait 2 seconds
    //std::thread::sleep(std::time::Duration::from_secs(2));

    // stop server
    *server_running.borrow() = false;

    server_thread.join().unwrap();
}