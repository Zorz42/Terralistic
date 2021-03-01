//
//  blockEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#define FILENAME blockEngine
#define NAMESPACE blockEngine
#include "core.hpp"

INIT_SCRIPT
    using namespace blockEngine;
    unique_blocks = {
        uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
        uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::DIRT,        /*break_time*/1000),
        uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::STONE_BLOCK, /*break_time*/1000),
        uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
        uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/itemEngine::STONE,       /*break_time*/1000),
        uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
        uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING,     /*break_time*/1000),
    };
INIT_SCRIPT_END

void blockEngine::prepare() {
    chunks = new chunk[(world_width >> 4) * (world_height >> 4)];
    blocks = new block[world_width * world_height];
}

void blockEngine::close() {
    delete[] chunks;
    delete[] blocks;
}

blockEngine::block& blockEngine::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < world_height && x >= 0 && x < world_width, "requested block is out of bounds");
    return blocks[y * world_width + x];
}

blockEngine::chunk& blockEngine::getChunk(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (world_height >> 4) && x >= 0 && x < (world_width >> 4), "requested chunk is out of bounds");
    return chunks[y * (world_width >> 4) + x];
}

void blockEngine::prepareWorld() {
    for(unsigned short x = 0; x < world_width; x++)
        setNaturalLight(x);
}
