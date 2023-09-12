use crate::client::settings::{Setting, Settings, SliderSelection};
use crate::libraries::graphics as gfx;

pub struct GlobalSettings {
    blur_setting: i32,
    scale_setting: i32,
    fps_setting: i32,
}

impl GlobalSettings {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blur_setting: -1,
            scale_setting: -1,
            fps_setting: -1,
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
            choices: vec!["Small".to_owned(), "Normal".to_owned(), "Large".to_owned()],
            selected: 1,
        });

        self.fps_setting = settings.register_setting(Setting::Slider {
            text: "Fps limit".to_owned(),
            config_label: "fps_limit".to_owned(),
            upper_limit: 300,
            lower_limit: 5,
            choices: vec!["VSync".to_owned(), "Unlimited".to_owned()],
            selected: SliderSelection::Choice(0),
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
            Ok(Setting::Choice { selected, .. }) => match *selected {
                0 => {
                    graphics.renderer.scale = 0.5;
                }
                1 => {
                    graphics.renderer.scale = 1.0;
                }
                2 => {
                    graphics.renderer.scale = 1.5;
                }
                _ => {}
            },
            _ => {
                println!("Error: Setting not found or is invalid enum");
            }
        }

        match settings.get_setting(self.fps_setting) {
            Ok(Setting::Slider { selected, .. }) => match *selected {
                SliderSelection::Choice(choice) => {
                    match choice {
                        0 => {
                            // TODO: enable vsync
                        }
                        1 => {
                            graphics.renderer.disable_fps_limit();
                        }
                        _ => {}
                    }
                }
                SliderSelection::Slider(value) => {
                    graphics.renderer.set_fps_limit(value as f32);
                }
            },
            _ => {
                println!("Error: Setting not found or is invalid enum");
            }
        }
    }

    pub fn stop(&mut self, settings: &mut Settings) {
        for setting in [self.blur_setting, self.scale_setting, self.fps_setting] {
            if let Err(error) = settings.remove_setting(setting) {
                println!("Error removing setting: {error}");
            }
        }
    }
}
