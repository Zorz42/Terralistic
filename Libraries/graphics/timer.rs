// this function gets current time in milliseconds float
fn get_time() -> f64 {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    in_ms as f64
}

/*
Timer measures time in milliseconds with float precision
*/
pub struct Timer {
    start_time: f64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    pub fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }

    pub fn reset(&mut self) {
        self.start_time = get_time();
    }
}