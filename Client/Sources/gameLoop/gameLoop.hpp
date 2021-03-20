//
//  gameLoop.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef gameLoop_hpp
#define gameLoop_hpp

#undef main

#include <Graphics/graphics.hpp>

namespace gameLoop {

int main(const std::string& world_name, bool multiplayer);
inline bool running, online;

struct scene : public gfx::scene {
    const std::string& world_name;
    bool multiplayer;
    scene(const std::string& world_name, bool multiplayer) : world_name(world_name), multiplayer(multiplayer) {}
    void init();
    void stop();
    void onKeyUp(gfx::key key);
    void onKeyDown(gfx::key key);
    void render();
    void update();
};

}

#endif /* gameLoop_hpp */
