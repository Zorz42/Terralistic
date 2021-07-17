//
//  worldCreator.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldCreator_hpp
#define worldCreator_hpp

#include "graphics.hpp"

#include <string>
#include <utility>
#include <vector>

struct worldCreator : gfx::Scene {
    explicit worldCreator(std::vector<std::string> worlds) : worlds(std::move(worlds)) {}
    bool running = true, can_create = true;
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;

private:
    std::vector<std::string> worlds;
    gfx::Button back_button, create_button;
    gfx::Sprite new_world_title;
    gfx::TextInput world_name;
};

#endif /* worldCreator_hpp */
