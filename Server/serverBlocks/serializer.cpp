#include "serverBlocks.hpp"
#include <fstream>

std::vector<char> ServerBlocks::serialize() {
    std::vector<char> serial(width * height * 3);
    for(int y = 0; y < height; y++) {
        for(int x = 0; x < width; x++) {
            ServerBlock curr_block = getBlock(x, y);
            int pos = (y * width + x) * 3;
            serial[pos] = (char)curr_block.getBlockType();
            serial[pos + 1] = (char)curr_block.getLiquidType();
            serial[pos + 2] = (char)curr_block.getLiquidLevel();
        }
    }
    return serial;
}

void ServerBlocks::loadFromSerial(std::vector<char>& serial) {
    for(int y = 0; y < height; y++) {
        for(int x = 0; x < width; x++) {
            ServerBlock curr_block = getBlock(x, y);
            int pos = (y * width + x) * 3;
            curr_block.setTypeWithoutProcessing((BlockType)serial[pos]);
            curr_block.setTypeWithoutProcessing((LiquidType)serial[pos + 1]);
            curr_block.setLiquidLevel(serial[pos + 2]);
        }
    }
}
