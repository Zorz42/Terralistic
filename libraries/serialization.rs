use anyhow::Result;
use bincode::config::Configuration;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// The one place the binary format is chosen. Everything that goes over the network, into
/// a world save, or into a `.mod` file passes through here, so the format is defined once
/// rather than at each of the call sites.
///
/// This is bincode 2's default: little endian with variable length integers. bincode 1,
/// which this project used previously, wrote fixed width integers, so the two are not
/// interchangeable - see `WORLD_SAVE_VERSION` in `shared/versions.rs`.
const CONFIG: Configuration = bincode::config::standard();

/// Serializes a value to bytes.
pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    Ok(bincode::serde::encode_to_vec(value, CONFIG)?)
}

/// Serializes a value straight into a writer, without building a `Vec` first.
pub fn serialize_into<W: std::io::Write, T: Serialize>(writer: &mut W, value: &T) -> Result<()> {
    bincode::serde::encode_into_std_write(value, writer, CONFIG)?;
    Ok(())
}

/// Deserializes a value from bytes.
///
/// Note that unlike bincode 1, this ignores trailing bytes rather than treating them as an
/// error. Callers that need exactness should check the length themselves.
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    Ok(bincode::serde::decode_from_slice(bytes, CONFIG)?.0)
}
