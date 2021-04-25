//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "otherPlayers.hpp"
#include "playerHandler.hpp"
#include "dev.hpp"

// module for handling other players in online game

void players::init() {
    other_players.clear();
    listening_to = {packets::PLAYER_JOIN, packets::PLAYER_QUIT, packets::PLAYER_MOVEMENT};
}

void players::render() {
    // iterate through every player and render them
    for(player& i : other_players) {
        bool prev = main_player->player.flipped;
        main_player->player.flipped = i.flipped;
        main_player->player.x = i.x - map->view_x;
        main_player->player.y = i.y - map->view_y;
        gfx::render(main_player->player);
        main_player->player.flipped = prev;
    }
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
