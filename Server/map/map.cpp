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
