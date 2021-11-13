#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

class WorldCreator : public gfx::Scene {
    explicit WorldCreator(BackgroundRect* menu_back) : menu_back(menu_back) {}
    std::vector<std::string> worlds;
    gfx::Button back_button, create_button;
    gfx::Sprite new_world_title;
    gfx::TextInput world_name;
    BackgroundRect* menu_back;
    Settings* settings;
    bool can_create = true;
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
public:
    explicit WorldCreator(std::vector<std::string> worlds, BackgroundRect* menu_back, Settings* settings) : worlds(std::move(worlds)), menu_back(menu_back), settings(settings) {}
};
