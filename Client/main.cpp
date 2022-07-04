#include <filesystem>
#include <fstream>
#include "mainMenu.hpp"
#include "resourcePath.hpp"
#include "versions.hpp"
#include "updater.hpp"
#include "networking.hpp"

class ScaleChangeListener : public EventListener<SettingChangeEvent> {
    ChoiceSetting* scale_setting;
    void onEvent(SettingChangeEvent& event) override {
        switch(scale_setting->getSelectedChoice()) {
            case 0:
                gfx::setGlobalScale(0);
                break;
            case 1:
                gfx::setGlobalScale(0.5);
                break;
            case 2:
                gfx::setGlobalScale(1);
                break;
            case 3:
                gfx::setGlobalScale(2);
                break;
        }
    }
public:
    explicit ScaleChangeListener(ChoiceSetting* scale_setting) : scale_setting(scale_setting) {}
};

class FpsChangeListener : public EventListener<SettingChangeEvent> {
    ChoiceSetting* fps_setting;
    void onEvent(SettingChangeEvent& event) override {
        switch(fps_setting->getSelectedChoice()) {
            case 0:
                gfx::enableVsync(true);
                gfx::fps_limit = 0;
                break;
            case 1:
                gfx::enableVsync(false);
                gfx::fps_limit = 5;
                break;
            case 2:
                gfx::enableVsync(false);
                gfx::fps_limit = 60;
                break;
            case 3:
                gfx::enableVsync(false);
                gfx::fps_limit = 0;
                break;
        }
    }
public:
    explicit FpsChangeListener(ChoiceSetting* fps_setting) : fps_setting(fps_setting) {}
};

class BlurChangeListener : public EventListener<SettingChangeEvent> {
    BooleanSetting* blur_setting;
    void onEvent(SettingChangeEvent& event) override {
        gfx::blur_enabled = blur_setting->getValue();
    }
public:
    explicit BlurChangeListener(BooleanSetting* blur_setting) : blur_setting(blur_setting) {}
};

int main(int argc, char **argv) {
    srand((int)time(nullptr));

    if(argc == 2 && (std::string)argv[1] == "version") {
        std::cout << CURR_VERSION_STR << std::endl;
        return 0;
    }

    resource_path = getResourcePath(argv[0]);
    gfx::init(1130, 700, "Terralistic");
    gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    std::ifstream font_file(resource_path + "font.opa");
    if(!font_file.is_open())
        throw Exception("Could not open font file");
    std::vector<unsigned char> data = std::vector<unsigned char>((std::istreambuf_iterator<char>(font_file)), std::istreambuf_iterator<char>());
    data.erase(data.begin(), data.begin() + 8);
    
    gfx::loadFont(&data[0]);
    
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/");
    
    Settings settings;
    gfx::addAGlobalUpdateFunction(&settings);
    
    ChoiceSetting scale_setting("Scale", {"Auto", "Small", "Normal", "Large"}, 0);
    ScaleChangeListener scale_change_listener(&scale_setting);
    scale_setting.setting_change_event.addListener(&scale_change_listener);
    settings.addSetting(&scale_setting);
    
    ChoiceSetting fps_setting("FPS limit", {"Vsync", "5", "60", "Unlimited"}, 2);
    FpsChangeListener fps_change_listener(&fps_setting);
    fps_setting.setting_change_event.addListener(&fps_change_listener);
    settings.addSetting(&fps_setting);
    
    BooleanSetting blur_setting("Blur Effect", true);
    BlurChangeListener blur_change_listener(&blur_setting);
    blur_setting.setting_change_event.addListener(&blur_change_listener);
    settings.addSetting(&blur_setting);

    MenuBack* menu_back = new MenuBack;
    menu_back->init();
    
#ifndef DEVELOPER_MODE
    //UpdateChecker update_checker(&menu_back, argv[0]);
    //update_checker.run();
#endif
    MainMenu* main_menu = new MainMenu(menu_back, &settings);
    main_menu->run();

    delete main_menu;
    delete menu_back;

    settings.removeSetting(&scale_setting);
    scale_setting.setting_change_event.removeListener(&scale_change_listener);

    settings.removeSetting(&fps_setting);
    fps_setting.setting_change_event.removeListener(&fps_change_listener);

    settings.removeSetting(&blur_setting);
    blur_setting.setting_change_event.removeListener(&blur_change_listener);

    gfx::quit();

    return 0;
}
