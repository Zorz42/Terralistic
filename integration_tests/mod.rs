//! Tests that drive several subsystems together, rather than one module in isolation.
//!
//! The unit tests beside each module check that a piece behaves; these check that the
//! pieces still agree with each other - that a packet written by the server is understood
//! by the client, that a world written by `save_world` is read back by `load_world`, that
//! the lua in `base_game` still registers the content the rust side looks up by name.
//!
//! They are ordinary `#[test]`s in the same binary as everything else, because the crate
//! has no library target for a `tests/` directory to link against.
#![cfg(test)]

pub mod harness;

mod client_server;
mod mods;
mod networking;
mod server_lifecycle;
mod world_persistence;
