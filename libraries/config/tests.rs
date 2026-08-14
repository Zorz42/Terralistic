#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::panic)] // a wrong enum variant in a test is a test failure
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![cfg(test)]
mod tests {
    use crate::libraries::config::{Setting, Settings, SliderSelection};
    use std::path::PathBuf;

    /// A unique path in the temp dir, so tests do not collide with each other or with a
    /// real settings file.
    fn temp_config(name: &str) -> PathBuf {
        let unique = format!("terralistic_test_{name}_{}_{:?}.json", std::process::id(), std::thread::current().id());
        std::env::temp_dir().join(unique)
    }

    fn toggle(label: &str, toggled: bool) -> Setting {
        Setting::Toggle {
            text: label.to_owned(),
            config_label: label.to_owned(),
            toggled,
        }
    }

    fn choice(label: &str, selected: i32) -> Setting {
        Setting::Choice {
            text: label.to_owned(),
            config_label: label.to_owned(),
            choices: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            selected,
        }
    }

    fn slider(label: &str, selected: SliderSelection) -> Setting {
        Setting::Slider {
            text: label.to_owned(),
            config_label: label.to_owned(),
            upper_limit: 300,
            lower_limit: 5,
            choices: vec!["VSync".to_owned(), "Unlimited".to_owned()],
            selected,
        }
    }

    #[test]
    fn test_new_settings_has_none_registered() {
        let settings = Settings::new(temp_config("empty"));
        assert!(settings.get_all_settings().is_empty());
    }

    #[test]
    fn test_missing_config_file_is_not_an_error() {
        let path = temp_config("missing");
        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine

        // reading a config that is not there just starts from defaults
        let settings = Settings::new(path);
        assert!(settings.get_all_settings().is_empty());
    }

    #[test]
    fn test_register_returns_distinct_ids() {
        let mut settings = Settings::new(temp_config("ids"));

        let a = settings.register_setting(toggle("a", true));
        let b = settings.register_setting(toggle("b", false));

        assert_ne!(a, b);
        assert_eq!(settings.get_all_settings().len(), 2);
    }

    /// An id is a handle, not a position, and a removed setting does not give its number back.
    /// The in-game lights toggle is registered when a world loads and removed when it closes,
    /// so it is a higher id on every world opened - which is why the settings menu lays its
    /// rows out by list position rather than by id.
    #[test]
    fn test_a_removed_setting_does_not_free_its_id() {
        let mut settings = Settings::new(temp_config("id_reuse"));

        let first = settings.register_setting(toggle("lights", true));
        settings.remove_setting(first).unwrap();
        let second = settings.register_setting(toggle("lights", true));

        assert_ne!(first, second, "a reused id would put two settings in one place");
    }

    #[test]
    fn test_get_setting_by_id() {
        let mut settings = Settings::new(temp_config("get"));
        let id = settings.register_setting(toggle("blur", true));

        match settings.get_setting(id).unwrap() {
            Setting::Toggle { toggled, text, .. } => {
                assert!(*toggled);
                assert_eq!(text, "blur");
            }
            _ => panic!("expected a toggle"),
        }
    }

    #[test]
    fn test_unknown_setting_id_is_an_error() {
        let mut settings = Settings::new(temp_config("unknown"));
        assert!(settings.get_setting(999).is_err());
        assert!(settings.get_setting_mut(999).is_err());
        settings.remove_setting(999).unwrap_err();
    }

    #[test]
    fn test_get_setting_mut_allows_changes() {
        let mut settings = Settings::new(temp_config("mut"));
        let id = settings.register_setting(toggle("blur", false));

        if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(id) {
            *toggled = true;
        }

        match settings.get_setting(id).unwrap() {
            Setting::Toggle { toggled, .. } => assert!(*toggled),
            _ => panic!("expected a toggle"),
        }
    }

    #[test]
    fn test_removing_a_setting_takes_it_out() {
        let mut settings = Settings::new(temp_config("remove"));
        let id = settings.register_setting(toggle("blur", true));

        settings.remove_setting(id).unwrap();

        assert!(settings.get_all_settings().is_empty());
        assert!(settings.get_setting(id).is_err());
    }

    /// A setting's value is only written to the config when it is removed, and the config
    /// is only flushed to disk on save. This is the full lifecycle the client relies on.
    #[test]
    fn test_toggle_survives_a_save_and_reload() {
        let path = temp_config("toggle_round_trip");
        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine

        let mut settings = Settings::new(path.clone());
        let id = settings.register_setting(toggle("blur_effect", false));
        if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(id) {
            *toggled = true;
        }
        settings.remove_setting(id).unwrap();
        settings.save_config().unwrap();

        let mut reloaded = Settings::new(path.clone());
        let id = reloaded.register_setting(toggle("blur_effect", false));
        match reloaded.get_setting(id).unwrap() {
            Setting::Toggle { toggled, .. } => assert!(*toggled, "the saved value should win over the default"),
            _ => panic!("expected a toggle"),
        }

        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine
    }

