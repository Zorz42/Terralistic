#include "blocks.hpp"
#include "exception.hpp"
#include "compress.hpp"

BlockType::BlockType(std::string name) : name(std::move(name)) {}

Blocks::Blocks() : air("air"), hand("hand") {
    air.ghost = true;
    air.transparent = true;
    air.break_time = UNBREAKABLE;
    registerNewBlockType(&air);
    registerNewToolType(&hand);
}

void Blocks::create(int width_, int height_) {
    if(width_ < 0 || height_ < 0)
        throw Exception("Width and height must be positive");
    
    width = width_;
    height = height_;
    blocks = new Block[width * height];
}

Blocks::Block* Blocks::getBlock(int x, int y) {
    if(x < 0 || x >= width || y < 0 || y >= height)
        throw Exception("Block is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &blocks[y * width + x];
}

BlockType* Blocks::getBlockType(int x, int y) {
    return getBlockTypeById(getBlock(x, y)->id);
}

void Blocks::setBlockTypeSilently(int x, int y, BlockType* type) {
    getBlock(x, y)->id = type->id;
}

void Blocks::setBlockType(int x, int y, BlockType* type) {
    if(type->id != getBlock(x, y)->id) {
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

int Blocks::getBreakProgress(int x, int y) {
    for(int i = 0; i < breaking_blocks.size(); i++)
        if(breaking_blocks[i].x == x && breaking_blocks[i].y == y)
            return breaking_blocks[i].break_progress;
    return 0;
}

void Blocks::updateBreakingBlocks(int frame_length) {
    for(int i = 0; i < breaking_blocks.size(); i++) {
        if(breaking_blocks[i].is_breaking) {
            breaking_blocks[i].break_progress += frame_length;
            if(breaking_blocks[i].break_progress > getBlockType(breaking_blocks[i].x, breaking_blocks[i].y)->break_time)
                breakBlock(breaking_blocks[i].x, breaking_blocks[i].y);
        }
    }
}

int Blocks::getBreakStage(int x, int y) {
    return (float)getBreakProgress(x, y) / (float)getBlockType(x, y)->break_time * 9.f;
}

void Blocks::startBreakingBlock(int x, int y) {
    if(x < 0 || x >= width || y < 0 || y >= height)
        throw Exception("Block is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    
    BreakingBlock* breaking_block = nullptr;
    
    for(int i = 0; i < breaking_blocks.size(); i++)
        if(breaking_blocks[i].x == x && breaking_blocks[i].y == y)
            breaking_block = &breaking_blocks[i];
    
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
    if(x < 0 || x >= width || y < 0 || y >= height)
        throw Exception("Block is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    
    for(int i = 0; i < breaking_blocks.size(); i++)
        if(breaking_blocks[i].x == x && breaking_blocks[i].y == y) {
            breaking_blocks[i].is_breaking = false;
            BlockStoppedBreakingEvent event(x, y);
            block_stopped_breaking_event.call(event);
        }
}

void Blocks::breakBlock(int x, int y) {
    BlockBreakEvent event(x, y);
    block_break_event.call(event);
    
    setBlockType(x, y, &air);
}

int Blocks::getWidth() const {
    return width;
}

int Blocks::getHeight() const {
    return height;
}

std::vector<char> Blocks::toSerial() {
    std::vector<char> serial;
    unsigned long iter = 0;
    serial.resize(serial.size() + width * height + 4);
    *(unsigned short*)&serial[iter] = width;
    iter += 2;
    *(unsigned short*)&serial[iter] = height;
    iter += 2;
    Block* block = blocks;
    for(int i = 0; i < width * height; i++) {
        serial[iter++] = (char)block->id;
        block++;
    }
    return compress(serial);
}

void Blocks::fromSerial(const std::vector<char>& serial) {
    std::vector<char> decompressed = decompress(serial);
    
    const char* iter = &decompressed[0];
    int width_, height_;
    width_ = *(unsigned short*)iter;
    iter += 2;
    height_ = *(unsigned short*)iter;
    iter += 2;
    create(width_, height_);
    Block* block = blocks;
    for(int i = 0; i < width * height; i++) {
        block->id = *iter++;
        block++;
    }
}

void Blocks::registerNewBlockType(BlockType* block_type) {
    block_type->id = (int)block_types.size();
    block_types.push_back(block_type);
}

BlockType* Blocks::getBlockTypeById(int block_id) {
    return block_types[block_id];
}

BlockType* Blocks::getBlockTypeByName(const std::string& name) {
    for(int i = 0; i < block_types.size(); i++)
        if(block_types[i]->name == name)
            return block_types[i];
    return nullptr;
}

void Blocks::registerNewToolType(Tool* tool) {
    tool_types.push_back(tool);
}

Tool* Blocks::getToolTypeByName(const std::string& name) {
    for(int i = 0; i < tool_types.size(); i++)
        if(tool_types[i]->name == name)
            return tool_types[i];
    return nullptr;
}

int Blocks::getNumBlockTypes() {
    return (int)block_types.size();
}

Blocks::~Blocks() {
    delete[] blocks;
}
