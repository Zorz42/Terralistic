//
//  chunk.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 30/12/2020.
//

#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"

void blockEngine::chunk::render(unsigned short x, unsigned short y) {
    SDL_Rect rect = {((x << 4) * BLOCK_WIDTH - view_x + swl::window_width / 2), ((y << 4) * BLOCK_WIDTH - view_y + swl::window_height / 2), BLOCK_WIDTH << 4, BLOCK_WIDTH << 4};
    swl::render(texture, rect);
    /*for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++)
            blocks[x_][y_].draw((x << 4) + x_, (y << 4) + y_);*/
}

void blockEngine::chunk::updateTexture() {
    texture = swl::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4);
    swl::setRenderTarget(texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            SDL_Rect rect = {x_ * BLOCK_WIDTH, y_ * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH};
            swl::setDrawColor(135, 206, 235);
            swl::render(rect);
            blocks[x_][y_].draw(x_, y_);
            light_blocks[x_][y_].render(x_, y_);
        }
    swl::resetRenderTarget();
}
