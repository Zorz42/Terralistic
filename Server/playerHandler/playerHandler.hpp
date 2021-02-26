//
//  playerHandler.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 30/01/2021.
//

#ifndef playerHandler_hpp
#define playerHandler_hpp

#include "networkingModule.hpp"

namespace playerHandler {

class player {
public:
    player(unsigned short id) : id(id) {}
    networking::connection* conn;
    unsigned short id;
    bool flipped = false;
    int x = 0, y = 0;
    inventory::inventory inventory;
    unsigned short breaking_x, breaking_y;
    bool breaking = false;
};

inline std::vector<player> players;
player* getPlayerByConnection(networking::connection* conn);

void lookForItems();

}

#endif /* playerHandler_hpp */
