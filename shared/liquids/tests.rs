#![cfg(test)]
//! There is nothing here to test.
//!
//! `shared/liquids/` is not merely unused, it is entirely commented out: both
//! `liquids.rs` (271 lines) and `liquid_type.rs` (92 lines) are a single `/* ... */` block
//! from their first line to their last, so the module compiles to nothing at all.
//!
//! Nothing can be tested until someone decides whether liquids are coming back. See the
//! open item in `docs/IMPROVEMENTS.md`.
mod tests {}
