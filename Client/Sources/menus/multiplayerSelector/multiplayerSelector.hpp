//
//  multiplayerSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef multiplayerSelector_hpp
#define multiplayerSelector_hpp

#include <Graphics/graphics.hpp>

namespace multiplayerSelector {

struct scene : gfx::scene {
    void init();
    void onKeyDown(gfx::key key);
    void render();
};

}

#endif /* multiplayerSelector_hpp */
