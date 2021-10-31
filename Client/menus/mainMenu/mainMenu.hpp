#pragma once

#include "graphics.hpp"
#include "menuBack.hpp"

class MainMenu : public gfx::Scene {
    gfx::Button singleplayer_button, multiplayer_button, settings_button, mods_button, exit_button;
    gfx::Sprite title, version;
    MenuBack* menu_back;
    gfx::Sprite debug_title;
public:
    MainMenu(MenuBack* menu_back) : menu_back(menu_back) {}
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
};
