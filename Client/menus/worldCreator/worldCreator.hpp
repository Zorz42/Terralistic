#pragma once
#include "menuBack.hpp"
#include "settings.hpp"

class WorldCreator : public gfx::Scene {
    std::vector<std::string> worlds;
    gfx::Button back_button, create_button;
    gfx::Sprite new_world_title;
    gfx::Sprite new_world_name;
    gfx::Sprite new_world_seed;
    gfx::TextInput world_name;
    gfx::TextInput world_seed;
    BackgroundRect* menu_back;
    Settings* settings;
    bool can_create = true;
    void init() override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
public:
    WorldCreator(std::vector<std::string> worlds, BackgroundRect* menu_back, Settings* settings) : gfx::Scene("WorldCreator"), worlds(std::move(worlds)), menu_back(menu_back), settings(settings) {}
};
