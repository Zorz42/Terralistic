use crate::client::settings::{Setting, Settings};
use crate::libraries::graphics as gfx;

pub struct GlobalSettings {
    blur_setting: i32,
    scale_setting: i32,
}

impl GlobalSettings {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blur_setting: -1,
            scale_setting: -1,
        }
    }

    pub fn init(&mut self, settings: &mut Settings) {
        self.blur_setting = settings.register_setting(Setting::Toggle {
            text: "Blur effect".to_owned(),
            config_label: "blur_effect".to_owned(),
            toggled: true,
        });

        self.scale_setting = settings.register_setting(Setting::Choice {
            text: "Scale".to_owned(),
            config_label: "scale".to_owned(),
            choices: vec!["Large".to_owned(), "Normal".to_owned(), "Small".to_owned()],
            selected: 1,
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

        match settings.get_setting(self.scale_setting) {
            Ok(Setting::Choice { selected, .. }) => {}
            _ => {
                println!("Error: Setting not found or is invalid enum");
            }
        }
    }

    pub fn stop(&mut self, settings: &mut Settings) {
        for setting in [self.blur_setting, self.scale_setting] {
            if let Err(error) = settings.remove_setting(setting) {
                println!("Error removing setting: {error}");
            }
        }
    }
}