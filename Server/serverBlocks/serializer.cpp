#include "serverBlocks.hpp"
#include <fstream>

void ServerBlocks::serialize(std::vector<char>& serial) {
    serial.reserve(width * height * 3);
    for(int y = 0; y < height; y++) {
        for(int x = 0; x < width; x++) {
            ServerBlock curr_block = getBlock(x, y);
            serial.push_back((char)curr_block.getBlockType());
            serial.push_back((char)curr_block.getLiquidType());
            serial.push_back((char)curr_block.getLiquidLevel());
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
