pub use core_server::{print_to_console, send_to_ui, Server, MULTIPLAYER_PORT, SINGLEPLAYER_PORT};
pub use networking::BindAddress;

/// Reached only by the integration tests, which drive the networking layer directly
/// against a real client instead of going through `Server`. Not part of the server's
/// surface in a normal build.
#[cfg(test)]
pub use networking::{Connection, DisconnectEvent, NewConnectionEvent, PacketFromClientEvent, SendTarget, ServerNetworking};
/// Same, for the tests that write a world save by hand: the "players" key is a map of
/// these, and naming the type is what makes an empty one obviously the right thing.
#[cfg(test)]
pub use players::SavedPlayerData;

mod blocks;
mod chat;
mod commands;
mod core_server;
mod entities;
mod items;
mod mod_manager;
mod networking;
mod players;
mod tests;
mod walls;
mod world_generator;
