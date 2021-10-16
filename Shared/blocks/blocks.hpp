#pragma once

#include <vector>
#include "properties.hpp"
#include "events.hpp"

#define BLOCK_WIDTH 8

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

class BlockStartedBreakingEvent {
public:
    BlockStartedBreakingEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class BlockStoppedBreakingEvent {
public:
    BlockStoppedBreakingEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class Blocks {
    struct Block {
        Block() : type(BlockType::AIR) {}
        BlockType type:8;
    };
    
    struct BreakingBlock {
        unsigned short break_progress = 0;
        bool is_breaking = true;
        int x, y;
    };
    
    Block *blocks = nullptr;
    int width, height;
    
    std::vector<BreakingBlock> breaking_blocks;
    
    Block* getBlock(int x, int y);
public:
    void create(int width, int height);
    
    const BlockInfo& getBlockInfo(int x, int y);
    BlockType getBlockType(int x, int y);
    void setBlockType(int x, int y, BlockType type);
    void setBlockTypeSilently(int x, int y, BlockType type);
    
    unsigned short getBreakProgress(int x, int y);
    unsigned char getBreakStage(int x, int y);
    void startBreakingBlock(int x, int y);
    void stopBreakingBlock(int x, int y);
    void updateBreakingBlocks(int frame_length);
    
    void breakBlock(int x, int y);
    
    int getWidth() const;
    int getHeight() const;
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    EventSender<BlockChangeEvent> block_change_event;
    EventSender<BlockBreakEvent> block_break_event;
    EventSender<BlockStartedBreakingEvent> block_started_breaking_event;
    EventSender<BlockStoppedBreakingEvent> block_stopped_breaking_event;
    
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
