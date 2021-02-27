//
//  chunk.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 30/12/2020.
//

#define FILENAME chunk
#define NAMESPACE blockEngine
#include "core.hpp"

#include "playerHandler.hpp"

void blockEngine::chunk::render(unsigned short x, unsigned short y) const {
    swl::rect rect = {short((x << 4) * BLOCK_WIDTH - playerHandler::view_x + swl::window_width / 2), short((y << 4) * BLOCK_WIDTH - playerHandler::view_y + swl::window_height / 2), BLOCK_WIDTH << 4, BLOCK_WIDTH << 4};
    swl::render(texture, rect);
}

void blockEngine::chunk::createTexture() {
    texture = swl::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4);
}
