#include "settings.hpp"
#include <cassert>

std::string ChoiceSetting::exportToStr() {
    return std::to_string(getSelectedChoice());
}

void ChoiceSetting::loadFromStr(std::string value) {
    setSelectedChoice(std::stoi(value));
}

void Settings::addSetting(Setting* setting) {
    settings.push_back(setting);
    if(config_file.keyExists(setting->indent))
        setting->loadFromStr(config_file.getStr(setting->indent));
}

void Settings::removeSetting(Setting* setting) {
    settings.erase(std::find(settings.begin(), settings.end(), setting));
    config_file.setStr(setting->indent, setting->exportToStr());
}

int ChoiceSetting::getSelectedChoice() {
    return selected_choice;
}

void ChoiceSetting::setSelectedChoice(int choice) {
    selected_choice = choice;
    
    SettingChangeEvent event;
    setting_change_event.call(event);
}

Settings::~Settings() {
    assert(settings.empty());
}

const std::vector<Setting*>& Settings::getSettings() {
    return settings;
}
