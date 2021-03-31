//
//  multiplayerSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef multiplayerSelector_hpp
#define multiplayerSelector_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

namespace multiplayerSelector {

struct scene : gfx::scene {
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
};

}

#endif /* multiplayerSelector_hpp */
