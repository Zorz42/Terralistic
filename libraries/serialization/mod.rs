//! The one place the binary format is chosen. Packets, world saves and `.mod` files all pass
//! through here, so the format is defined once rather than at every call site.
//!
//! **Why postcard.** This was bincode until it was published unmaintained. postcard is driven
//! by serde, so the 59 types deriving `Serialize` did not change, and it has a written,
//! versioned wire specification with stability promised since 1.0 - which matters most here,
//! since a world save carries no description of its own layout. `bitcode` is smaller and
//! faster but lists format stability as a *non-goal*, which would invalidate saved worlds.
//!
//! The encoding is little endian with LEB128 varints. bincode 1's were fixed width and
//! bincode 2's tagged differently, so none of the three interchange - see `WORLD_SAVE_VERSION`.

mod tests;

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Serializes a value to bytes.
pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    Ok(postcard::to_stdvec(value)?)
}

/// Serializes a value straight into a writer, without building a `Vec` first.
pub fn serialize_into<W: std::io::Write, T: Serialize>(writer: &mut W, value: &T) -> Result<()> {
    postcard::to_io(value, writer)?;
    Ok(())
}

/// Deserializes a value from bytes. Trailing bytes are ignored, as bincode 2 did before it;
/// callers needing exactness check the length themselves.
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    Ok(postcard::from_bytes(bytes)?)
}
