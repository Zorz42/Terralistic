//! Deterministic fixed-point numbers: a signed 16.16 fraction in an `i32`.
//!
//! Exists so a simulation can be replayed and compared with `==`. IEEE floats are already
//! reproducible for `+ - * /` and `sqrt`, but a simulation built on them cannot answer
//! *"is this state the same one?"* without an epsilon, and an epsilon is a tuning knob that
//! is either too tight (replays constantly) or too loose (drift hides underneath it).
//! `Fixed` is `Eq + Ord + Hash`, so the question has an exact answer and state can be hashed.
//!
//! Two properties are load-bearing and both are tested:
//!
//! - **Every operation truncates toward zero**, never floors. `>>` would bias every result
//!   towards negative infinity, so a value moving left would decay differently from the same
//!   value moving right - an asymmetry that is invisible in a unit test and obvious in a game.
//! - **Narrowing saturates, it does not wrap.** A runaway value pins at the end of the range
//!   instead of reappearing at the other end.
//!
//! Resolution is 1/65536 and the range is +/-32768. That is chosen against a world measured in
//! whole units a few thousand across: fine enough that the smallest meaningful step is still
//! thousands of increments, wide enough that no coordinate in such a world comes close to the
//! end. Values below the resolution truncate to zero, so things that are slowing down
//! eventually *stop* rather than approaching rest forever.
//!
//! **Not in scope**: transcendental functions. `sin`, `exp` and `powf` are supplied by the
//! platform's libm, are not specified to be correctly rounded, and genuinely differ between
//! platforms - they are the one real source of float non-determinism, and the point of this
//! type is that it cannot reach them. `sqrt` is here because it can be computed exactly in
//! integers. If you need a curve, table it.

pub use fixed::*;

mod fixed;
mod tests;
