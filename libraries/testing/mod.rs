//! The bits of a test harness every other library ends up rewriting: a directory that cleans itself
//! up, a port nothing else in the process is using, and a spin-until-or-fail.
//!
//! All `#[cfg(test)]`, so none of it reaches the shipped binary.
//!
//! It exists because the crate has no dev-dependencies and a library's own `tests.rs` cannot
//! reach the game's integration harness - which had already produced two `free_port`
//! implementations with different port ranges.
//!
//! **Not in scope**: anything that knows what is being tested. Fixtures and drivers belong to
//! whoever is testing that thing.

#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // a failing helper should fail the test loudly

pub use testing::*;

mod testing;
