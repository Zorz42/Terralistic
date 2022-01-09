#pragma once
#include "events.hpp"
#include "graphics.hpp"

#define BLOCK_WIDTH 8
#define UNBREAKABLE -1

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

class Tool {
public:
    Tool(const std::string& name) : name(name) {}
    std::string name;
};

class BlockType {
public:
    BlockType(std::string name);
    Tool* effective_tool;
    int required_tool_power;
    bool ghost, transparent;
    std::string name;
    std::vector<BlockType*> connects_to;
    int break_time;
    int id;
};

class Blocks {
    class Block {
    public:
        Block() : id(/*air*/0) {}
        int id:8;
    };
    
    class BreakingBlock {
    public:
        int break_progress = 0;
        bool is_breaking = true;
        int x, y;
    };
    
    Block *blocks = nullptr;
    int width, height;

    std::vector<BreakingBlock> breaking_blocks;
    std::vector<BlockType*> block_types;
    std::vector<Tool*> tool_types;
    
    Block* getBlock(int x, int y);
public:
    Blocks();
    void create(int width, int height);

    BlockType air;
    Tool hand;
    
    BlockType* getBlockType(int x, int y);
    void setBlockType(int x, int y, BlockType* type);
    void setBlockTypeSilently(int x, int y, BlockType* type);
    
    int getBreakProgress(int x, int y);
    int getBreakStage(int x, int y);
    void startBreakingBlock(int x, int y);
    void stopBreakingBlock(int x, int y);
    void updateBreakingBlocks(int frame_length);
    
    void breakBlock(int x, int y);
    
    int getWidth() const;
    int getHeight() const;
    
    std::vector<char> toSerial();
    void fromSerial(const std::vector<char>& serial);
    
    void registerNewBlockType(BlockType* block_type);
    BlockType* getBlockTypeById(int block_id);
    BlockType* getBlockTypeByName(const std::string& name);
    int getNumBlockTypes();
    
    void registerNewToolType(Tool* tool);
    Tool* getToolTypeByName(const std::string& name);
    
    EventSender<BlockChangeEvent> block_change_event;
    EventSender<BlockBreakEvent> block_break_event;
    EventSender<BlockStartedBreakingEvent> block_started_breaking_event;
    EventSender<BlockStoppedBreakingEvent> block_stopped_breaking_event;
    
    ~Blocks();
};
