//! Timestamped, levelled log lines and one place to send them.
//!
//! `format_timestamp` is the wording, `LogLevel` is the severity, and `LogSink` is a
//! process-global place for lines to go when the code producing them has no owner to reach
//! through.
//!
//! # Not in scope
//!
//! Files, rotation, filtering and structured fields. This is a `println!` with a clock and a
//! fan-out, which is what a game server's console actually needs; anything more is a job for
//! a real logging crate.

pub use log::*;

mod log;
mod tests;
