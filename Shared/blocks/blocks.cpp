#include "blocks.hpp"

void Blocks::create(unsigned short width_, unsigned short height_) {
    width = width_;
    height = height_;
    blocks = new Block[width * height];
}

Blocks::Block* Blocks::getBlock(unsigned short x, unsigned short y) {
    if(x >= width || y >= height)
        throw BlockOutOfBoundsException();
    return &blocks[y * width + x];
}

const BlockInfo& Blocks::getBlockInfo(unsigned short x, unsigned short y) {
    return ::getBlockInfo(getBlock(x, y)->type);
}

BlockType Blocks::getBlockType(unsigned short x, unsigned short y) {
    return getBlock(x, y)->type;
}

void Blocks::setBlockTypeSilently(unsigned short x, unsigned short y, BlockType type) {
    if((int)type < 0 || type >= BlockType::NUM_BLOCKS)
        throw InvalidBlockTypeException();
    getBlock(x, y)->type = type;
}

void Blocks::setBlockType(unsigned short x, unsigned short y, BlockType type) {
    if(type != getBlock(x, y)->type) {
        setBlockTypeSilently(x, y, type);
        
        BlockChangeEvent event(x, y);
        block_change_event.call(event);
    }
}

unsigned short Blocks::getBreakProgress(unsigned short x, unsigned short y) {
    return getBlock(x, y)->break_progress;
}

void Blocks::setBreakProgress(unsigned short x, unsigned short y, unsigned short progress) {
    Block* block = getBlock(x, y);
    if(block->break_progress != progress) {
        block->break_progress = progress;
        if(block->break_progress > getBlockInfo(x, y).break_time)
            breakBlock(x, y);
        
        unsigned char stage = (unsigned char)((float)block->break_progress / (float)getBlockInfo(x, y).break_time * 9.f);
        if(stage != block->break_stage) {
            block->break_stage = stage;
            
            BlockBreakStageChangeEvent event(x, y);
            block_break_stage_change_event.call(event);
        }
    }
}

unsigned char Blocks::getBreakStage(unsigned short x, unsigned short y) {
    return getBlock(x, y)->break_stage;
}

void Blocks::breakBlock(unsigned short x, unsigned short y) {
    setBreakProgress(x, y, 0);
    BlockBreakEvent event(x, y);
    block_break_event.call(event);
    
    setBlockType(x, y, BlockType::AIR);
}

unsigned short Blocks::getWidth() {
    return width;
}

unsigned short Blocks::getHeight() {
    return height;
}

void Blocks::serialize(std::vector<char>& serial) {
    unsigned long iter = serial.size();
    serial.resize(serial.size() + width * height);
    Block* block = blocks;
    for(int i = 0; i < width * height; i++) {
        serial[iter++] = (char)block->type;
        block++;
    }
}

char* Blocks::loadFromSerial(char* iter) {
    Block* block = blocks;
    for(int i = 0; i < width * height; i++) {
        block->type = (BlockType)*iter++;
        block++;
    }
    return iter;
}
