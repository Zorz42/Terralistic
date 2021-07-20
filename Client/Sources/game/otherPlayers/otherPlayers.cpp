//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "otherPlayers.hpp"
#include "playerRenderer.hpp"
#include <cassert>

// module for handling other players in online game

void clientPlayers::render() {
    // iterate through every player and render them
    for(clientPlayer* i : other_players) {
        int intensity = 0;
        for(int x = i->x / BLOCK_WIDTH; x < i->x / BLOCK_WIDTH + 2; x++)
            for(int y = i->y / BLOCK_WIDTH; y < i->y / BLOCK_WIDTH + 3; y++)
                intensity += world_map->getBlock(x, y).getLightLevel();
        
        intensity /= 3 * 2;
        
        playerRenderer::render(i->x, i->y, world_map->view_x, world_map->view_y, i->flipped, intensity * 255 / MAX_LIGHT, i->name_text);
    }
}

clientPlayer* clientPlayers::getPlayerById(unsigned short id) {
    for(clientPlayer* player : other_players)
        if(player->id == id)
            return player;
    assert(false);
    return nullptr;
}

void clientPlayers::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::PLAYER_JOIN: {
            auto* new_player = new clientPlayer();
            event.packet >> new_player->x >> new_player->y >> new_player->id >> new_player->name;
            new_player->name_text.renderText(new_player->name, {0, 0, 0});
            other_players.push_back(new_player);
            break;
        }
        case PacketType::PLAYER_QUIT: {
            unsigned id;
            event.packet >> id;
            for(auto i = other_players.begin(); i != other_players.end(); i++)
                if((*i)->id == id) {
                    other_players.erase(i);
                    break;
                }
            break;
        }
        case PacketType::PLAYER_MOVEMENT: {
            int x, y;
            unsigned short id;
            bool flipped;
            event.packet >> x >> y >> flipped >> id;
            
            clientPlayer* player = getPlayerById(id);
            player->flipped = flipped;
            player->x = x;
            player->y = y;
            break;
        }
        default:;
    }
}
