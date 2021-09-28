#include "serverBlocks-old.hpp"
#include <fstream>

void ServerBlocks::serialize(std::vector<char>& serial) {
    unsigned long initial_pos = serial.size(), iter = initial_pos;
    serial.resize(serial.size() + width * height * 3);
    ServerMapBlock* block_iter = blocks;
    for(int i = 0; i < width * height; i++) {
        serial[iter++] = (char)block_iter->block_type;
        serial[iter++] = (char)block_iter->liquid_type;
        serial[iter++] = (char)block_iter->liquid_level;
        block_iter++;
    }
}

char* ServerBlocks::loadFromSerial(char* iter) {
    ServerMapBlock* block_iter = blocks;
    for(int i = 0; i < width * height; i++) {
        block_iter->block_type = (BlockType)*iter++;
        block_iter->liquid_type = (LiquidType)*iter++;
        block_iter->liquid_level = *iter++;
        
        block_iter++;
    }
    return iter;
}
