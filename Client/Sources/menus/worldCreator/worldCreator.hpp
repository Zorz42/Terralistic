//
//  worldCreator.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldCreator_hpp
#define worldCreator_hpp

#include <string>
#include <Graphics/graphics.hpp>

namespace worldCreator {

void loop(std::vector<std::string> worlds);

struct scene : public gfx::scene {
    bool running = true, can_create;
    void init();
    void onKeyDown(gfx::key key);
    void render();
};

}

#endif /* worldCreator_hpp */
