#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

class MainMenu : public gfx::Scene {
    gfx::Button singleplayer_button, multiplayer_button, settings_button, mods_button, exit_button;
    gfx::Sprite title, version;
    MenuBack* menu_back;
    Settings* settings;
    gfx::Sprite debug_title;
public:
    MainMenu(MenuBack* menu_back, Settings* settings) : menu_back(menu_back), settings(settings) {}
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
};
