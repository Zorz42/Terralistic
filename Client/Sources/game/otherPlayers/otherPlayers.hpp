//
//  otherPlayers.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#ifndef otherPlayers_hpp
#define otherPlayers_hpp

#include <Graphics/graphics.hpp>
#include "game.hpp"

namespace players {

struct module : public gfx::sceneModule<game::scene> {
    using gfx::sceneModule<game::scene>::sceneModule;
    void init();
    void render();
};

}

#endif /* otherPlayers_hpp */
