//
//  lightBlock.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/12/2020.
//

#include "lightingEngine.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"

void lightingEngine::lightBlock::render(unsigned short x, unsigned short y) const {
    if(level != MAX_LIGHT) {
        SDL_Rect rect = {x * BLOCK_WIDTH, y * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH};
        auto light_level = (unsigned char)(255 - 255.0 / MAX_LIGHT * level);
        swl::setDrawColor(0, 0, 0, light_level);
        swl::render(rect);
    }
}

void lightingEngine::lightBlock::update(unsigned short x, unsigned short y, bool update) {
    if(update)
        to_update = false;
    lightBlock* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    unsigned short x_[] = {(unsigned short)(x - 1), (unsigned short)(x + 1), x, x}, y_[] = {y, y, (unsigned short)(y - 1), (unsigned short)(y + 1)};
    if(x != 0)
        neighbors[0] = &getLightBlock(x - 1, y);
    if(x != blockEngine::world_width - 1)
        neighbors[1] = &getLightBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = &getLightBlock(x, y - 1);
    if(y != blockEngine::world_height - 1)
        neighbors[3] = &getLightBlock(x, y + 1);
    bool update_neighbors = false;
    if(!source) {
        unsigned char level_to_be = 0;
        for(int i = 0; i < 4; i++) {
            if(neighbors[i] != nullptr) {
                auto light_step = (unsigned char)(blockEngine::getBlock(x_[i], y_[i]).getUniqueBlock().transparent ? 3 : 15);
                auto light = (unsigned char)(light_step > neighbors[i]->level ? 0 : neighbors[i]->level - light_step);
                if(light > level_to_be)
                    level_to_be = light;
            }
        }
        if(!level_to_be)
            return;
        if(level_to_be != level) {
            level = level_to_be;
            update_neighbors = true;
        }
    }
    if((update_neighbors || source) && update) {
        blockEngine::setUpdateBlock(x, y, true);
        blockEngine::getChunk(x >> 4, y >> 4).update = true;
        for(int i = 0; i < 4; i++)
            if(neighbors[i] != nullptr && !neighbors[i]->source)
                neighbors[i]->update(x_[i], y_[i]);
    }
}
