pub mod blocks;
pub mod chat;
pub mod entities;
pub mod inventory;
pub mod items;
pub mod lights;
pub mod liquids;
pub mod mod_data;
pub mod mod_manager;
pub mod packet;
pub mod players;
// login disabled: the account server is unreachable, see docs/LOGIN.md
// pub mod tls_client;
mod tests;
pub mod versions;
pub mod walls;

/// How many blocks across one chunk of the world is.
///
/// The game's own tuning constant, not the grid library's: `Chunks` takes the partition
/// size as an argument precisely so that this number lives with the game that chose it.
/// Everything that partitions the world - the light update counts and the client's three
/// chunk mesh caches - uses this one, so they all line up.
pub const CHUNK_SIZE: i32 = 16;
