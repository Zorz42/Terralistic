pub mod blocks;
pub mod chat;
pub mod entities;
pub mod inventory;
pub mod items;
pub mod lights;
pub mod liquids;
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

/// What every function the game exposes to lua is called from lua.
///
/// `ScriptHost` adds it, so `add_global_function("get_block", ..)` is `terralistic_get_block`
/// in a mod. It is here rather than in the scripting library because the prefix is what keeps
/// *this* game's names out of a mod's way, and a different host would want its own.
pub const MOD_FUNCTION_PREFIX: &str = "terralistic_";
