use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

#[derive(Clone)]
pub enum SliderSelection {
    Slider(i32),
    Choice(i32),
}

#[derive(Clone)]
pub enum Setting {
    Toggle {
        text: String,
        config_label: String,
        toggled: bool,
    },
    Choice {
        text: String,
        config_label: String,
        choices: Vec<String>,
        selected: i32,
    },
    Slider {
        text: String,
        config_label: String,
        upper_limit: i32,
        lower_limit: i32,
        choices: Vec<String>,
        selected: SliderSelection,
    },
}

pub struct Settings {
    settings: HashMap<i32, Setting>,
    config_path: PathBuf,
    config_data: HashMap<String, i32>,
    curr_setting_id: i32,
}

impl Settings {
    /// # Errors
    /// If config file couldn't be read.
    #[must_use]
    pub fn new(config_path: PathBuf) -> Self {
        let data = fs::read_to_string(config_path.clone());

        let config_data: HashMap<String, i32> = data.map_or_else(
            |_| HashMap::new(),
            |data| serde_json::from_str(&data).unwrap_or_default(),
        );
        Self {
            settings: HashMap::new(),
            config_path,
            config_data,
            curr_setting_id: 0,
        }
    }

    /// Adds a new setting, returns the id of the setting.
    pub fn register_setting(&mut self, mut setting: Setting) -> i32 {
        let config_label = match setting.clone() {
            Setting::Toggle { config_label, .. } | Setting::Choice { config_label, .. } | Setting::Slider { config_label, .. } => config_label,
        };

        if let Some(config_value) = self.config_data.get(&config_label) {
            let config_value = *config_value;
            match &mut setting {
                Setting::Toggle { toggled, .. } => {
                    *toggled = config_value != 0;
                }
                Setting::Choice { selected, .. } => {
                    *selected = config_value;
                }
                Setting::Slider {
                    selected,
                    choices,
                    lower_limit,
                    ..
                } => {
                    if config_value < choices.len() as i32 {
                        *selected = SliderSelection::Choice(config_value);
                    } else {
                        *selected = SliderSelection::Slider(
                            config_value - choices.len() as i32 + *lower_limit,
                        );
                    }
                }
            }
        }

        self.curr_setting_id += 1;
        self.settings.insert(self.curr_setting_id, setting);
        self.curr_setting_id
    }

    /// # Errors
    /// If the setting does not exist.
    pub fn remove_setting(&mut self, id: i32) -> Result<()> {
        if !self.settings.contains_key(&id) {
            anyhow::bail!("Setting with id does not exist");
        }
        self.settings.remove(&id);
        Ok(())
    }

    /// # Errors
    /// If id doesn't exist.
    pub fn get_setting_mut(&mut self, id: i32) -> Result<&mut Setting> {
        return self.settings.get_mut(&id).ok_or_else(|| anyhow!("Invalid setting id"));
    }

    /// # Errors
    /// If id doesn't exist.
    pub fn get_setting(&self, id: i32) -> Result<&Setting> {
        return self.settings.get(&id).ok_or_else(|| anyhow!("Invalid setting id"));
    }

    fn save_config(&mut self) -> Result<()> {
        for setting in self.settings.values() {
            let (config_label, value) = match setting {
                Setting::Toggle {
                    config_label,
                    toggled,
                    ..
                } => (config_label.clone(), i32::from(*toggled)),

                Setting::Choice {
                    config_label,
                    selected,
                    ..
                } => (config_label.clone(), *selected),

                Setting::Slider {
                    config_label,
                    selected,
                    choices,
                    lower_limit,
                    ..
                } => {
                    let val = match selected {
                        SliderSelection::Slider(value) => {
                            *value - choices.len() as i32 + *lower_limit
                        }

                        SliderSelection::Choice(value) => *value,
                    };

                    (config_label.clone(), val)
                }
            };

            self.config_data.insert(config_label, value);
        }

        let json_str = serde_json::to_string_pretty(&self.config_data)?;
        fs::write(self.config_path.clone(), json_str)?;
        Ok(())
    }

    #[must_use]
    pub const fn get_all_settings(&self) -> &HashMap<i32, Setting> {
        &self.settings
    }
}
