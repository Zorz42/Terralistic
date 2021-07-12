//
//  otherPlayers.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#ifndef otherPlayers_hpp
#define otherPlayers_hpp

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#else
#include "graphics.hpp"
#endif

#include "clientNetworking.hpp"
#include "clientMap.hpp"

struct clientPlayer {
    unsigned short id{0};
    int x{0}, y{0};
    bool flipped = false;
    std::string name;
    gfx::image name_text;
};

class clientPlayers : public gfx::sceneModule, packetListener {
    std::vector<clientPlayer*> other_players;
    clientPlayer* getPlayerById(unsigned short id);
    map* world_map;
public:
    clientPlayers(networkingManager* manager, map* world_map) : packetListener(manager), world_map(world_map) { listening_to = {PacketType::PLAYER_JOIN, PacketType::PLAYER_QUIT, PacketType::PLAYER_MOVEMENT}; }
    void render() override;
    void onPacket(Packet &packet) override;
};

#endif /* otherPlayers_hpp */
