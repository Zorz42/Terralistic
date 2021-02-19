//
//  lighting.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 30/01/2021.
//

#include "blockEngine.hpp"

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

