#include <filesystem>
#include <iostream>
#include "mainMenu.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"
#include "serverPlayers.hpp"
#include "settings.hpp"
#include "updater.hpp"
#include "versions.hpp"

void processScaleChoice(int scale) {
    switch(scale) {
        case 0:
            gfx::setGlobalScale(0.5);
            break;
        case 1:
            gfx::setGlobalScale(1);
            break;
        case 2:
            gfx::setGlobalScale(2);
            break;
    }
}

class ScaleChangeListener : public EventListener<SettingChangeEvent> {
    ChoiceSetting* scale_setting;
    void onEvent(SettingChangeEvent& event) override {
        processScaleChoice(scale_setting->getSelectedChoice());
    }
public:
    ScaleChangeListener(ChoiceSetting* scale_setting) : scale_setting(scale_setting) {}
};


int main(int argc, char **argv) {
    srand((unsigned int)time(0));
    
    if(argc == 2 && (std::string)argv[1] == "version") {
        std::cout << CURR_VERSION_STR << std::endl;
        return 0;
    }
    
    gfx::init(getResourcePath(argv[0]), 1130, 700);
    gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    gfx::loadFont("pixel_font.ttf", 8);
    
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/");
    
    Settings settings;
    ChoiceSetting scale_setting("Scale", {"Small", "Normal", "Large"}, 1);
    settings.addSetting(&scale_setting);
    
    processScaleChoice(scale_setting.getSelectedChoice());
    
    ScaleChangeListener scale_change_listener(&scale_setting);
    scale_setting.setting_change_event.addListener(&scale_change_listener);
    
    initProperties();
    
    MenuBack menu_back;
    menu_back.init();
    
#ifndef DEVELOPER_MODE
    UpdateChecker update_checker(&menu_back, argv[0]);
    update_checker.run();
    if(update_checker.hasUpdated()) {
        system(((std::string)"\"" + argv[0] + "\"&").c_str());
        gfx::quit();
        return 0;
    }
#endif
    MainMenu(&menu_back).run();
    
    settings.removeSetting(&scale_setting);
    scale_setting.setting_change_event.removeListener(&scale_change_listener);
    
    gfx::quit();

    return 0;
}
