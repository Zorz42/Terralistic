//
//  blocks.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#include "blocks.hpp"
#include <fstream>

void blocks::createWorld(unsigned short width, unsigned short height) {
    block_arr = new blockData[(width << 4) * (height << 4)];
    this->width = width << 4;
    this->height = height << 4;
    biomes = new biome[this->width];

}

int blocks::getSpawnX() const {
    return width / 2 * BLOCK_WIDTH;
}

int blocks::getSpawnY() {
    int result = 0;
    for(unsigned short y = 0; y < height; y++) {
        if(!getBlock(width / 2 - 1, y).isTransparent() || !getBlock(width / 2, y).isTransparent())
            break;
        result += BLOCK_WIDTH;
    }
    return result;
}

blocks::~blocks() {
    delete[] block_arr;
}

void blocks::saveTo(std::string path) {
    std::ofstream world_file(path, std::ios::binary);

    for(int y = 0; y < height; y++) {
        char* world_buffer = new char[width * 3];
        for(int x = 0; x < width; x++) {
            block curr_block = getBlock(x, y);
            int pos = x * 3;
            world_buffer[pos] = (char)curr_block.getType();
            world_buffer[pos + 1] = (char)curr_block.getLiquidType();
            world_buffer[pos + 2] = (char)curr_block.getLiquidLevel();
        }
        world_file.write(world_buffer, width * 3);
        delete[] world_buffer;
    }
    world_file.close();
}

void blocks::loadFrom(std::string path) {
    std::ifstream world_file(path, std::ios::binary);
    world_file.unsetf(std::ios::skipws);

    for(int y = 0; y < height; y++) {
        char* world_buffer = new char[width * 3];
        world_file.read(world_buffer, width * 3);

        for(int x = 0; x < width; x++) {
            block curr_block = getBlock(x, y);
            int pos = x * 3;
            curr_block.setType((blockType) world_buffer[pos], false);
            curr_block.setType((liquidType) world_buffer[pos + 1], false);
            curr_block.setLiquidLevel(world_buffer[pos + 2]);
        }
        delete[] world_buffer;
    }
    world_file.close();
}
