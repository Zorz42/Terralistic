#include "blocks.hpp"

void Blocks::create(int width_, int height_) {
    width = width_;
    height = height_;
    blocks = new Block[width * height];
}

Blocks::Block* Blocks::getBlock(int x, int y) {
    if(x >= width || y >= height)
        throw BlockOutOfBoundsException();
    return &blocks[y * width + x];
}

const BlockInfo& Blocks::getBlockInfo(int x, int y) {
    return ::getBlockInfo(getBlock(x, y)->type);
}

BlockType Blocks::getBlockType(int x, int y) {
    return getBlock(x, y)->type;
}

void Blocks::setBlockTypeSilently(int x, int y, BlockType type) {
    if((int)type < 0 || type >= BlockType::NUM_BLOCKS)
        throw InvalidBlockTypeException();
    getBlock(x, y)->type = type;
}

void Blocks::setBlockType(int x, int y, BlockType type) {
    if(type != getBlock(x, y)->type) {
        setBlockTypeSilently(x, y, type);
        
        BlockChangeEvent event(x, y);
        block_change_event.call(event);
    }
}

unsigned short Blocks::getBreakProgress(int x, int y) {
    return getBlock(x, y)->break_progress;
}

void Blocks::setBreakProgress(int x, int y, unsigned short progress) {
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

unsigned char Blocks::getBreakStage(int x, int y) {
    return getBlock(x, y)->break_stage;
}

void Blocks::breakBlock(int x, int y) {
    setBreakProgress(x, y, 0);
    BlockBreakEvent event(x, y);
    block_break_event.call(event);
    
    setBlockType(x, y, BlockType::AIR);
}

unsigned short Blocks::getWidth() const {
    return width;
}

unsigned short Blocks::getHeight() const {
    return height;
}

void Blocks::serialize(std::vector<char>& serial) {
    unsigned long iter = serial.size();
    serial.resize(serial.size() + width * height + 4);
    *(unsigned short*)&serial[iter] = width;
    iter += 2;
    *(unsigned short*)&serial[iter] = height;
    iter += 2;
    Block* block = blocks;
    for(int i = 0; i < width * height; i++) {
        serial[iter++] = (char)block->type;
        block++;
    }
}

char* Blocks::loadFromSerial(char* iter) {
    unsigned short width_, height_;
    width_ = *(unsigned short*)iter;
    iter += 2;
    height_ = *(unsigned short*)iter;
    iter += 2;
    create(width_, height_);
    Block* block = blocks;
    for(int i = 0; i < width * height; i++) {
        block->type = (BlockType)*iter++;
        block++;
    }
    return iter;
}

Blocks::~Blocks() {
    delete[] blocks;
}
