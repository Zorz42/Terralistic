//
//  network.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/02/2021.
//

#include "otherPlayers.hpp"
#include "networkingModule.hpp"

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
    unsigned short id = packet.getUShort();
    for(auto & player : players::players)
        if(player.id == id) {
            player.flipped = packet.getChar();
            player.y = packet.getInt();
            player.x = packet.getInt();
            break;
        }
}

networking::registerListener player_join_listener(playerJoinListener, packets::PLAYER_JOIN), player_quit_listener(playerQuitListener, packets::PLAYER_QUIT), player_movement_listener(playerMovementListener, packets::PLAYER_MOVEMENT);
