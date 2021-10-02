#pragma once

#include <vector>
#include "properties.hpp"
#include "events.hpp"

#define BLOCK_WIDTH 8
#define MAX_LIGHT 100

class BlockChangeEvent {
public:
    BlockChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class BlockBreakEvent {
public:
    BlockBreakEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class BlockBreakStageChangeEvent {
public:
    BlockBreakStageChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class Blocks {
    struct Block {
        BlockType type:8;
        unsigned char break_stage;
        unsigned short break_progress = 0;
    };
    
    Block *blocks = nullptr;
    int width, height;
    
    Block* getBlock(int x, int y);
public:
    void create(int width, int height);
    
    const BlockInfo& getBlockInfo(int x, int y);
    BlockType getBlockType(int x, int y);
    void setBlockType(int x, int y, BlockType type);
    void setBlockTypeSilently(int x, int y, BlockType type);
    
    unsigned short getBreakProgress(int x, int y);
    void setBreakProgress(int x, int y, unsigned short progress);
    unsigned char getBreakStage(int x, int y);
    
    void breakBlock(int x, int y);
    
    unsigned short getWidth();
    unsigned short getHeight();
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    EventSender<BlockChangeEvent> block_change_event;
    EventSender<BlockBreakEvent> block_break_event;
    EventSender<BlockBreakStageChangeEvent> block_break_stage_change_event;
    
    ~Blocks();
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
