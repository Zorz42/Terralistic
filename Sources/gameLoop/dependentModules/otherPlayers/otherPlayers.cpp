//
//  otherPlayers.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#include "otherPlayers.hpp"
#include "playerHandler.hpp"

void players::prepare() {
    players.clear();
}

void players::render() {
    for(player& i : players) {
        bool prev = playerHandler::player.flipped;
        playerHandler::player.flipped = i.flipped;
        playerHandler::player.setX(short(i.x - playerHandler::view_x));
        playerHandler::player.setY(short(i.y - playerHandler::view_y));
        playerHandler::player.render();
        playerHandler::player.flipped = prev;
    }
}
