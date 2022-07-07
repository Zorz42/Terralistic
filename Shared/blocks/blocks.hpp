#pragma once
#include "events.hpp"
#include "graphics.hpp"

#define BLOCK_WIDTH 8
#define UNBREAKABLE -1
#define CHUNK_SIZE 16
#define RANDOM_TICK_SPEED 10

class BlockChangeEvent {
public:
    BlockChangeEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class BlockRandomTickEvent{
public:
    BlockRandomTickEvent(int x, int y): x(x), y(y){}
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

class BlockUpdateEvent {
public:
    BlockUpdateEvent(int x, int y) : x(x), y(y) {}
    int x, y;
};

class Tool {
public:
    Tool(const std::string& name) : name(name) {}
    std::string name;
};

struct DefaultData{
    virtual ~DefaultData(){}
    virtual void save(std::vector<char>& data, unsigned long& index){}
    virtual void load(const char*& iter){}
    virtual int getSavedSize(){return 0;}
};

struct dataDeliverer;

class Blocks;

class BlockType {
public:
    BlockType(std::string name);
    Tool* effective_tool;
    int required_tool_power;
    bool ghost, transparent;
    std::string name;
    std::vector<BlockType*> connects_to;
    int break_time;
    int light_emission_r, light_emission_g, light_emission_b;
    int id;
    int width = 0, height = 0;
    bool can_update_states;
    int block_data_index = 0;
    
    virtual int updateState(Blocks* blocks, int x, int y);
};

class Blocks {
    class Block {
    public:
        Block() : id(/*air*/0), x_from_main(0), y_from_main(0) {}
        int id:8;
        int x_from_main:8, y_from_main:8;
        DefaultData* additional_block_data;
    };
    
    class BreakingBlock {
    public:
        int break_progress = 0;
        bool is_breaking = true;
        int x, y;
    };
    
    class BlockChunk {
    public:
        int breaking_blocks_count = 0;
    };
    
    Block *blocks = nullptr;
    BlockChunk *chunks = nullptr;
    int width, height;
    dataDeliverer* data_deliverer;
    std::vector<BreakingBlock> breaking_blocks;
    std::vector<BlockType*> block_types;
    std::vector<Tool*> tool_types;
    
    Block* getBlock(int x, int y);
    BlockChunk* getChunk(int x, int y);
public:
    Blocks();
    void create(int width, int height);

    BlockType air;
    Tool hand;
    
    BlockType* getBlockType(int x, int y);
    void setBlockType(int x, int y, BlockType* type, int x_from_main=0, int y_from_main=0);
    void setBlockTypeSilently(int x, int y, BlockType* type);
    int getBlockXFromMain(int x, int y);
    int getBlockYFromMain(int x, int y);
    DefaultData* getBlockData(int x, int y);
    
    int getBreakProgress(int x, int y);
    int getBreakStage(int x, int y);
    void startBreakingBlock(int x, int y);
    void stopBreakingBlock(int x, int y);
    void updateBreakingBlocks(int frame_length);
    int getChunkBreakingBlocksCount(int x, int y);
    
    void breakBlock(int x, int y);
    
    int getWidth() const;
    int getHeight() const;
    
    std::vector<char> toSerial();
    void fromSerial(const std::vector<char>& serial);
    
    void registerNewBlockType(BlockType* block_type);
    BlockType* getBlockTypeById(int block_id);
    BlockType* getBlockTypeByName(const std::string& name);
    int getNumBlockTypes();
    dataDeliverer* getDataDeliverer() const{return data_deliverer;};
    void setDataDeliverer(dataDeliverer* c_data_deliverer) {data_deliverer = c_data_deliverer;};

    void registerNewToolType(Tool* tool);
    Tool* getToolTypeByName(const std::string& name);
    
    bool updateStateSide(int x, int y, int side_x, int side_y);
    
    EventSender<BlockChangeEvent> block_change_event;
    EventSender<BlockBreakEvent> block_break_event;
    EventSender<BlockStartedBreakingEvent> block_started_breaking_event;
    EventSender<BlockStoppedBreakingEvent> block_stopped_breaking_event;
    EventSender<BlockUpdateEvent> block_update_event;
    
    ~Blocks();
};
