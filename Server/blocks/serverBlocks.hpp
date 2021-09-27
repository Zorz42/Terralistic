#ifndef serverBlocks_hpp
#define serverBlocks_hpp

#include <string>
#include "properties.hpp"
#include "events.hpp"
#include "biomes.hpp"

#define BLOCK_WIDTH 8
#define MAX_LIGHT 100

enum class FlowDirection {NONE, LEFT, RIGHT};

class ServerBlocks;

struct ServerMapBlock {
    explicit ServerMapBlock(BlockType block_type=BlockType::AIR, LiquidType liquid_type=LiquidType::EMPTY) : block_type(block_type), liquid_type(liquid_type), break_stage(0), flow_direction(FlowDirection::NONE) {}

    BlockType block_type:8;
    unsigned short break_progress = 0;
    unsigned break_stage:4;
    
    bool light_source = false, update_light = true;
    unsigned char light_level = 0;
    
    LiquidType liquid_type:8;
    FlowDirection flow_direction:4;
    unsigned char liquid_level = 0;
    unsigned int when_to_update_liquid = 1;
};

class ServerBlock {
    ServerMapBlock* block_data = nullptr;
    unsigned short x = 0, y = 0;
    ServerBlocks* blocks = nullptr;

    void updateNeighbors();
public:
    ServerBlock(unsigned short x, unsigned short y, ServerMapBlock* block_data, ServerBlocks* blocks) : x(x), y(y), block_data(block_data), blocks(blocks) {}
    ServerBlock() = default;

    void update();
    bool refersToABlock() { return block_data != nullptr; }
    unsigned short getX() const { return x; }
    unsigned short getY() const { return y; }
    void breakBlock();
    
    void setType(BlockType block_type);
    void setBreakProgress(unsigned short ms);
    unsigned short getBreakProgress() { return block_data->break_progress; }
    unsigned char getBreakStage() { return block_data->break_stage; }
    BlockType getBlockType() { return block_data->block_type; }
    const BlockInfo& getBlockInfo();
    
    void setType(LiquidType liquid_type);
    
    void liquidUpdate();
    LiquidType getLiquidType() { return block_data->liquid_type; }
    bool canUpdateLiquid();
    void setLiquidLevel(unsigned char level);
    unsigned char getLiquidLevel() { return block_data->liquid_level; }
    FlowDirection getFlowDirection() { return block_data->flow_direction; }
    void setFlowDirection(FlowDirection flow_direction) { block_data->flow_direction = flow_direction; }
    const LiquidInfo& getUniqueLiquid();
    void scheduleLiquidUpdate();
    
    void lightUpdate();
    void setLightSource(unsigned char power);
    void setLightLevel(unsigned char light_level);
    unsigned char getLightLevel() { return block_data->light_level; }
    void scheduleLightUpdate() { block_data->update_light = true; }
    bool hasScheduledLightUpdate() { return block_data->update_light; }
};

class ServerBlockChangeEvent {
public:
    ServerBlockChangeEvent(ServerBlock block, BlockType type) : block(block), type(type) {}
    ServerBlock block;
    BlockType type;
};

class ServerBlockBreakEvent {
public:
    explicit ServerBlockBreakEvent(ServerBlock block) : block(block) {}
    ServerBlock block;
};

class ServerBlockUpdateEvent {
public:
    explicit ServerBlockUpdateEvent(ServerBlock block) : block(block) {}
    ServerBlock block;
};

class ServerBlockBreakStageChangeEvent {
public:
    ServerBlockBreakStageChangeEvent(ServerBlock block, unsigned char break_stage) : block(block), break_stage(break_stage) {}
    ServerBlock block;
    unsigned char break_stage;
};

class ServerLiquidChangeEvent {
public:
    ServerLiquidChangeEvent(ServerBlock block, LiquidType liquid_type, unsigned char liquid_level) : block(block), liquid_type(liquid_type), liquid_level(liquid_level) {}
    ServerBlock block;
    LiquidType liquid_type;
    unsigned char liquid_level;
};

class ServerLightChangeEvent {
public:
    ServerLightChangeEvent(ServerBlock block) : block(block) {}
    ServerBlock block;
};

class ServerBlocks {
    ServerMapBlock *blocks = nullptr;
    unsigned short width = 0, height = 0;
    ServerMapBlock* getMapBlock(unsigned short x, unsigned short y);
    
public:
    ServerBlock getBlock(unsigned short x, unsigned short y);
    
    void createWorld(unsigned short width, unsigned short height);
    
    Biome *biomes = nullptr;
    
    int getSpawnX() const;
    int getSpawnY();
    
    void serialize(std::vector<char>& serial);
    char* loadFromSerial(char* iter);
    
    std::vector<char> toData();
    
    unsigned short getHeight() const { return height; }
    unsigned short getWidth() const { return width; }
    
    BlockType getTypeDirectly(unsigned short x, unsigned short y);
    unsigned char getLiquidLevelDirectly(unsigned short x, unsigned short y);
    LiquidType getLiquidTypeDirectly(unsigned short x, unsigned short y);
    
    void setTypeDirectly(unsigned short x, unsigned short y, BlockType type);
    void setTypeDirectly(unsigned short x, unsigned short y, LiquidType type);
    void setLiquidLevelDirectly(unsigned short x, unsigned short y, unsigned char level);
    void setLightLevelDirectly(unsigned short x, unsigned short y, unsigned char level);
    void setLightSourceDirectly(unsigned short x, unsigned short y, bool source);
    
    EventSender<ServerBlockChangeEvent> block_change_event;
    EventSender<ServerBlockBreakEvent> block_break_event;
    EventSender<ServerBlockUpdateEvent> block_update_event;
    EventSender<ServerBlockBreakStageChangeEvent> block_break_stage_change_event;
    EventSender<ServerLiquidChangeEvent> liquid_change_event;
    EventSender<ServerLightChangeEvent> light_change_event;
    
    ~ServerBlocks();
};


#endif
