//
//  playerRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/04/2021.
//

#include "playerRenderer.hpp"

#include "graphics.hpp"

static gfx::Image player;

void playerRenderer::init() {
    player.setTexture(gfx::loadImageFile("texturePack/misc/player.png"));
}

#define HEADER_PADDING 4

void playerRenderer::render(int x, int y, int view_x, int view_y, bool flipped, gfx::Image& header) {
    render(x, y, view_x, view_y, flipped);
    header.render(1, gfx::getWindowWidth() / 2 - header.getTextureWidth() / 2 + x - view_x, gfx::getWindowHeight() / 2 - playerRenderer::getPlayerHeight() / 2 + y - view_y - header.getTextureHeight() - HEADER_PADDING);
}

void playerRenderer::render(int x, int y, int view_x, int view_y, bool flipped) {
    player.flipped = flipped;
    player.render(2, gfx::getWindowWidth() / 2 - playerRenderer::getPlayerWidth() / 2 + x - view_x, gfx::getWindowHeight() / 2 - playerRenderer::getPlayerHeight() / 2 + y - view_y);
}

unsigned short playerRenderer::getPlayerWidth() {
    return player.getTextureWidth() * 2;
}

unsigned short playerRenderer::getPlayerHeight() {
    return player.getTextureHeight() * 2;
}
