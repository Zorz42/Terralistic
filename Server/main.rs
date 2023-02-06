use std::sync::{Arc, Mutex};
use terralistic_server::Server;
use terralistic_server::MULTIPLAYER_PORT;

fn main() {
    let server_running = Arc::new(Mutex::new(true));

    let loading_text = Arc::new(Mutex::new("Loading".to_string()));

    let path = std::env::current_dir().unwrap().join("server_data");

    let mut server = Server::new(MULTIPLAYER_PORT);
    server.start(
        &server_running,
        &loading_text,
        vec![include_bytes!("../BaseGame/BaseGame.mod").to_vec()],
        &path.join("server.world"),
    );
}
