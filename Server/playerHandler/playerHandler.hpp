//
//  playerHandler.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 30/01/2021.
//

#ifndef playerHandler_hpp
#define playerHandler_hpp

#include "networkingModule.hpp"
#include "inventory.hpp"
#include "map.hpp"

class player {
public:
    player(unsigned short id) : id(id) {}
    connection* conn;
    unsigned short id;
    bool flipped = false;
    int x = 0, y = 0;
    unsigned short sight_width = 0, sight_height = 0;
    inventory inventory;
    unsigned short breaking_x, breaking_y;
    bool breaking = false;
};

class playerHandler : packetListener {
public:
    map* world_map;
    networkingManager* manager;
    void onPacket(packets::packet& packet, connection& conn);
    
    playerHandler(networkingManager* manager, map* world_map) : manager(manager), packetListener(manager), world_map(world_map) { listening_to = {packets::PLAYER_MOVEMENT, packets::PLAYER_JOIN, packets::DISCONNECT, packets::INVENTORY_SWAP, packets::HOTBAR_SELECTION}; }
};

inline std::vector<player> players;
player* getPlayerByConnection(connection* conn);
void lookForItems(map& world_map);

#endif /* playerHandler_hpp */
