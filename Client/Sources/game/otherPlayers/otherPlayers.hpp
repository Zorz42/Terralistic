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
#include "networkingModule.hpp"

namespace players {

struct module : public gfx::sceneModule<game::scene>, networking::packetListener {
    using gfx::sceneModule<game::scene>::sceneModule;
    void init();
    void render();
    void onPacket(packets::packet packet);
};

}

#endif /* otherPlayers_hpp */
