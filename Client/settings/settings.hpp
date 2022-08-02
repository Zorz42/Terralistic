#pragma once
#include "configManager.hpp"
#include "platform_folders.h"
#include "graphics.hpp"
#include "events.hpp"

enum class SettingType {CHOICE_SETTING, BOOLEAN_SETTING, SLIDER_SETTING};

class SettingChangeEvent {};

class Setting {
public:
    SettingType type;
    Setting(const std::string& ident) : ident(ident) {}
    const std::string ident;
    virtual std::string exportToStr() = 0;
    virtual void loadFromStr(std::string value) = 0;
    EventSender<SettingChangeEvent> setting_change_event;
};

class ChoiceSetting : public Setting {
    int selected_choice;
public:
    ChoiceSetting(const std::string& ident, const std::vector<std::string>& choices, int default_value) : Setting(ident), choices(choices), selected_choice(default_value) { type = SettingType::CHOICE_SETTING; };
    const std::vector<std::string> choices;
    int getSelectedChoice() const;
    void setSelectedChoice(int choice);
    
    std::string exportToStr() override;
    void loadFromStr(std::string value) override;
};

class BooleanSetting : public Setting {
    bool value;
public:
    BooleanSetting(const std::string& ident, bool default_value) : Setting(ident), value(default_value) { type = SettingType::BOOLEAN_SETTING; };
    bool getValue() const;
    void setValue(bool new_value);
    
    std::string exportToStr() override;
    void loadFromStr(std::string value) override;
};

class SliderSetting : public Setting {
    int selected_choice;
public:
    SliderSetting(const std::string& ident, int min, int max, int step, const std::vector<std::string>& choices, const std::string& slider_text, int default_value) : Setting(ident), min(min), max(max), step(step), choices(choices), slider_text(slider_text), selected_choice(default_value) { type = SettingType::SLIDER_SETTING; };
    const std::vector<std::string> choices;
    const std::string slider_text;
    const int min, max, step;
    int getSelectedChoice() const;
    void setSelectedChoice(int choice);
    int getSliderValue() const;
    
    std::string exportToStr() override;
    void loadFromStr(std::string value) override;
};

class Settings : public gfx::GlobalUpdateFunction, EventListener<SettingChangeEvent> {
    std::vector<Setting*> settings;
    ConfigFile config_file;
    void update() override;
    void onEvent(SettingChangeEvent& event) override;
    bool is_loading = false;
    gfx::Timer timer;
public:
    Settings() : config_file(sago::getDataHome() + "/Terralistic/settings.txt") {}
    void addSetting(Setting* setting);
    void removeSetting(Setting* setting);
    const std::vector<Setting*>& getSettings();
    void reloadSettings();
    ~Settings();
};
