//
//  blocks.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#include "blocks.hpp"

void blocks::createWorld(unsigned short width, unsigned short height) {
    block_arr = new blockData[(width << 4) * (height << 4)];
    this->width = width << 4;
    this->height = height << 4;
    biomes = new biome[this->width];

}

int blocks::getSpawnX() const {
    return width / 2 * BLOCK_WIDTH;
}

int blocks::getSpawnY() {
    int result = 0;
    for(unsigned short y = 0; y < height; y++) {
        if(!getBlock(width / 2 - 1, y).isTransparent() || !getBlock(width / 2, y).isTransparent())
            break;
        result += BLOCK_WIDTH;
    }
    return result;
}

blocks::~blocks() {
    delete[] block_arr;
}
