//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "otherPlayers.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "dev.hpp"

// module for handling other players in online game

void players::prepare() {
    players.clear();
}

void players::render() {
    // iterate through every player and render them
    for(player& i : players) {
        bool prev = playerHandler::player.flipped;
        playerHandler::player.flipped = i.flipped;
        playerHandler::player.setX(short(i.x - playerHandler::view_x));
        playerHandler::player.setY(short(i.y - playerHandler::view_y));
        playerHandler::player.render();
        playerHandler::player.flipped = prev;
    }
}

players::player* getPlayerById(unsigned short id) {
    for(auto & player : players::players)
        if(player.id == id)
            return &player;
    ASSERT(false, "Could not get player by id");
    return nullptr;
}

// handle all joins, quits and movements of players
void playerJoinListener(packets::packet& packet) {
    players::player player;
    player.id = packet.getUShort();
    player.y = packet.getInt();
    player.x = packet.getInt();
    players::players.push_back(player);
}
void playerQuitListener(packets::packet& packet) {
    unsigned short id = packet.getUShort();
    for(auto i = players::players.begin(); i != players::players.end(); i++)
        if(i->id == id) {
            players::players.erase(i);
            break;
        }
}
void playerMovementListener(packets::packet& packet) {
    players::player* player = getPlayerById(packet.getUShort());
    player->flipped = packet.getChar();
    player->y = packet.getInt();
    player->x = packet.getInt();
}

networking::registerListener player_join_listener(playerJoinListener, packets::PLAYER_JOIN), player_quit_listener(playerQuitListener, packets::PLAYER_QUIT), player_movement_listener(playerMovementListener, packets::PLAYER_MOVEMENT);
