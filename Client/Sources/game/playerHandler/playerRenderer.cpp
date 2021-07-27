//
//  playerRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/04/2021.
//

#include "playerHandler.hpp"
#include "graphics.hpp"

void PlayerHandler::initRenderer() {
    player_image.loadFromFile("resourcePack/misc/player.png");
}

#define HEADER_PADDING 4

void PlayerHandler::render(ClientPlayer& player_to_draw) {
    player_image.flipped = player_to_draw.flipped;
    player_image.render(2, gfx::getWindowWidth() / 2 - getPlayerWidth() / 2 + player_to_draw.x - world_map->view_x, gfx::getWindowHeight() / 2 - getPlayerHeight() / 2 + player_to_draw.y - world_map->view_y);
}

void PlayerHandler::render(OtherPlayer& player_to_draw) {
    ClientPlayer client_player = player_to_draw;
    render(client_player);
    player_to_draw.name_text.render(1, gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.x - world_map->view_x, gfx::getWindowHeight() / 2 - getPlayerHeight() / 2 + player_to_draw.y - world_map->view_y - player_to_draw.name_text.getTextureHeight() - HEADER_PADDING);
}

unsigned short PlayerHandler::getPlayerWidth() {
    return player_image.getTextureWidth() * 2;
}

unsigned short PlayerHandler::getPlayerHeight() {
    return player_image.getTextureHeight() * 2;
}
