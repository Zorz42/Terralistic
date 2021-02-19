//
//  chunk.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 30/12/2020.
//

#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"
#include "playerHandler.hpp"

void blockEngine::chunk::render(unsigned short x, unsigned short y) const {
    swl::rect rect = {short((x << 4) * BLOCK_WIDTH - playerHandler::view_x + swl::window_width / 2), short((y << 4) * BLOCK_WIDTH - playerHandler::view_y + swl::window_height / 2), BLOCK_WIDTH << 4, BLOCK_WIDTH << 4};
    swl::render(texture, rect);
}

void blockEngine::chunk::updateTexture() {
    update = false;
    swl::setRenderTarget(texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++)
            if(blocks[x_][y_].to_update) {
                swl::rect rect = {short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH};
                swl::setDrawColor(135, 206, 235);
                swl::render(rect);
                blocks[x_][y_].draw(x_, y_);
                blocks[x_][y_].to_update = false;
            }
    swl::resetRenderTarget();
}

void blockEngine::chunk::createTexture() {
    texture = swl::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4);
}
