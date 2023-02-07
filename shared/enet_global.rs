#![allow(clippy::unwrap_used)]
// in this case unwrap is used because if the enet global is not initialized,
// there is no way to recover from that. There should not be any more instances
// of enet being initialized after the global is initialized or else it will
// panic.

use enet::Enet;
use once_cell::sync::Lazy;

pub static ENET_GLOBAL: Lazy<Enet> = Lazy::new(|| Enet::new().unwrap());
