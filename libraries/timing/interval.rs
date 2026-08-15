/// Something that happens every so often, on a clock the caller drives.
///
/// Unlike `FixedStep`, which reads the real clock, an `Interval` is asked "is it due at this time"
/// - so a set of them shares one simulated clock and each still runs at its own rate.
///
/// **A missed interval is not owed**: the next due time is set from *now*, so falling behind
/// costs one late step rather than a burst. `FixedStep::new` chooses the opposite, and the
/// difference is whether anything downstream can tell that a step did not happen.
#[derive(Clone, Copy, Debug)]
pub struct Interval {
    /// How long between occurrences. Zero or less never comes due at all.
    period_ms: f64,
    next_ms: f64,
}

impl Interval {
    /// An interval whose first occurrence is one whole period after `now_ms` - not
    /// immediately, or everything registered gets a free occurrence as it appears.
    #[must_use]
    pub fn starting_at(now_ms: f64, period_ms: f64) -> Self {
        Self {
            period_ms,
            next_ms: now_ms + period_ms,
        }
    }

    /// Whether this interval has come due by `now_ms`, scheduling the next one if it has.
    pub fn is_due(&mut self, now_ms: f64) -> bool {
        if self.period_ms <= 0.0 || now_ms < self.next_ms {
            return false;
        }

        self.next_ms = now_ms + self.period_ms;
        true
    }
}
