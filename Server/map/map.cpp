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
    chunk_states = new chunkState[width * height];
    for(int i = 0; i < width * height; i++)
        chunk_states[i] = chunkState::unloaded;
    blocks = new blockData[(width << 4) * (height << 4)];
    this->width = width << 4;
    this->height = height << 4;
}

map::~map() {
    if(chunk_states)
        delete[] chunk_states;
    if(blocks)
        delete[] blocks;
}

map::chunkState& map::getChunkState(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (getWorldHeight() >> 4) && x >= 0 && x < (getWorldWidth() >> 4), "requested chunk is out of bounds");
    return chunk_states[y * (getWorldWidth() >> 4) + x];
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

void map::onBlockChange(block& block) {
    packets::packet packet(packets::BLOCK_CHANGE);
    packet << block.getX() << block.getY() << (unsigned char) block.getType();
    networking::sendToEveryone(packet);
}

void map::onLightChange(block& block) {
    
}

void map::onBreakStageChange(block& block) {
    
}
