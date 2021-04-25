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

#include "networkingModule.hpp"
#include "playerHandler.hpp"
#include "renderMap.hpp"

class players : public gfx::sceneModule, packetListener {
    struct player {
        unsigned short id{0};
        int x{0}, y{0};
        bool flipped = false;
    };

    std::vector<player> other_players;
    player* getPlayerById(unsigned short id);
    mainPlayer* main_player;
    renderMap* map;
public:
    players(networkingManager* manager, mainPlayer* player, renderMap* map) : packetListener(manager), main_player(player), map(map) {}
    void init();
    void render();
    void onPacket(packets::packet packet);
};

#endif /* otherPlayers_hpp */
