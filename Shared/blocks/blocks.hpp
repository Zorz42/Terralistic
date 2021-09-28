#ifndef blocks_hpp
#define blocks_hpp

#include <vector>
#include "properties.hpp"
#include "events.hpp"

#define BLOCK_WIDTH 8
#define MAX_LIGHT 100

class BlockChangeEvent {
public:
    BlockChangeEvent(unsigned short x, unsigned short y, BlockType type) : x(x), y(y) {}
    unsigned short x, y;
};

class BlockBreakEvent {
public:
    BlockBreakEvent(unsigned short x, unsigned short y) : x(x), y(y) {}
    unsigned short x, y;
};

class BlockBreakStageChangeEvent {
public:
    BlockBreakStageChangeEvent(unsigned short x, unsigned short y) : x(x), y(y) {}
    unsigned short x, y;
};

class Blocks {
    struct Block {
        BlockType type:8;
        unsigned char break_stage;
        unsigned short break_progress = 0;
    };
    
    Block *blocks = nullptr;
    unsigned short width, height;
    
    Block* getBlock(unsigned short x, unsigned short y);
public:
    void create(unsigned short width, unsigned short height);
    
    const BlockInfo& getBlockInfo(unsigned short x, unsigned short y);
    BlockType getBlockType(unsigned short x, unsigned short y);
    void setBlockType(unsigned short x, unsigned short y, BlockType type);
    void setBlockTypeSilently(unsigned short x, unsigned short y, BlockType type);
    
    unsigned short getBreakProgress(unsigned short x, unsigned short y);
    void setBreakProgress(unsigned short x, unsigned short y, unsigned short progress);
    unsigned char getBreakStage(unsigned short x, unsigned short y);
    
    void breakBlock(unsigned short x, unsigned short y);
    
    unsigned short getWidth(), getHeight();
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    EventSender<BlockChangeEvent> block_change_event;
    EventSender<BlockBreakEvent> block_break_event;
    EventSender<BlockBreakStageChangeEvent> block_break_stage_change_event;
};

class BlockOutOfBoundsException : public std::exception {
public:
    const char* what() const throw() {
        return "Block is accessed out of the bounds!";
    }
};

class InvalidBlockTypeException : public std::exception {
public:
    const char* what() const throw() {
        return "Block type does not exist!";
    }
};

#endif
