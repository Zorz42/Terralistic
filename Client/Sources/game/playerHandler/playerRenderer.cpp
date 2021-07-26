//
//  playerRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/04/2021.
//

#include "playerHandler.hpp"

#include "graphics.hpp"

void playerHandler::initRenderer() {
    player_image.loadFromFile("resourcePack/misc/player.png");
}

#define HEADER_PADDING 4

void playerHandler::render(int x, int y, int view_x, int view_y, bool flipped, gfx::Image& header) {
    render(x, y, view_x, view_y, flipped);
    header.render(1, gfx::getWindowWidth() / 2 - header.getTextureWidth() / 2 + x - view_x, gfx::getWindowHeight() / 2 - getPlayerHeight() / 2 + y - view_y - header.getTextureHeight() - HEADER_PADDING);
}

void playerHandler::render(int x, int y, int view_x, int view_y, bool flipped) {
    player_image.flipped = flipped;
    player_image.render(2, gfx::getWindowWidth() / 2 - getPlayerWidth() / 2 + x - view_x, gfx::getWindowHeight() / 2 - getPlayerHeight() / 2 + y - view_y);
}

unsigned short playerHandler::getPlayerWidth() {
    return player_image.getTextureWidth() * 2;
}

unsigned short playerHandler::getPlayerHeight() {
    return player_image.getTextureHeight() * 2;
}
