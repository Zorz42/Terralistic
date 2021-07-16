//
//  otherPlayers.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#ifndef otherPlayers_hpp
#define otherPlayers_hpp

#include "clientNetworking.hpp"
#include "clientMap.hpp"

struct clientPlayer {
    unsigned short id{0};
    int x{0}, y{0};
    bool flipped = false;
    std::string name;
    gfx::Image name_text;
};

class clientPlayers : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    std::vector<clientPlayer*> other_players;
    clientPlayer* getPlayerById(unsigned short id);
    map* world_map;
public:
    clientPlayers(networkingManager* manager, map* world_map) : world_map(world_map) {}
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
};

#endif /* otherPlayers_hpp */