    #[test]
    fn test_choice_survives_a_save_and_reload() {
        let path = temp_config("choice_round_trip");
        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine

        let mut settings = Settings::new(path.clone());
        let id = settings.register_setting(choice("scale", 0));
        if let Ok(Setting::Choice { selected, .. }) = settings.get_setting_mut(id) {
            *selected = 2;
        }
        settings.remove_setting(id).unwrap();
        settings.save_config().unwrap();

        let mut reloaded = Settings::new(path.clone());
        let id = reloaded.register_setting(choice("scale", 0));
        match reloaded.get_setting(id).unwrap() {
            Setting::Choice { selected, .. } => assert_eq!(*selected, 2),
            _ => panic!("expected a choice"),
        }

        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine
    }

    /// A slider stores either one of the named choices or a numeric value, packed into a
    /// single integer in the config. Both directions have to survive the trip.
    #[test]
    fn test_slider_choice_survives_a_save_and_reload() {
        let path = temp_config("slider_choice");
        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine

        let mut settings = Settings::new(path.clone());
        let id = settings.register_setting(slider("fps_limit", SliderSelection::Choice(0)));
        if let Ok(Setting::Slider { selected, .. }) = settings.get_setting_mut(id) {
            *selected = SliderSelection::Choice(1);
        }
        settings.remove_setting(id).unwrap();
        settings.save_config().unwrap();

        let mut reloaded = Settings::new(path.clone());
        let id = reloaded.register_setting(slider("fps_limit", SliderSelection::Choice(0)));
        match reloaded.get_setting(id).unwrap() {
            Setting::Slider { selected, .. } => match selected {
                SliderSelection::Choice(value) => assert_eq!(*value, 1),
                SliderSelection::Slider(value) => panic!("expected a choice, got slider {value}"),
            },
            _ => panic!("expected a slider"),
        }

        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine
    }

    #[test]
    fn test_slider_value_survives_a_save_and_reload() {
        let path = temp_config("slider_value");
        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine

        let mut settings = Settings::new(path.clone());
        let id = settings.register_setting(slider("fps_limit", SliderSelection::Choice(0)));
        if let Ok(Setting::Slider { selected, .. }) = settings.get_setting_mut(id) {
            *selected = SliderSelection::Slider(144);
        }
        settings.remove_setting(id).unwrap();
        settings.save_config().unwrap();

        let mut reloaded = Settings::new(path.clone());
        let id = reloaded.register_setting(slider("fps_limit", SliderSelection::Choice(0)));
        match reloaded.get_setting(id).unwrap() {
            Setting::Slider { selected, .. } => match selected {
                SliderSelection::Slider(value) => assert_eq!(*value, 144),
                SliderSelection::Choice(value) => panic!("expected a slider value, got choice {value}"),
            },
            _ => panic!("expected a slider"),
        }

        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine
    }

    /// A setting that was never removed is never written, which is what the warning in
    /// `save_config` is about.
    #[test]
    fn test_unremoved_settings_are_not_saved() {
        let path = temp_config("unremoved");
        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine

        let mut settings = Settings::new(path.clone());
        let id = settings.register_setting(toggle("never_removed", false));
        if let Ok(Setting::Toggle { toggled, .. }) = settings.get_setting_mut(id) {
            *toggled = true;
        }
        settings.save_config().unwrap();

        let mut reloaded = Settings::new(path.clone());
        let id = reloaded.register_setting(toggle("never_removed", false));
        match reloaded.get_setting(id).unwrap() {
            Setting::Toggle { toggled, .. } => assert!(!*toggled, "an unremoved setting should not have been persisted"),
            _ => panic!("expected a toggle"),
        }

        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine
    }

    #[test]
    fn test_corrupt_config_falls_back_to_defaults() {
        let path = temp_config("corrupt");
        std::fs::write(&path, "this is not json").unwrap();

        let mut settings = Settings::new(path.clone());
        let id = settings.register_setting(toggle("blur", true));
        match settings.get_setting(id).unwrap() {
            Setting::Toggle { toggled, .. } => assert!(*toggled, "a corrupt config should not override the default"),
            _ => panic!("expected a toggle"),
        }

        drop(std::fs::remove_file(&path)); // may not exist yet, that is fine
    }
}
