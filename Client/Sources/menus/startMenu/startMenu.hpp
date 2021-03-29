//
//  startMenu.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef startMenu_hpp
#define startMenu_hpp

#undef main

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

namespace startMenu {

struct scene : public gfx::scene {
    void onKeyDown(gfx::key key);
    void render();
};

}
#endif /* startMenu_hpp */
