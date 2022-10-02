#include <filesystem>
#include <fstream>
#include "mainMenu.hpp"
#include "resourcePath.hpp"
#include "versions.hpp"
#include "updater.hpp"
#include "networking.hpp"
#include "testing.hpp"
#include "readOpa.hpp"

class ScaleChangeListener : public EventListener<SettingChangeEvent> {
    ChoiceSetting* scale_setting;
    void onEvent(SettingChangeEvent& event) override {
        switch(scale_setting->getSelectedChoice()) {
            case 0:
                gfx::setGlobalScale(SYSTEM_SCALE);
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
    SliderSetting* fps_setting;
    void onEvent(SettingChangeEvent& event) override {
        switch(fps_setting->getSelectedChoice()) {
            case 0:
                gfx::enableVsync(true);
                gfx::fps_limit = 0;
                break;
            case 1:
                gfx::enableVsync(false);
                gfx::fps_limit = 0;
                break;
            default:
                gfx::enableVsync(false);
                gfx::fps_limit = fps_setting->getSliderValue();
        }
    }
public:
    explicit FpsChangeListener(SliderSetting* fps_setting) : fps_setting(fps_setting) {}
};

class BlurChangeListener : public EventListener<SettingChangeEvent> {
    BooleanSetting* blur_setting;
    void onEvent(SettingChangeEvent& event) override {
        gfx::blur_enabled = blur_setting->getValue();
    }
public:
    explicit BlurChangeListener(BooleanSetting* blur_setting) : blur_setting(blur_setting) {}
};

class AntiStutterListener : public EventListener<SettingChangeEvent> {
    BooleanSetting* stutter_setting;
    void onEvent(SettingChangeEvent& event) override {
        gfx::anti_stutter = stutter_setting->getValue();
    }
public:
    explicit AntiStutterListener(BooleanSetting* stutter_setting) : stutter_setting(stutter_setting) {}
};

int main(int argc, const char **argv) {
#ifdef __APPLE__
    pthread_setname_np("Main");
#endif

#ifdef __linux__
    pthread_setname_np(pthread_self(), "Main");
#endif
    
    srand((int)time(nullptr));

    if(argc > 1 && (std::string)argv[1] == "version") {
        std::cout << CURR_VERSION_STR << std::endl;
        return 0;
    }

    if(argc > 1 && (std::string)argv[1] == "test")
        return performTests() ? 0 : 1;

    resource_path = getResourcePath(argv[0]);
    gfx::init(1130, 700, "Terralistic");
    gfx::setMinimumWindowSize(gfx::getWindowWidth(), gfx::getWindowHeight());
    
    gfx::loadFont(readOpa(resource_path + "font.opa"));
    
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/");
    
    Settings settings;
    gfx::addAGlobalUpdateFunction(&settings);
    
    ChoiceSetting scale_setting("Scale", {"Auto", "Small", "Normal", "Large"}, 0);
    ScaleChangeListener scale_change_listener(&scale_setting);
    scale_setting.setting_change_event.addListener(&scale_change_listener);
    settings.addSetting(&scale_setting);
    
    SliderSetting fps_setting("FPS limit", 10, 300, 10, {"Vsync", "Unlimited"}, "FPS", "Custom", 0);
    FpsChangeListener fps_change_listener(&fps_setting);
    fps_setting.setting_change_event.addListener(&fps_change_listener);
    settings.addSetting(&fps_setting);
    
    BooleanSetting blur_setting("Blur Effect", true);
    BlurChangeListener blur_change_listener(&blur_setting);
    blur_setting.setting_change_event.addListener(&blur_change_listener);
    settings.addSetting(&blur_setting);

    BooleanSetting stutter_setting("Anti stutter", true);
    AntiStutterListener stutter_listener(&stutter_setting);
    stutter_setting.setting_change_event.addListener(&stutter_listener);
    settings.addSetting(&stutter_setting);

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
    
    settings.removeSetting(&stutter_setting);
    stutter_setting.setting_change_event.removeListener(&stutter_listener);

    gfx::quit();

    return 0;
}

#ifdef _WIN32
#include <Windows.h>
int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR pCmdLine, int nCmdShow) {
    const char** argv = new const char*[1];
    argv[0] = (char*)hInstance;
    return main(1, argv);
}
#endif