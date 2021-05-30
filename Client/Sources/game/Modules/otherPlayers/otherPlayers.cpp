//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "otherPlayers.hpp"
#include "playerRenderer.hpp"
#include "assert.hpp"

// module for handling other players in online game

void players::render() {
    // iterate through every player and render them
    for(player* i : other_players)
        playerRenderer::render(i->x, i->y, world_map->view_x, world_map->view_y, i->flipped, i->name_text);
}

players::player* players::getPlayerById(unsigned short id) {
    for(player* player : other_players)
        if(player->id == id)
            return player;
    ASSERT(false, "Could not get player by id");
    return nullptr;
}

void players::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::PLAYER_JOIN: {
            player* new_player = new player();
            new_player->name = packet.getString();
            new_player->id = packet.getUShort();
            new_player->y = packet.getInt();
            new_player->x = packet.getInt();
            new_player->name_text.setTexture(gfx::renderText(new_player->name, {0, 0, 0}));
            other_players.push_back(new_player);
            break;
        }
        case packets::PLAYER_QUIT: {
            unsigned short id = packet.getUShort();
            for(auto i = other_players.begin(); i != other_players.end(); i++)
                if((*i)->id == id) {
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
