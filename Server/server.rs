use crate::server_module::ServerModule;

pub struct Server {
    tps_limit: f64,
    modules: Vec<Box<dyn ServerModule>>,
    running: bool,
}

impl Server {
    pub fn new() -> Server {
        Server {
            tps_limit: 20.0,
            modules: vec![

            ],
            running: false,
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
        self.running = true;
        while self.running {
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

    pub fn stop(&mut self) {
        self.running = false;
    }
}