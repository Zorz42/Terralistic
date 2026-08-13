use serde_derive::{Deserialize, Serialize};

/// The build's version, used both for the network handshake and for stamping world saves.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The version of the world save layout, bumped by hand whenever the contents of the
/// world file change shape.
///
/// This is separate from `VERSION` because the save format only changes occasionally,
/// and a normal release should not invalidate everyone's worlds.
///
/// 1: the original layout, written with bincode 1 (fixed width integers)
/// 2: the same layout written with bincode 2 (tagged variable length integers)
/// 3: the same layout written with postcard, behind the header below
pub const WORLD_SAVE_VERSION: u32 = 3;

/// What every world file starts with, ahead of anything a serializer wrote.
///
/// **This exists so that a change of encoding stays diagnosable.** Versions 1 and 2 kept the
/// save version *inside* the serialized map, which works right up until the encoding is the
/// thing that changed - and then the version cannot be read either, and a player who upgrades
/// gets "Hit the end of buffer, expected more data" instead of being told their world is from
/// an older build. The magic and the version are fixed width little endian bytes that no
/// serializer touches, so they stay readable across any future format change.
pub const WORLD_SAVE_MAGIC: &[u8; 8] = b"TERRAWLD";

/// Magic plus a `u32` version.
pub const WORLD_SAVE_HEADER_LEN: usize = WORLD_SAVE_MAGIC.len() + 4;

/// Sent by the client as the first packet of the handshake, before `NamePacket`.
///
/// Packet ids are a hash of `TypeId`, which is not stable across compiler versions, so a
/// client and server built by different toolchains simply fail to recognise each other's
/// packets and hang with no explanation. This makes the mismatch explicit and reportable.
#[derive(Serialize, Deserialize)]
pub struct VersionPacket {
    pub version: String,
}

impl VersionPacket {
    #[must_use]
    pub fn current() -> Self {
        Self { version: VERSION.to_owned() }
    }
}
