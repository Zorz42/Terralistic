#include "blocks.hpp"
#include <cassert>

void Blocks::createWorld(unsigned short world_width, unsigned short world_height) {
    assert(world_width % 16 == 0);
    assert(world_height % 16 == 0);
    blocks = new MapBlock[world_width * world_height];
    width = world_width;
    height = world_height;
    biomes = new Biome[width];

}

int Blocks::getSpawnX() {
    return width / 2 * BLOCK_WIDTH;
}

int Blocks::getSpawnY() {
    int spawn_y = 0;
    for(unsigned short y = 0; y < height; y++) {
        if(!getBlock(width / 2 - 1, y).getUniqueBlock().transparent || !getBlock(width / 2, y).getUniqueBlock().transparent)
            break;
        spawn_y += BLOCK_WIDTH;
    }
    return spawn_y;
}

Blocks::~Blocks() {
    delete[] blocks;
}
