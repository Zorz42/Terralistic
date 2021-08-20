#ifndef worldCreator_hpp
#define worldCreator_hpp

#include <string>
#include <vector>
#include "graphics.hpp"
#include "menuBack.hpp"

class WorldCreator : public gfx::Scene {
    WorldCreator(MenuBack* menu_back) : menu_back(menu_back) {}
    std::vector<std::string> worlds;
    gfx::Button back_button, create_button;
    gfx::Sprite new_world_title;
    gfx::TextInput world_name;
    MenuBack* menu_back;
    bool can_create = true;
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
public:
    explicit WorldCreator(std::vector<std::string> worlds, MenuBack* menu_back) : worlds(std::move(worlds)), menu_back(menu_back) {}
};

#endif
