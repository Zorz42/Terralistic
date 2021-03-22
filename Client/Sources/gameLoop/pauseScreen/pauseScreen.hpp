//
//  pauseScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#include <Graphics/graphics.hpp>

namespace pauseScreen {

struct module : public gfx::sceneModule {
    void render();
    void onKeyDown(gfx::key key);
};

}

#endif /* pauseScreen_hpp */
