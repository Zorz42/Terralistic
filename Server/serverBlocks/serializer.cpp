#include "serverBlocks.hpp"
#include <fstream>

void ServerBlocks::serialize(std::vector<char>& serial) {
    unsigned long initial_pos = serial.size();
    serial.resize(serial.size() + width * height * 3);
    for(int y = 0; y < height; y++) {
        for(int x = 0; x < width; x++) {
            ServerBlock curr_block = getBlock(x, y);
            unsigned long pos = initial_pos + (y * width + x) * 3;
            serial[pos] = (char)curr_block.getBlockType();
            serial[pos + 1] = (char)curr_block.getLiquidType();
            serial[pos + 2] = (char)curr_block.getLiquidLevel();
        }
    }
}

char* ServerBlocks::loadFromSerial(char* iter) {
    for(int y = 0; y < height; y++) {
        for(int x = 0; x < width; x++) {
            ServerBlock curr_block = getBlock(x, y);
            curr_block.setTypeWithoutProcessing((BlockType)*iter++);
            curr_block.setTypeWithoutProcessing((LiquidType)*iter++);
            curr_block.setLiquidLevel(*iter++);
        }
    }
    return iter;
}
