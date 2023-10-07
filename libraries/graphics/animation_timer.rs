/// `AnimationTimer` has one function: `is_frame_ready`
/// It accumulates the time elapsed for example: if you wait 1000ms and duration is 100ms,
/// then first 10 times `is_frame_ready` will return true and 11th time it will return false.

pub struct AnimationTimer {
    per_frame: i32,
    start_time: std::time::Instant,
    ms_passed: i32,
}

impl AnimationTimer {
    /// Creates a new `AnimationTimer` with the given duration.
    #[must_use]
    pub fn new(per_frame: i32) -> Self {
        Self {
            per_frame,
            start_time: std::time::Instant::now(),
            ms_passed: 0,
        }
    }

    /// Returns true if the frame is ready to be rendered.
    #[must_use]
    pub fn frame_ready(&mut self) -> bool {
        let elapsed = self.start_time.elapsed().as_millis() as i32;
        if elapsed > self.ms_passed {
            self.ms_passed += self.per_frame;
            true
        } else {
            false
        }
    }
}
