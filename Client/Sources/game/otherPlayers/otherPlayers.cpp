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

void clientPlayers::render() {
    // iterate through every player and render them
    for(clientPlayer* i : other_players)
        playerRenderer::render(i->x, i->y, world_map->view_x, world_map->view_y, i->flipped, i->name_text);
}

clientPlayer* clientPlayers::getPlayerById(unsigned short id) {
    for(clientPlayer* player : other_players)
        if(player->id == id)
            return player;
    ASSERT(false, "Could not get player by id")
    return nullptr;
}

void clientPlayers::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::PLAYER_JOIN: {
            auto* new_player = new clientPlayer();
            event.packet << new_player->x << new_player->y << new_player->id << new_player->name;
            new_player->name_text.renderText(new_player->name, {0, 0, 0});
            other_players.push_back(new_player);
            break;
        }
        case PacketType::PLAYER_QUIT: {
            auto id = event.packet.get<unsigned short>();
            for(auto i = other_players.begin(); i != other_players.end(); i++)
                if((*i)->id == id) {
                    other_players.erase(i);
                    break;
                }
            break;
        }
        case PacketType::PLAYER_MOVEMENT: {
            clientPlayer* player = getPlayerById(event.packet.get<unsigned short>());
            player->flipped = event.packet.get<char>();
            player->y = event.packet.get<int>();
            player->x = event.packet.get<int>();
            break;
        }
        default:;
    }
}
