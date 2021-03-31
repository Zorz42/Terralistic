//
//  game.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef game_hpp
#define game_hpp

#ifdef __WIN32__
#include <utility>

#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "networkingModule.hpp"

namespace game {

int main(const std::string& world_name, bool multiplayer);
inline bool online;

struct scene : public gfx::scene {
    const std::string world_name;
    gfx::sprite fps_text;
    networking::networkingManager networking_manager;
    
    bool multiplayer;
    scene(std::string  world_name, bool multiplayer) : world_name(std::move(world_name)), multiplayer(multiplayer) {}
    void init() override;
    void stop() override;
    void render() override;
    void update() override;
};

}

#endif /* game_hpp */
