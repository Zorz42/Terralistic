//
//  light.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "blocks.hpp"

void block::lightUpdate() {
    block_data->update_light = false;
    
    block neighbors[4];
    if(x != 0)
        neighbors[0] = parent_map->getBlock(x - 1, y);
    if(x != parent_map->getWidth() - 1)
        neighbors[1] = parent_map->getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = parent_map->getBlock(x, y - 1);
    if(y != parent_map->getHeight() - 1)
        neighbors[3] = parent_map->getBlock(x, y + 1);
    
    if(!block_data->light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors) {
            if(neighbor.refersToABlock()) {
                unsigned char light_step = neighbor.getUniqueBlock().transparent ? 3 : 15;
                unsigned char light = light_step > neighbor.getLightLevel() ? 0 : neighbor.getLightLevel() - light_step;
                if(light > level_to_be)
                    level_to_be = light;
            }
        }
        setLightLevel(level_to_be);
    }
}

void block::setLightSource(unsigned char power) {
    block_data->light_source = true;
    setLightLevel(power);
}

void block::removeLightSource() {
    block_data->light_source = false;
    setLightLevel(0);
}

void blocks::setNaturalLight() {
    for(unsigned short x = 0; x < width; x++)
        setNaturalLight(x);
}

void blocks::setNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < height && getBlock(x, y).getUniqueBlock().transparent; y++)
        getBlock(x, y).setLightSource(MAX_LIGHT);
}

[[maybe_unused]] void blocks::removeNaturalLight(unsigned short x) {
    for(unsigned short y = 0; y < height && getBlock(x, y).getUniqueBlock().transparent; y++)
        getBlock(x, y).removeLightSource();
}

void block::setLightLevel(unsigned char light_level) {
    if(block_data->light_level != light_level) {
        block_data->light_level = light_level;
        block neighbors[4];
        if(x != 0)
            neighbors[0] = parent_map->getBlock(x - 1, y);
        if(x != parent_map->getWidth() - 1)
            neighbors[1] = parent_map->getBlock(x + 1, y);
        if(y != 0)
            neighbors[2] = parent_map->getBlock(x, y - 1);
        if(y != parent_map->getHeight() - 1)
            neighbors[3] = parent_map->getBlock(x, y + 1);
        for(auto neighbor : neighbors)
            if(neighbor.refersToABlock() && !neighbor.isLightSource())
                neighbor.scheduleLightUpdate();
        syncWithClient();
    }
}
