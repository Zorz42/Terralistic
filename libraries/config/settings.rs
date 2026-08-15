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

impl Setting {
    /// The key this setting is persisted under.
    #[must_use]
    pub fn config_label(&self) -> &str {
        match self {
            Self::Toggle { config_label, .. } | Self::Choice { config_label, .. } | Self::Slider { config_label, .. } => config_label,
        }
    }
}

pub struct Settings {
    settings: HashMap<i32, Setting>,
    config_path: PathBuf,
    config_data: HashMap<String, i32>,
    curr_setting_id: i32,
}

impl Settings {
    #[must_use]
    pub fn new(config_path: PathBuf) -> Self {
        let data = fs::read_to_string(config_path.clone());

        let config_data: HashMap<String, i32> = data.map_or_else(|_| HashMap::new(), |data| serde_json::from_str(&data).unwrap_or_default());
        Self {
            settings: HashMap::new(),
            config_path,
            config_data,
            curr_setting_id: 0,
        }
    }

    /// Adds a setting, restoring its saved value if the config has one, and returns its id.
    pub fn register_setting(&mut self, mut setting: Setting) -> i32 {
        if let Some(&value) = self.config_data.get(setting.config_label()) {
            match &mut setting {
                Setting::Toggle { toggled, .. } => *toggled = value != 0,
                Setting::Choice { selected, .. } => *selected = value,
                Setting::Slider { selected, choices, lower_limit, .. } => {
                    *selected = if value < choices.len() as i32 {
                        SliderSelection::Choice(value)
                    } else {
                        SliderSelection::Slider(value - choices.len() as i32 + *lower_limit)
                    };
                }
            }
        }

        self.curr_setting_id += 1;
        self.settings.insert(self.curr_setting_id, setting);
        self.curr_setting_id
    }

    /// Removes a setting, keeping its value in the config so the next registration finds it.
    pub fn remove_setting(&mut self, id: i32) -> Result<()> {
        let Some(setting) = self.settings.get(&id) else {
            anyhow::bail!("Setting with id does not exist");
        };

        let value = match setting {
            Setting::Toggle { toggled, .. } => i32::from(*toggled),
            Setting::Choice { selected, .. } => *selected,
            Setting::Slider { selected, choices, lower_limit, .. } => match selected {
                SliderSelection::Slider(value) => *value + choices.len() as i32 - *lower_limit,
                SliderSelection::Choice(value) => *value,
            },
        };

        self.config_data.insert(setting.config_label().to_owned(), value);
        self.settings.remove(&id);
        Ok(())
    }

    pub fn get_setting_mut(&mut self, id: i32) -> Result<&mut Setting> {
        self.settings.get_mut(&id).ok_or_else(|| anyhow!("Invalid setting id"))
    }

    pub fn get_setting(&self, id: i32) -> Result<&Setting> {
        self.settings.get(&id).ok_or_else(|| anyhow!("Invalid setting id"))
    }

    pub fn save_config(&self) -> Result<()> {
        if !self.settings.is_empty() {
            println!("Warning: not all settings were removed, therefore not saved!");
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
