//
//  light.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "serverMap.hpp"

void serverMap::block::lightUpdate() {
    block_data->update_light = false;
    
    block neighbors[4];
    if(x != 0)
        neighbors[0] = parent_map->getBlock(x - 1, y);
    if(x != parent_map->width - 1)
        neighbors[1] = parent_map->getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = parent_map->getBlock(x, y - 1);
    if(y != parent_map->height - 1)
        neighbors[3] = parent_map->getBlock(x, y + 1);
    
    if(!block_data->light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors) {
            if(neighbor.refersToABlock()) {
                unsigned char light_step = neighbor.isTransparent() ? 3 : 15;
                unsigned char light = light_step > neighbor.getLightLevel() ? 0 : neighbor.getLightLevel() - light_step;
                if(light > level_to_be)
                    level_to_be = light;
            }
        }
        setLightLevel(level_to_be);
    }
}

void serverMap::block::setLightSource(unsigned char power) {
    block_data->light_source = true;
    setLightLevel(power);
}

void serverMap::block::removeLightSource() {
    block_data->light_source = false;
    setLightLevel(0);
}

void serverMap::setNaturalLight() {
    for(unsigned short x = 0; x < width; x++)
        setNaturalLight(x);
}

void serverMap::setNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < height && getBlock(x, y).isTransparent(); y++)
        getBlock(x, y).setLightSource(MAX_LIGHT);
}

[[maybe_unused]] void serverMap::removeNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < height && getBlock(x, y).isTransparent(); y++)
        getBlock(x, y).removeLightSource();
}

void serverMap::block::setLightLevel(unsigned char light_level) {
    if(block_data->light_level != light_level) {
        block_data->light_level = light_level;
        block neighbors[4];
        if(x != 0)
            neighbors[0] = parent_map->getBlock(x - 1, y);
        if(x != parent_map->width - 1)
            neighbors[1] = parent_map->getBlock(x + 1, y);
        if(y != 0)
            neighbors[2] = parent_map->getBlock(x, y - 1);
        if(y != parent_map->height - 1)
            neighbors[3] = parent_map->getBlock(x, y + 1);
        for(auto neighbor : neighbors)
            if(neighbor.refersToABlock() && !neighbor.isLightSource())
                neighbor.scheduleLightUpdate();
        if(!parent_map->online_players.empty())
            syncWithClient();
    }
}
