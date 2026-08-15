use std::time::Instant;

/// Measures the time between calls. The first answers `None` rather than the time since
/// construction, which is however long starting up took and not a frame anybody rendered.
#[derive(Default, Debug)]
pub struct DeltaTimer {
    previous: Option<Instant>,
}

impl DeltaTimer {
    #[must_use]
    pub const fn new() -> Self {
        Self { previous: None }
    }

    /// Milliseconds since the previous call, or `None` on the first.
    pub fn tick(&mut self) -> Option<f32> {
        let delta = self.previous.map(|previous| previous.elapsed().as_secs_f32() * 1000.0);
        self.previous = Some(Instant::now());
        delta
    }
}

/// Frame rate and frame time, averaged over the last whole second.
///
/// Wrap the frame in `begin_frame` and `end_frame`: the delta is measured between successive
/// `begin_frame`s and so includes whatever the caller slept, while the frame *time* covers only the
/// work in between. Different numbers on purpose - one is what a simulation advances by, the other
/// is what a performance readout shows.
#[derive(Debug)]
pub struct FrameStats {
    delta_timer: DeltaTimer,
    frame_start: Instant,
    delta_time: f32,

    /// Counters for the second currently being measured.
    stat_window_start: Instant,
    frames: i32,
    frame_time_total: f32,
    max_frame_time: f32,

    /// What the last whole second measured, and what callers read - so the numbers on screen
    /// hold still for a second instead of flickering every frame.
    fps: i32,
    avg_frame_time: f32,
    max_frame_time_stat: f32,
}

impl FrameStats {
    #[must_use]
    pub fn new() -> Self {
        Self {
            delta_timer: DeltaTimer::new(),
            frame_start: Instant::now(),
            delta_time: 0.0,

            stat_window_start: Instant::now(),
            frames: 0,
            frame_time_total: 0.0,
            max_frame_time: 0.0,

            fps: 0,
            avg_frame_time: 0.0,
            max_frame_time_stat: 0.0,
        }
    }

    pub fn begin_frame(&mut self) {
        self.delta_time = self.delta_timer.tick().unwrap_or(0.0);
        self.frame_start = Instant::now();

        if self.stat_window_start.elapsed().as_secs() >= 1 {
            self.fps = self.frames;
            self.max_frame_time_stat = self.max_frame_time;
            self.avg_frame_time = self.frame_time_total / self.frames as f32;

            self.frames = 0;
            self.max_frame_time = 0.0;
            self.frame_time_total = 0.0;
            self.stat_window_start = Instant::now();
        }
    }

    pub fn end_frame(&mut self) {
        let frame_time = self.frame_start.elapsed().as_secs_f32() * 1000.0;
        self.frames += 1;
        self.frame_time_total += frame_time;
        self.max_frame_time = f32::max(self.max_frame_time, frame_time);
    }

    /// Milliseconds from the start of the previous frame to the start of this one.
    #[must_use]
    pub const fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    #[must_use]
    pub const fn get_fps(&self) -> i32 {
        self.fps
    }

    #[must_use]
    pub const fn get_max_frame_time(&self) -> f32 {
        self.max_frame_time_stat
    }

    #[must_use]
    pub const fn get_avg_frame_time(&self) -> f32 {
        self.avg_frame_time
    }
}
