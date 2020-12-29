//
//  lightBlock.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/12/2020.
//

#include "lightingEngine.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"

void lightingEngine::lightBlock::render(short x, short y) const {
    if(level != MAX_LIGHT) {
        SDL_Rect rect = {x, y, BLOCK_WIDTH, BLOCK_WIDTH};
        auto light_level = static_cast<unsigned char>(255 - 255.0 / MAX_LIGHT * level);
        swl::setDrawColor(0, 0, 0, light_level);
        swl::render(rect);
    }
}

void lightingEngine::lightBlock::update(bool update) {
    lightBlock* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    if(getX() != 0)
        neighbors[0] = this - 1;
    if(getX() != blockEngine::world_width - 1)
        neighbors[1] = this + 1;
    if(getY() != 0)
        neighbors[2] = this - blockEngine::world_width;
    if(getY() != blockEngine::world_height - 1)
        neighbors[3] = this + blockEngine::world_width;
    bool update_neighbors = false;
    if(!source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors) {
            if(neighbor != nullptr) {
                unsigned char light_step = static_cast<unsigned char>(blockEngine::world[neighbor -
                                                                                         light_map].getUniqueBlock().transparent
                                                                      ? 3 : 15);
                unsigned char light = static_cast<unsigned char>(light_step > neighbor->level ? 0 : neighbor->level -
                                                                                                    light_step);
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
    if((update_neighbors || source) && update)
        for(auto & neighbor : neighbors)
            if(neighbor != nullptr && !neighbor->source)
                neighbor->update();
}

unsigned short lightingEngine::lightBlock::getX() {
    return static_cast<unsigned short>((unsigned int) (this - light_map) % blockEngine::world_width);
}

unsigned short lightingEngine::lightBlock::getY() {
    return static_cast<unsigned short>((unsigned int) (this - light_map) / blockEngine::world_width);
}

