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
        
        for(int i = 0; i < breaking_blocks.size(); i++)
            if(breaking_blocks[i].x == x && breaking_blocks[i].y == y) {
                breaking_blocks.erase(breaking_blocks.begin() + i);
                break;
            }
        
        BlockChangeEvent event(x, y);
        block_change_event.call(event);
    }
}

unsigned short Blocks::getBreakProgress(int x, int y) {
    for(BreakingBlock breaking_block : breaking_blocks)
        if(breaking_block.x == x && breaking_block.y == y)
            return breaking_block.break_progress;
    return 0;
}

void Blocks::updateBreakingBlocks(int frame_length) {
    for(int i = 0; i < breaking_blocks.size(); i++) {
        if(breaking_blocks[i].is_breaking) {
            breaking_blocks[i].break_progress += frame_length;
            if(breaking_blocks[i].break_progress > getBlockInfo(breaking_blocks[i].x, breaking_blocks[i].y).break_time)
                breakBlock(breaking_blocks[i].x, breaking_blocks[i].y);
        }
    }
}

unsigned char Blocks::getBreakStage(int x, int y) {
    return (float)getBreakProgress(x, y) / (float)getBlockInfo(x, y).break_time * 9.f;
}

void Blocks::startBreakingBlock(int x, int y) {
    BreakingBlock* breaking_block = nullptr;
    
    for(BreakingBlock& block : breaking_blocks)
        if(block.x == x && block.y == y)
            breaking_block = &block;
    
    if(!breaking_block) {
        BreakingBlock new_breaking_block;
        new_breaking_block.x = x;
        new_breaking_block.y = y;
        breaking_blocks.push_back(new_breaking_block);
        breaking_block = &breaking_blocks.back();
    }
    
    breaking_block->is_breaking = true;
        
    BlockStartedBreakingEvent event(x, y);
    block_started_breaking_event.call(event);
}

void Blocks::stopBreakingBlock(int x, int y) {
    for(BreakingBlock& breaking_block : breaking_blocks)
        if(breaking_block.x == x && breaking_block.y == y) {
            breaking_block.is_breaking = false;
            BlockStoppedBreakingEvent event(x, y);
            block_stopped_breaking_event.call(event);
        }
}

void Blocks::breakBlock(int x, int y) {
    BlockBreakEvent event(x, y);
    block_break_event.call(event);
    
    setBlockType(x, y, BlockType::AIR);
}

int Blocks::getWidth() const {
    return width;
}

int Blocks::getHeight() const {
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
