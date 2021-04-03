//
//  worldCreator.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldCreator_hpp
#define worldCreator_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include <string>
#include <vector>

struct worldCreator : public gfx::scene {
    worldCreator(const std::vector<std::string>& worlds) : worlds(worlds) {}
    bool running = true, can_create = false;
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
    
private:
    std::vector<std::string> worlds;
    gfx::button back_button, create_button;
    gfx::sprite new_world_title, faded_create;
    gfx::textInput world_name;
};

#endif /* worldCreator_hpp */
