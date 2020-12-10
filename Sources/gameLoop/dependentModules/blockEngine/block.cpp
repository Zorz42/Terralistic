//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/12/2020.
//

#include <algorithm>
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"

void blockEngine::block::draw() {
    if(getUniqueBlock().texture) {
        SDL_Rect rect = getRect(), cutout_rect = {0, 8 * block_orientation, 8, 8};
        swl::render(getUniqueBlock().texture, rect, cutout_rect);
    }
}

SDL_Rect blockEngine::block::getRect() {
    SDL_Rect rect = {0, 0, BLOCK_WIDTH, BLOCK_WIDTH};
    rect.x = int(getX() * BLOCK_WIDTH - view_x + swl::window_width / 2);
    rect.y = int(getY() * BLOCK_WIDTH - view_y + swl::window_height / 2);
    return rect;
}

unsigned int blockEngine::block::getX() {
    return (unsigned int)(this - world) % world_width;
}

unsigned int blockEngine::block::getY() {
    return (unsigned int)(this - world) / world_width;
}

void blockEngine::block::update() {
    block_orientation = 0;
    
    if(getUniqueBlock().only_on_floor) {
        if(getBlock(getX(), getY() + 1).getUniqueBlock().transparent)
            getBlock(getX(), getY()).block_id = AIR;
    }
    
    if(!getUniqueBlock().single_texture) {
        char x[] = {0, 1, 0, -1};
        char y[] = {-1, 0, 1, 0};
        Uint8 c = 1;
        for(int i = 0; i < 4; i++) {
            if(getX() + x[i] >= world_width || getX() + x[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(getY() + y[i] >= world_height || getY() + y[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(getBlock(getX() + x[i], getY() + y[i]).block_id == block_id || std::count(getUniqueBlock().connects_to.begin(), getUniqueBlock().connects_to.end(), getBlock(getX() + x[i], getY() + y[i]).block_id))
                block_orientation += c;
            c += c;
        }
    }
}

blockEngine::uniqueBlock& blockEngine::block::getUniqueBlock() {
    return unique_blocks.at(block_id);
}
