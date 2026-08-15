use std::time::{Duration, Instant};

/// How much of a frame a piece of optional work may take.
///
/// Started once and handed to everything that may spend it, each asking `has_time_left` before
/// another unit of work - which keeps the frame rate up while meshes rebuild or a world loads, at
/// the cost of more frames.
///
/// A type rather than an `Instant` and a comparison per call site so that the limit travels
/// with the clock: as a bare `Instant` it is a literal rewritten at every check.
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
