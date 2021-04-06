//
//  map.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#include "core.hpp"

INIT_SCRIPT
    map::unique_blocks = {
        map::uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
        map::uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/map::itemType::DIRT,        /*break_time*/1000),
        map::uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/map::itemType::STONE_BLOCK, /*break_time*/1000),
        map::uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
        map::uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/map::itemType::STONE,       /*break_time*/1000),
        map::uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
        map::uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
    };
INIT_SCRIPT_END

void map::createWorld(unsigned short width, unsigned short height) {
    chunk_states = new chunkState[getWorldWidth() * getWorldHeight()];
    blocks = new blockData[(getWorldWidth() << 4) * (getWorldHeight() << 4)];
    width = width << 4;
    width = height << 4;
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
