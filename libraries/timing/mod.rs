//! Clocks, budgets and limiters.
//!
//! Everything here answers one of three questions: *is it time yet* (`FixedStep`, `Interval`), *how
//! long should I sleep* (`FrameLimiter`), or *have I spent my budget* (`Budget`) - plus
//! `FrameStats` and `DeltaTimer`, which measure rather than decide.
//!
//! These were five hand-rolled implementations, two already fixed for the same class of bug
//! months apart: a counter that overflowed after 24.8 days and a ledger that stalled after
//! four hours. Both were width bugs in an accumulator, and there is now one accumulator.
//!
//! **Not in scope**: doing the sleeping - whoever owns the loop has better uses for that time
//! than blocking in here - and wall-clock dates. Everything measures from a start it chose
//! itself, which is what makes it independent of the system clock changing underneath.

pub use budget::*;
pub use fixed_step::*;
pub use frame_limiter::*;
pub use frame_stats::*;
pub use interval::*;

mod budget;
mod fixed_step;
mod frame_limiter;
mod frame_stats;
mod interval;
mod tests;
