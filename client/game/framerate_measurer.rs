use std::time::Instant;

pub struct FramerateMeasurer {
    ms_timer: Instant,
    ms_counter: i32,
    prev_time: Instant,

    fps_counter: i32,
    frame_time_counter: f32,
    max_frame_time: f32,
    stat_update_timer: Instant,

    fps_stat: i32,
    max_frame_time_stat: f32,
    avg_frame_time_stat: f32,

    delta_time: f32,
}

impl FramerateMeasurer {
    pub fn new() -> Self {
        Self {
            ms_timer: Instant::now(),
            ms_counter: 0,
            prev_time: Instant::now(),

            fps_counter: 0,
            frame_time_counter: 0.0,
            max_frame_time: 0.0,
            stat_update_timer: Instant::now(),

            fps_stat: 0,
            max_frame_time_stat: 0.0,
            avg_frame_time_stat: 0.0,

            delta_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.delta_time = self.prev_time.elapsed().as_secs_f32() * 1000.0;
        self.prev_time = Instant::now();

        if self.stat_update_timer.elapsed().as_secs() >= 1 {
            self.fps_stat = self.fps_counter;
            self.max_frame_time_stat = self.max_frame_time;
            self.avg_frame_time_stat = self.frame_time_counter / self.fps_counter as f32;

            self.fps_counter = 0;
            self.max_frame_time = 0.0;
            self.frame_time_counter = 0.0;
            self.stat_update_timer = Instant::now();
        }
    }

    pub fn update_post_render(&mut self) {
        self.fps_counter += 1;
        self.frame_time_counter += self.prev_time.elapsed().as_secs_f32() * 1000.0;
        self.max_frame_time = f32::max(
            self.max_frame_time,
            self.prev_time.elapsed().as_secs_f32() * 1000.0,
        );
    }

    pub fn has_5ms_passed(&mut self) -> bool {
        let result = self.ms_counter < self.ms_timer.elapsed().as_millis() as i32;
        if result {
            self.ms_counter += 5;
        }
        result
    }

    pub const fn get_fps(&self) -> i32 {
        self.fps_stat
    }

    pub const fn get_max_frame_time(&self) -> f32 {
        self.max_frame_time_stat
    }

    pub const fn get_avg_frame_time(&self) -> f32 {
        self.avg_frame_time_stat
    }

    pub const fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}
