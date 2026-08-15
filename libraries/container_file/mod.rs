//! A versioned binary container: a magic string, a format version, then named sections.
//!
//! **The header is outside the serializer**, because a version kept inside the encoded body
//! works right up until the encoding is what changed - and then the version cannot be read
//! either, so the reader reports a decode error instead of "this file is from an older build".
//! `read_header` checks fixed width bytes before the body is touched.
//!
//! **Not in scope**: what the sections are called and when to bump the version. This knows a
//! file is a header and a map of named byte strings; the owner knows what that means.
//!
//! **The version covers the container, not the contents.** Changing the shape of what goes
//! into a section breaks every existing file silently unless the owner bumps it by hand.

pub use container::*;

mod container;
mod tests;
