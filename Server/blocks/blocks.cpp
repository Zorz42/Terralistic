#include <cassert>
#include "serverBlocks.hpp"

void ServerBlocks::createWorld(unsigned short world_width, unsigned short world_height) {
    width = world_width;
    height = world_height;
    
    assert(width % 16 == 0 && height % 16 == 0);
    blocks = new ServerMapBlock[width * height];
    biomes = new Biome[width];
}

int ServerBlocks::getSpawnX() const {
    return width / 2 * BLOCK_WIDTH * 2;
}

int ServerBlocks::getSpawnY() {
    int spawn_y = 0;
    for(unsigned short y = 0; y < height; y++) {
        if(!getBlock(width / 2 - 1, y).getUniqueBlock().transparent || !getBlock(width / 2, y).getUniqueBlock().transparent)
            break;
        spawn_y += BLOCK_WIDTH * 2;
    }
    return spawn_y;
}

ServerBlocks::~ServerBlocks() {
    delete[] blocks;
}
