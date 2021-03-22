//
//  pauseScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include <Graphics/graphics.hpp>
#include "game.hpp"

namespace pauseScreen {

struct module : public gfx::sceneModule<game::scene> {
    using gfx::sceneModule<game::scene>::sceneModule;
    void render();
    void onKeyDown(gfx::key key);
};

}

#endif /* pauseScreen_hpp */
