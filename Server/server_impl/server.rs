mod server_module;

use std::borrow::Borrow;
use std::cell::Cell;
use crate::server_module::ServerModule;

pub struct Server {
    tps_limit: f64,
    modules: Vec<Box<dyn ServerModule>>,
    running: Cell<bool>,
}

impl Server {
    pub fn new(running: Cell<bool>) -> Server {
        Server {
            tps_limit: 20.0,
            modules: vec![

            ],
            running,
        }
    }

    pub fn start(&mut self) {
        println!("Starting server...");
        // init modules
        for module in &mut self.modules {
            module.init();
        }

        // start server loop
        println!("Server started!");
        let mut last_time = std::time::Instant::now();
        while self.running.borrow().get() {
            let delta_time = last_time.elapsed().as_secs_f64();
            last_time = std::time::Instant::now();

            // update modules
            for module in &mut self.modules {
                module.update(delta_time as f32);
            }

            // sleep
            let sleep_time = 1.0 / self.tps_limit - delta_time;
            if sleep_time > 0.0 {
                std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
            }
        }

        // stop modules
        for module in &mut self.modules {
            module.stop();
        }

        println!("Server stopped.");
    }
}