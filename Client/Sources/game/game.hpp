//
//  game.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef game_hpp
#define game_hpp

#ifdef __WIN32__
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
    networking::networkingManager networking_manager;
    
    bool multiplayer;
    scene(const std::string& world_name, bool multiplayer) : world_name(world_name), multiplayer(multiplayer) {}
    void init();
    void stop();
    void render();
    void update();
};

}

#endif /* game_hpp */
