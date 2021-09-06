#include <cassert>
#include "serverBlocks.hpp"

void ServerBlocks::createWorld(unsigned short world_width, unsigned short world_height) {
    width = world_width;
    height = world_height;
    
    blocks = new ServerMapBlock[width * height];
    biomes = new Biome[width];
}

int ServerBlocks::getSpawnX() const {
    return width / 2 * BLOCK_WIDTH * 2;
}

int ServerBlocks::getSpawnY() {
    int spawn_y = 0;
    for(unsigned short y = 0; y < height; y++) {
        if(!getBlock(width / 2 - 1, y).getBlockInfo().transparent || !getBlock(width / 2, y).getBlockInfo().transparent)
            break;
        spawn_y += BLOCK_WIDTH * 2;
    }
    return spawn_y;
}

ServerBlocks::~ServerBlocks() {
    delete[] blocks;
}

std::vector<char> ServerBlocks::toData() {
    std::vector<char> map_data(width * height * 4);
    
    ServerMapBlock* block_iter = blocks;
    int* map_data_iter = (int*)&map_data[0];
    for(int i = 0; i < width * height; i++) {
        *map_data_iter++ = (int)block_iter->block_type | (int)block_iter->liquid_type << 8 | (int)block_iter->liquid_level << 16 | (int)block_iter->light_level << 24;
        block_iter++;
    }
    
    return map_data;
}
