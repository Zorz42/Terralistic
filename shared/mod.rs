pub mod blocks;
pub mod chat;
pub mod entities;
pub mod inventory;
pub mod items;
pub mod lights;
pub mod liquids;
pub mod packet;
pub mod players;
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

/// How long one simulation tick is, in milliseconds, and how many of them make a second.
///
/// The simulation is stepped at this rate on both the client and the server - see the
/// `FixedStep::new(TICK_MS)` in each main loop - so it is also the unit every timestamp on
/// the wire is counted in. Physics constants are expressed per second and divided by
/// `TICKS_PER_SECOND` at the point of use, which is what the bare `/ 200` divisors used to be.
pub const TICK_MS: i64 = 5;
pub const TICKS_PER_SECOND: i32 = 1000 / TICK_MS as i32;

/// How far ahead of the server the client runs its own simulation.
///
/// An input stamped for tick T is only useful if it reaches the server before the server
/// simulates T, so the client counts from a tick the server has not reached yet. The server
/// looks at its inbox once per update - 50ms at 20 TPS, ten ticks - and the rest of the
/// budget is network and a margin, which at 100ms covers a loopback and a typical connection.
///
/// Too small and inputs land late, which costs a rollback rather than being wrong. Too large
/// and the player's own actions reach everyone else later than they need to. A fixed lead is
/// chosen here; measuring how early inputs actually arrive and adapting would be better on a
/// bad connection, and is the obvious next change.
pub const INPUT_LEAD_TICKS: u64 = 20;

/// How often the server tells every client where all the entities are, in ticks.
///
/// This was once a second, which for anything moving was long enough that the correction
/// when it arrived was a visible jump. Ten times a second costs one batched packet per two
/// server updates, and - because both sides now simulate the same deterministic physics -
/// most of what it carries agrees with what the client already had.
pub const ENTITY_SYNC_INTERVAL_TICKS: u64 = 20;
