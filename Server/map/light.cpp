//
//  light.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "map.hpp"

void map::block::lightUpdate(bool update) {
    if(update)
        block_data->to_update_light = false;
    std::pair<short, short> neighbors[4] = {{-1, -1}, {-1, -1}, {-1, -1}, {-1, -1}};
    if(x != 0 && parent_map->getChunkState((x - 1) >> 4, y >> 4) == chunkState::loaded)
        neighbors[0] = {x - 1, y};
    if(x != parent_map->getWorldWidth() - 1 && parent_map->getChunkState((x + 1) >> 4, y >> 4) == chunkState::loaded)
        neighbors[1] = {x + 1, y};
    if(y != 0 && parent_map->getChunkState(x >> 4, (y - 1) >> 4) == chunkState::loaded)
        neighbors[2] = {x, y - 1};
    if(y != parent_map->getWorldHeight() - 1 && parent_map->getChunkState(x >> 4, (y + 1) >> 4) == chunkState::loaded)
        neighbors[3] = {x, y + 1};
    bool update_neighbors = false;
    if(!block_data->light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors) {
            if(neighbor.first != -1) {
                block neighbor_block = parent_map->getBlock(neighbor.first, neighbor.second);
                auto light_step = (unsigned char)(neighbor_block.isTransparent() ? 3 : 15);
                auto light = (unsigned char)(light_step > neighbor_block.getLightLevel() ? 0 : neighbor_block.getLightLevel() - light_step);
                if(light > level_to_be)
                    level_to_be = light;
            }
        }
        if(!level_to_be)
            return;
        if(level_to_be != getLightLevel()) {
            block_data->light_level = level_to_be;
            parent_map->onLightChange(*this);
        }
    }
    if((update_neighbors || isLightSource()) && update)
        for(auto neighbor : neighbors)
            if(neighbor.first != -1 && !parent_map->getBlock(neighbor.first, neighbor.second).isLightSource())
                parent_map->getBlock(neighbor.first, neighbor.second).scheduleLightUpdate();
}

void map::block::setLightSource(unsigned char power) {
    block_data->light_source = true;
    block_data->light_level = power;
    block_data->to_update_light = true;
}

void map::block::removeLightSource() {
    block_data->light_source = false;
    block_data->light_level = 0;
    block_data->to_update_light = true;
}

void map::setNaturalLight() {
    for(unsigned short x = 0; x < getWorldWidth(); x++)
        setNaturalLight(x);
}

void map::setNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < getWorldHeight() && getBlock(x, y).isTransparent(); y++)
        getBlock(x, y).setLightSource(MAX_LIGHT);
}

void map::removeNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < getWorldHeight() && getBlock(x, y).isTransparent(); y++)
        getBlock(x, y).removeLightSource();
}
