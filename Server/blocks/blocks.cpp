#include "blocks.hpp"
#include <cassert>
#include <fstream>

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
    int result = 0;
    for(unsigned short y = 0; y < height; y++) {
        if(!getBlock(width / 2 - 1, y).getUniqueBlock().transparent || !getBlock(width / 2, y).getUniqueBlock().transparent)
            break;
        result += BLOCK_WIDTH;
    }
    return result;
}

Blocks::~Blocks() {
    delete[] blocks;
}

void Blocks::saveTo(std::string path) {
    std::ofstream world_file(path, std::ios::binary);

    for(int y = 0; y < height; y++) {
        char* world_buffer = new char[width * 3];
        for(int x = 0; x < width; x++) {
            Block curr_block = getBlock(x, y);
            int pos = x * 3;
            world_buffer[pos] = (char)curr_block.getBlockType();
            world_buffer[pos + 1] = (char)curr_block.getLiquidType();
            world_buffer[pos + 2] = (char)curr_block.getLiquidLevel();
        }
        world_file.write(world_buffer, width * 3);
        delete[] world_buffer;
    }
    world_file.close();
}

void Blocks::loadFrom(std::string path) {
    std::ifstream world_file(path, std::ios::binary);
    world_file.unsetf(std::ios::skipws);

    for(int y = 0; y < height; y++) {
        char* world_buffer = new char[width * 3];
        world_file.read(world_buffer, width * 3);

        for(int x = 0; x < width; x++) {
            Block curr_block = getBlock(x, y);
            int pos = x * 3;
            curr_block.setTypeWithoutProcessing((BlockType)world_buffer[pos]);
            curr_block.setTypeWithoutProcessing((LiquidType)world_buffer[pos + 1]);
            curr_block.setLiquidLevel(world_buffer[pos + 2]);
        }
        delete[] world_buffer;
    }
    world_file.close();
}
