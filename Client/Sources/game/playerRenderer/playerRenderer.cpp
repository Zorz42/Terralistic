//
//  playerRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/04/2021.
//

#include "playerRenderer.hpp"

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#else
#include "graphics.hpp"
#endif

static gfx::image player;

void playerRenderer::init() {
    player.setTexture(gfx::loadImageFile("texturePack/misc/player.png"));
}

#define HEADER_PADDING 4

void playerRenderer::render(int x, int y, int view_x, int view_y, bool flipped, gfx::image& header) {
    render(x, y, view_x, view_y, flipped);
    gfx::render(header, 1, gfx::getWindowWidth() / 2 - header.getTextureWidth() / 2 + x - view_x, gfx::getWindowHeight() / 2 - playerRenderer::getPlayerHeight() / 2 + y - view_y - header.getTextureHeight() - HEADER_PADDING);
}

void playerRenderer::render(int x, int y, int view_x, int view_y, bool flipped) {
    player.flipped = flipped;
    gfx::render(player, 2, gfx::getWindowWidth() / 2 - playerRenderer::getPlayerWidth() / 2 + x - view_x, gfx::getWindowHeight() / 2 - playerRenderer::getPlayerHeight() / 2 + y - view_y);
}

unsigned short playerRenderer::getPlayerWidth() {
    return player.getTextureWidth() * 2;
}

unsigned short playerRenderer::getPlayerHeight() {
    return player.getTextureHeight() * 2;
}
