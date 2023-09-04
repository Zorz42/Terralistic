use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;

pub struct GlobalSettings {
    blur_setting: i32,
}

impl GlobalSettings {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blur_setting: -1,
        }
    }

    pub fn init(&mut self, settings: &mut Settings) {
        self.blur_setting = settings.register_setting(Setting::Toggle {
            text: "Blur effect".to_owned(),
            config_label: "blur_effect".to_owned(),
            toggled: true,
        });
    }

    pub fn update(&mut self, graphics: &mut gfx::GraphicsContext, settings: &Settings) {
        match settings.get_setting(self.blur_setting) {
            Ok(Setting::Toggle { toggled, .. }) => {
                graphics.renderer.enable_blur(*toggled);
            }
            _ => {
                println!("Error: Setting not found or is invalid enum");
            }
        }
    }
}