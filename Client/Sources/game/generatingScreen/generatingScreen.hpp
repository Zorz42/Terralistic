//
//  generatingScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#ifndef generatingScreen_hpp
#define generatingScreen_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include <thread>

namespace terrainGenerator {

struct scene : public gfx::scene {
    unsigned int seed;
    std::thread thread;
    scene(unsigned int seed) : seed(seed) {}
    void init();
    void update();
    void render();
    void stop();
};

}

#endif /* generatingScreen_hpp */
