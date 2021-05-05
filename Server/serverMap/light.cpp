//
//  light.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "serverMap.hpp"
#include "serverNetworking.hpp"

void map::block::lightUpdate() {
    block neighbors[4];
    if(x != 0)
        neighbors[0] = parent_map->getBlock(x - 1, y);
    if(x != parent_map->getWorldWidth() - 1)
        neighbors[1] = parent_map->getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = parent_map->getBlock(x, y - 1);
    if(y != parent_map->getWorldHeight() - 1)
        neighbors[3] = parent_map->getBlock(x, y + 1);
    
    bool update_neighbors = false;
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
        if(!level_to_be)
            return;
        if(level_to_be != getLightLevel()) {
            block_data->light_level = level_to_be;
            update_neighbors = true;
            packets::packet packet(packets::LIGHT_CHANGE);
            packet << getX() << getY() << (unsigned char)getLightLevel();
            parent_map->manager->sendToEveryone(packet);
        }
    }
    
    if(update_neighbors || isLightSource())
        for(auto neighbor : neighbors)
            if(neighbor.refersToABlock() && !neighbor.isLightSource())
                neighbor.scheduleLightUpdate();
}

void map::block::setLightSource(unsigned char power) {
    block_data->light_source = true;
    block_data->light_level = power;
    scheduleLightUpdate();
}

void map::block::removeLightSource() {
    block_data->light_source = false;
    block_data->light_level = 0;
    scheduleLightUpdate();
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
