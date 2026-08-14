mod background;
mod block_selector;
mod blocks;
mod camera;
mod chat;
mod chunk_tracker;
pub mod core_client;
mod debug_menu;
mod entities;
mod floating_text;
mod framerate_measurer;
mod health;
mod inventory;
mod items;
mod lights;
mod liquids;
mod mod_manager;
mod networking;
/// Reached only by the integration tests, which connect a real client to a real server.
/// The game itself goes through `core_client::run_game`.
#[cfg(test)]
pub use networking::{ClientNetworking, WelcomePacketEvent};
mod pause_menu;
mod players;
pub mod private_world;
mod respawn_screen;
mod tests;
// login disabled: the account server is unreachable, see docs/LOGIN.md
// pub mod tls_client;
mod walls;
