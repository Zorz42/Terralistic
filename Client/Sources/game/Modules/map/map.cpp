//
//  map.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "map.hpp"
#include "dev.hpp"

void map::createWorld(unsigned short width, unsigned short height) {
    chunks = new chunkData[width * height];
    blocks = new blockData[(width << 4) * (height << 4)];
    this->width = width << 4;
    this->height = height << 4;
    
    for(unsigned short x = 0; x < (getWorldWidth() >> 4); x++)
        for(unsigned short y = 0; y < (getWorldHeight() >> 4); y++)
            getChunk(x, y).createTexture();
}

map::~map() {
    if(chunks)
        delete[] chunks;
    if(blocks)
        delete[] blocks;
}
