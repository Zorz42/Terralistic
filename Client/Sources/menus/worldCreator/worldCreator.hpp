#ifndef worldCreator_hpp
#define worldCreator_hpp

#include "graphics.hpp"

#include <string>
#include <vector>

struct WorldCreator : gfx::Scene {
    explicit WorldCreator(std::vector<std::string> worlds) : worlds(std::move(worlds)) {}
    bool can_create = true;
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;

private:
    std::vector<std::string> worlds;
    gfx::Button back_button, create_button;
    gfx::Sprite new_world_title;
    gfx::TextInput world_name;
};

#endif
