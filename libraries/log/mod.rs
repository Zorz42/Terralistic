//! Timestamped, levelled log lines and one place to send them.
//!
//! `format_timestamp` is the wording, `LogLevel` the severity, and the sink a process-global
//! destination for lines whose producer has no owner to reach through.
//!
//! **Not in scope**: files, rotation, filtering, structured fields. This is a `println!` with a
//! clock and a fan-out, which is what a game server's console needs; more is a logging crate.

pub use log::*;

mod log;
mod tests;
