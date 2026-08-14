//! The game's protocol.
//!
//! The wire format is `libraries::net::Packet` - any serializable type, identified by a hash
//! of that type. What lives here is what those packets *mean* to this game. Most of them are
//! declared next to the subsystem they belong to (`BlockChangePacket` in `shared/blocks`,
//! `InventorySwapPacket` in `shared/inventory`); this module holds the handshake's own and
//! re-exports the wire type so the rest of the game has one place to reach for it.

use serde_derive::{Deserialize, Serialize};

pub use crate::libraries::net::Packet;

mod tests;

/// This packet is sent when all the welcome packets have been sent.
#[derive(Serialize, Deserialize)]
pub struct WelcomeCompletePacket;
