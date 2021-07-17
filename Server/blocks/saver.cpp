#include "blocks.hpp"
#include <fstream>

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
