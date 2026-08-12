/// `AnimationTimer` has one function: `frame_ready`.
///
/// It accumulates the time elapsed, for example: if you wait 1000ms and duration is 100ms,
/// then first 10 times `frame_ready` will return true and 11th time it will return false.
///
/// The counters are 64 bit because they are absolute milliseconds since the timer was made,
/// not a delta. As `i32` they overflowed after 24.8 days of uptime, at which point the
/// elapsed time compared as negative and every animation in the game stopped for good.
#[derive(Debug)]
pub struct AnimationTimer {
    per_frame: i64,
    start_time: std::time::Instant,
    ms_passed: i64,
}

impl AnimationTimer {
    /// Creates a new `AnimationTimer` with the given duration.
    #[must_use]
    pub fn new(per_frame: i64) -> Self {
        Self {
            per_frame,
            start_time: std::time::Instant::now(),
            ms_passed: 0,
        }
    }

    /// Stops the timer from ever reporting another ready frame.
    ///
    /// The golden-image tests use this to pin animations that are driven by elapsed wall
    /// clock time, which would otherwise make a capture depend on how long the run took to
    /// reach that widget.
    #[cfg(feature = "render-tests")]
    pub const fn freeze(&mut self) {
        self.ms_passed = i64::MAX;
    }

    /// Returns true if the frame is ready to be rendered.
    #[must_use]
    pub fn frame_ready(&mut self) -> bool {
        let elapsed = self.start_time.elapsed().as_millis() as i64;
        if elapsed > self.ms_passed {
            self.ms_passed += self.per_frame;
            true
        } else {
            false
        }
    }
}
