//
//  chunk.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "clientMap.hpp"
#include <cassert>

void map::chunk::updateTexture() {
    chunk_data->update = false;
    gfx::setRenderTarget(chunk_data->back_texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            block curr_block = parent_map->getBlock((x << 4) + x_, (y << 4) + y_);
            if(curr_block.block_data->update)
                curr_block.updateOrientation();
        }
    chunk_data->back_texture.clear();
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            block curr_block = parent_map->getBlock((x << 4) + x_, (y << 4) + y_);
            curr_block.drawBack();
        }
    gfx::setRenderTarget(chunk_data->front_texture);
    chunk_data->front_texture.clear();
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            block curr_block = parent_map->getBlock((x << 4) + x_, (y << 4) + y_);
            curr_block.drawFront();
        }
    gfx::resetRenderTarget();
}

void map::chunk::drawBack() {
    chunk_data->back_texture.render(1, (x << 4) * BLOCK_WIDTH - parent_map->view_x + (gfx::getWindowWidth() >> 1), (y << 4) * BLOCK_WIDTH - parent_map->view_y + (gfx::getWindowHeight() >> 1));
}

void map::chunk::drawFront() {
    chunk_data->front_texture.render(1, (x << 4) * BLOCK_WIDTH - parent_map->view_x + (gfx::getWindowWidth() >> 1), (y << 4) * BLOCK_WIDTH - parent_map->view_y + (gfx::getWindowHeight() >> 1));
}

void map::chunk::createTexture() {
    chunk_data->back_texture.createBlankImage(BLOCK_WIDTH * 16, BLOCK_WIDTH * 16);
    chunk_data->front_texture.createBlankImage(BLOCK_WIDTH * 16, BLOCK_WIDTH * 16);
}

map::chunk map::getChunk(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < (getWorldHeight() >> 4) && x >= 0 && x < (getWorldWidth() >> 4));
    return chunk(x, y, &chunks[y * (getWorldWidth() >> 4) + x], this);
}
