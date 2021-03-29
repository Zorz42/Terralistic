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

namespace worldCreator {

struct scene : public gfx::scene {
    const std::vector<std::string>& worlds;
    scene(const std::vector<std::string>& worlds) : worlds(worlds) {}
    bool running = true, can_create;
    void init();
    void onKeyDown(gfx::key key);
    void render();
};

}

#endif /* worldCreator_hpp */
