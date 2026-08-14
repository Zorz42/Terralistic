//! The bits of a test harness that every other library ends up rewriting.
//!
//! A directory that cleans itself up, a port nothing else in the process is using, and a
//! spin-until-or-fail. All of it is `#[cfg(test)]`, so none of it reaches the shipped binary.
//!
//! # Why this exists
//!
//! The crate has no dev-dependencies, and a library's own `tests.rs` cannot reach the game's
//! integration harness. So every library that touches a socket or the filesystem writes its
//! own `free_port` and `wait_until` - which had already happened twice by the time this was
//! extracted, with two different port ranges that could collide.
//!
//! # Not in scope
//!
//! Anything that knows what is being tested. Fixtures, drivers and "start a server and join
//! it" belong to whoever is testing that thing.

#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // a failing helper should fail the test loudly

pub use testing::*;

mod testing;
