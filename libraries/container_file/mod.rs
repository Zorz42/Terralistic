//! A versioned binary container: a magic string, a format version, then named sections.
//!
//! # Why the header is outside the serializer
//!
//! **A version kept inside the encoded body works right up until the encoding is the thing
//! that changed** - and then the version cannot be read either, so the reader hands the body
//! to a decoder that makes nonsense of it and reports a decode error instead of "this file is
//! from an older build". `read_header` checks fixed width bytes that no serializer touches,
//! before the body is looked at.
//!
//! # Not in scope
//!
//! What the sections are called and what is in them, and when to bump the version. This
//! knows a file is a header and a map of named byte strings; the owner knows what that means.
//!
//! **The version covers the container, not the contents.** Changing the shape of what goes
//! into a section still breaks every existing file, silently, unless the owner bumps its
//! version by hand. That is what the version is for.

pub use container::*;

mod container;
mod tests;
