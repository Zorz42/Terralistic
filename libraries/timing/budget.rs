use std::time::{Duration, Instant};

/// How much of a frame a piece of optional work is allowed to take.
///
/// A budget is started once and then handed to everything that may spend it, each of which
/// asks `has_time_left` before doing another unit of work and stops when the answer is no.
/// That keeps the frame rate up while a large amount of background work - rebuilding meshes,
/// loading, decompressing - is still outstanding, at the cost of the work taking more frames.
///
/// The point of it being a type rather than an `Instant` and a comparison at each call site
/// is that the limit travels with the clock. Passed as a bare `Instant`, the limit is a
/// literal written out again everywhere it is checked, and a caller that forgets the check
/// spends the whole frame without anything saying so.
#[derive(Clone, Copy, Debug)]
pub struct Budget {
    start: Instant,
    limit: Duration,
}

impl Budget {
    #[must_use]
    pub fn new(limit: Duration) -> Self {
        Self { start: Instant::now(), limit }
    }

    #[must_use]
    pub fn of_ms(limit_ms: u64) -> Self {
        Self::new(Duration::from_millis(limit_ms))
    }

    /// A budget that is already spent, so a caller can turn the optional work off entirely.
    #[must_use]
    pub fn exhausted() -> Self {
        Self::new(Duration::ZERO)
    }

    #[must_use]
    pub fn has_time_left(&self) -> bool {
        self.start.elapsed() < self.limit
    }

    /// How long the budget has been running, for whoever wants to report it.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}
