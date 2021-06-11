//
//  serverMap.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "serverMap.hpp"
#include "assert.hpp"
#include "serverNetworking.hpp"

void serverMap::createWorld(unsigned short width, unsigned short height) {
    blocks = new blockData[(width << 4) * (height << 4)];
    this->width = width << 4;
    this->height = height << 4;
    biomes = new biome[this->width];
}

serverMap::~serverMap() {
    if(blocks)
        delete[] blocks;
    for(player* i : all_players)
        delete i;
}
