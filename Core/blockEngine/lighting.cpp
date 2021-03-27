//
//  lighting.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 30/01/2021.
//

#include "core.hpp"

void blockEngine::removeNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < world_height && getBlock(x, y).getUniqueBlock().transparent; y++)
        removeLightSource(x, y);
}

void blockEngine::setNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < world_height && getBlock(x, y).getUniqueBlock().transparent; y++)
        addLightSource(x, y, MAX_LIGHT);
}

void blockEngine::addLightSource(unsigned short x, unsigned short y, unsigned char power) {
    block& block = getBlock(x, y);
    block.light_source = true;
    block.light_level = power;
    block.to_update_light = true;
}

void blockEngine::removeLightSource(unsigned short x, unsigned short y) {
    block& block = getBlock(x, y);
    block.light_source = false;
    block.light_level = 0;
    block.to_update_light = true;
}

void blockEngine::block::light_update(bool update) {
    unsigned short x = getX(), y = getY();
    if(update)
        to_update_light = false;
    block* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    if(x != 0 && getChunkState((x - 1) >> 4, y >> 4) == blockEngine::loaded)
        neighbors[0] = &getBlock(x - 1, y);
    if(x != blockEngine::world_width - 1 && getChunkState((x + 1) >> 4, y >> 4) == blockEngine::loaded)
        neighbors[1] = &getBlock(x + 1, y);
    if(y != 0 && getChunkState(x >> 4, (y - 1) >> 4) == blockEngine::loaded)
        neighbors[2] = &getBlock(x, y - 1);
    if(y != blockEngine::world_height - 1 && getChunkState(x >> 4, (y + 1) >> 4) == blockEngine::loaded)
        neighbors[3] = &getBlock(x, y + 1);
    bool update_neighbors = false;
    if(!light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors) {
            if(neighbor != nullptr) {
                auto light_step = (unsigned char)(neighbor->getUniqueBlock().transparent ? 3 : 15);
                auto light = (unsigned char)(light_step > neighbor->light_level ? 0 : neighbor->light_level - light_step);
                if(light > level_to_be)
                    level_to_be = light;
            }
        }
        if(!level_to_be)
            return;
        if(level_to_be != light_level) {
            light_level = level_to_be;
            light_change_data data;
            data.x = x;
            data.y = y;
            events::callEvent(light_change, (void*)&data);
            update_neighbors = true;
        }
    }
    if((update_neighbors || light_source) && update)
        for(int i = 0; i < 4; i++)
            if(neighbors[i] != nullptr && !neighbors[i]->light_source)
                neighbors[i]->to_update_light = true;
}
