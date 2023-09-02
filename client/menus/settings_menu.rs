use std::path::{Path, PathBuf};

pub enum SliderSelection {
    Slider(i32),
    Choice(i32),
}

pub enum Setting {
    Toggle {
        text: String,
        toggled: bool,
    },
    Choice {
        text: String,
        choices: Vec<String>,
        selected: i32,
    },
    Slider {
        text: String,
        upper_limit: i32,
        lower_limit: i32,
        choices: Vec<String>,
        selected: SliderSelection,
    },
}

pub struct Settings {
    settings: Vec<Setting>,
}

impl Settings {
    pub fn new(config_path: PathBuf) -> Self {
        todo!();
    }

    pub fn register_setting(&mut self, setting: Setting) -> usize {
        todo!();
    }

    pub fn get_setting(&self, id: usize) -> &Setting {
        todo!();
    }

    pub fn get_config(&self, key: String) -> i32 {
        todo!();
    }

    pub fn write_config(&mut self, key: String, val: i32) {
        todo!();
    }

    pub fn save_config(&self) {
        todo!();
    }
}
