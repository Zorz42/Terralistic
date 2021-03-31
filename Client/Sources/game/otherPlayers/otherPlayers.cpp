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

void players::module::init() {
    other_players.clear();
    listening_to = {packets::PLAYER_JOIN, packets::PLAYER_QUIT, packets::PLAYER_MOVEMENT};
}

void players::module::render() {
    // iterate through every player and render them
    for(player& i : other_players) {
        bool prev = playerHandler::player.flipped;
        playerHandler::player.flipped = i.flipped;
        playerHandler::player.x = i.x - playerHandler::view_x;
        playerHandler::player.y = i.y - playerHandler::view_y;
        gfx::render(playerHandler::player);
        playerHandler::player.flipped = prev;
    }
}

players::module::player* players::module::getPlayerById(unsigned short id) {
    for(auto& player : other_players)
        if(player.id == id)
            return &player;
    ASSERT(false, "Could not get player by id");
    return nullptr;
}

void players::module::onPacket(packets::packet packet) {
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
