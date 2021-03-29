//
//  blockSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#ifndef blockSelector_hpp
#define blockSelector_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"

namespace blockSelector {

struct module : public gfx::sceneModule<game::scene> {
    using gfx::sceneModule<game::scene>::sceneModule;
    void render();
    void onKeyDown(gfx::key key);
    void onKeyUp(gfx::key key);
};

}

#endif /* blockSelector_hpp */
