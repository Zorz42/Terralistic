//
//  map.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "map.hpp"
#include "dev.hpp"
#include "networkingModule.hpp"

void map::createWorld(unsigned short width, unsigned short height) {
    blocks = new blockData[(width << 4) * (height << 4)];
    this->width = width << 4;
    this->height = height << 4;
}

map::~map() {
    if(blocks)
        delete[] blocks;
}

int map::getSpawnX() {
    return getWorldWidth() / 2 * BLOCK_WIDTH;
}

int map::getSpawnY() {
    int result = 0;
    for(unsigned short y = 0; y < getWorldHeight(); y++) {
        if(!getBlock(getWorldWidth() / 2 - 1, y).isTransparent() || !getBlock(getWorldWidth() / 2, y).isTransparent())
            break;
        result += BLOCK_WIDTH;
    }
    return result;
}
