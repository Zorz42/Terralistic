//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "core.hpp"

#include "otherPlayers.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"

// module for handling other players in online game

struct player {
    unsigned short id{0};
    int x{0}, y{0};
    bool flipped = false;
};

static std::vector<player> other_players;

void players::prepare() {
    other_players.clear();
}

void players::render() {
    // iterate through every player and render them
    for(player& i : other_players) {
        //bool prev = playerHandler::player.flipped;
        //playerHandler::player.flipped = i.flipped;
        playerHandler::player.x = i.x - playerHandler::view_x;
        playerHandler::player.y = i.y - playerHandler::view_y;
        gfx::render(playerHandler::player);
        //playerHandler::player.flipped = prev;
    }
}

player* getPlayerById(unsigned short id) {
    for(auto & player : other_players)
        if(player.id == id)
            return &player;
    ASSERT(false, "Could not get player by id");
    return nullptr;
}

// handle all joins, quits and movements of players
PACKET_LISTENER(packets::PLAYER_JOIN)
    player player;
    player.id = packet.getUShort();
    player.y = packet.getInt();
    player.x = packet.getInt();
    other_players.push_back(player);
PACKET_LISTENER_END

PACKET_LISTENER(packets::PLAYER_QUIT)
    unsigned short id = packet.getUShort();
    for(auto i = other_players.begin(); i != other_players.end(); i++)
        if(i->id == id) {
            other_players.erase(i);
            break;
        }
PACKET_LISTENER_END
    
PACKET_LISTENER(packets::PLAYER_MOVEMENT)
    player* player = getPlayerById(packet.getUShort());
    player->flipped = packet.getChar();
    player->y = packet.getInt();
    player->x = packet.getInt();
PACKET_LISTENER_END
