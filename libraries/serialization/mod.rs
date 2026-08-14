//! The one place the binary format is chosen.
//!
//! Everything that goes over the network, into a world save, or into a `.mod` file passes
//! through here, so the format is defined once rather than at each of the call sites.
//!
//! # Why postcard
//!
//! This was bincode until bincode's maintainers published a release containing nothing but a
//! `compile_error!` and a README beginning "Bincode is now unmaintained". postcard is one of
//! the three alternatives that release recommends, and the one that fits here:
//!
//! - It is driven by serde, so the 59 types that derive `Serialize` across the codebase did
//!   not have to change. The other suggestion with a compatible format, `wincode`, has its
//!   own derive macros and no serde bridge.
//! - **It has a written, versioned wire specification** and has promised format stability
//!   since 1.0, which is a stronger guarantee than bincode ever gave. That matters more here
//!   than anywhere else: a world save carries no description of its own layout, so the
//!   format is effectively part of the file format.
//!
//! `bitcode` was considered and rejected. It is smaller and faster, but its own README lists
//! "stable format across major versions" as a *non-goal* - every major release would quietly
//! invalidate every world anyone had saved.
//!
//! The encoding is little endian with LEB128 variable length integers. bincode 2's varints
//! were tagged differently and bincode 1's were fixed width, so none of the three are
//! interchangeable - see `WORLD_SAVE_VERSION` in `shared/versions.rs`.

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

/// Deserializes a value from bytes.
///
/// Trailing bytes are ignored rather than treated as an error, which is what bincode 2 did
/// before it. Callers that need exactness should check the length themselves.
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    Ok(postcard::from_bytes(bytes)?)
}
