//
//  otherPlayers.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#ifndef otherPlayers_hpp
#define otherPlayers_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"
#include "networkingModule.hpp"

namespace players {

struct module : public gfx::sceneModule<game::scene>, networking::packetListener {
    using gfx::sceneModule<game::scene>::sceneModule;
    void init();
    void render();
    void onPacket(packets::packet packet);
private:
    struct player {
        unsigned short id{0};
        int x{0}, y{0};
        bool flipped = false;
    };

    std::vector<player> other_players;
    player* getPlayerById(unsigned short id);
};

}

#endif /* otherPlayers_hpp */
