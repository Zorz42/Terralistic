mod background;
mod block_selector;
mod blocks;
mod camera;
mod chat;
pub mod core_client;
mod debug_menu;
mod entities;
mod floating_text;
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
mod prediction;
pub mod private_world;
mod respawn_screen;
mod tests;
mod tests_prediction;
mod walls;
