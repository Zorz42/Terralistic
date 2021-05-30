//
//  otherPlayers.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#ifndef otherPlayers_hpp
#define otherPlayers_hpp

#ifdef _WIN32
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif


#endif

#include "clientNetworking.hpp"
#include "clientMap.hpp"

class players : public gfx::sceneModule, packetListener {
    struct player {
        unsigned short id{0};
        int x{0}, y{0};
        bool flipped = false;
        std::string name;
        gfx::image name_text;
    };

    std::vector<player*> other_players;
    player* getPlayerById(unsigned short id);
    map* world_map;
public:
    players(networkingManager* manager, map* world_map) : packetListener(manager), world_map(world_map) { listening_to = {packets::PLAYER_JOIN, packets::PLAYER_QUIT, packets::PLAYER_MOVEMENT}; }
    void render();
    void onPacket(packets::packet packet);
};

#endif /* otherPlayers_hpp */
