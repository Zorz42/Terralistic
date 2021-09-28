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

void Blocks::setBlockType(unsigned short x, unsigned short y, BlockType type) {
    if(type != getBlock(x, y)->type) {
        BlockChangeEvent event(x, y, type);
        block_change_event.call(event);
        
        setBlockTypeSilently(x, y, type);
    }
}

void Blocks::setBlockTypeSilently(unsigned short x, unsigned short y, BlockType type) {
    if((int)type < 0 || type >= BlockType::NUM_BLOCKS)
        throw InvalidBlockTypeException();
    getBlock(x, y)->type = type;
}

unsigned short Blocks::getBreakProgress(unsigned short x, unsigned short y) {
    return getBlock(x, y)->break_progress;
}

void Blocks::setBreakProgress(unsigned short x, unsigned short y, unsigned short progress) {
    Block* block = getBlock(x, y);
    block->break_progress = progress;
    unsigned char stage = (unsigned char)((float)block->break_progress / (float)getBlockInfo(x, y).break_time * 9.f);
    if(stage != block->break_stage) {
        BlockBreakStageChangeEvent event(x, y);
        block_break_stage_change_event.call(event);
        
        block->break_stage = stage;
    }
}

unsigned char Blocks::getBreakStage(unsigned short x, unsigned short y) {
    return getBlock(x, y)->break_stage;
}

void Blocks::breakBlock(unsigned short x, unsigned short y) {
    setBlockType(x, y, BlockType::AIR);
    setBreakProgress(x, y, 0);
    
    BlockBreakEvent event(x, y);
    block_break_event.call(event);
}

unsigned short Blocks::getWidth() {
    return width;
}

unsigned short Blocks::getHeight() {
    return height;
}
