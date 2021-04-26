//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "otherPlayers.hpp"
#include "playerRenderer.hpp"
#include "dev.hpp"

// module for handling other players in online game

void players::init() {
    other_players.clear();
    listening_to = {packets::PLAYER_JOIN, packets::PLAYER_QUIT, packets::PLAYER_MOVEMENT};
}

void players::render() {
    // iterate through every player and render them
    for(player& i : other_players)
        playerRenderer::render(i.x, i.y, map->view_x, map->view_y, i.flipped);
}

players::player* players::getPlayerById(unsigned short id) {
    for(auto& player : other_players)
        if(player.id == id)
            return &player;
    ASSERT(false, "Could not get player by id");
    return nullptr;
}

void players::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::PLAYER_JOIN: {
            player player;
            player.id = packet.getUShort();
            player.y = packet.getInt();
            player.x = packet.getInt();
            other_players.push_back(player);
            break;
        }
        case packets::PLAYER_QUIT: {
            unsigned short id = packet.getUShort();
            for(auto i = other_players.begin(); i != other_players.end(); i++)
                if(i->id == id) {
                    other_players.erase(i);
                    break;
                }
            break;
        }
        case packets::PLAYER_MOVEMENT: {
            player* player = getPlayerById(packet.getUShort());
            player->flipped = packet.getChar();
            player->y = packet.getInt();
            player->x = packet.getInt();
            break;
        }
        default:;
    }
}
