#ifndef blocks_hpp
#define blocks_hpp

#include <string>
#include "properties.hpp"
#include "events.hpp"
#include "biomes.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

enum class FlowDirection {NONE, LEFT, RIGHT, BOTH = LEFT | RIGHT};

class Blocks;

struct MapBlock {
    MapBlock(BlockType block_type=BlockType::AIR, LiquidType liquid_type=LiquidType::EMPTY) : block_type(block_type), liquid_type(liquid_type) {}

    BlockType block_type;
    unsigned short break_progress = 0;
    unsigned char break_stage = 0;
    
    bool light_source = false, update_light = true, has_changed_light = false;
    unsigned char light_level = 0;
    
    LiquidType liquid_type = LiquidType::EMPTY;
    unsigned char liquid_level = 0;
    unsigned int when_to_update_liquid = 1;
    FlowDirection flow_direction = FlowDirection::NONE;
};

class Block {
    MapBlock* block_data = nullptr;
    unsigned short x = 0, y = 0;
    Blocks* parent_map{};

    void updateNeighbors();
public:
    Block(unsigned short x, unsigned short y, MapBlock* block_data, Blocks* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
    Block() = default;

    void update();
    inline bool refersToABlock() { return block_data != nullptr; }
    [[nodiscard]] inline unsigned short getX() const { return x; }
    [[nodiscard]] inline unsigned short getY() const { return y; }
    void breakBlock();
    
    void setTypeWithoutProcessing(BlockType block_type);
    void setType(BlockType block_type);
    void setBreakProgress(unsigned short ms);
    inline unsigned short getBreakProgress() { return block_data->break_progress; }
    inline unsigned char getBreakStage() { return block_data->break_stage; }
    inline BlockType getBlockType() { return block_data->block_type; }
    const BlockInfo& getUniqueBlock();
    
    void setTypeWithoutProcessing(LiquidType liquid_type);
    void setType(LiquidType liquid_type);
    bool extracted(Block &block_under);
    
    void liquidUpdate();
    inline LiquidType getLiquidType() { return block_data->liquid_type; }
    bool canUpdateLiquid();
    void setLiquidLevel(unsigned char level);
    void setLiquidLevelWithoutProcessing(unsigned char level);
    inline unsigned char getLiquidLevel() { return block_data->liquid_level; }
    inline FlowDirection getFlowDirection() { return block_data->flow_direction; }
    inline void setFlowDirection(FlowDirection flow_direction) { block_data->flow_direction = flow_direction; }
    const LiquidInfo& getUniqueLiquid();
    void scheduleLiquidUpdate();
    
    void lightUpdate();
    void setLightSource(unsigned char power);
    inline bool isLightSource() { return block_data->light_source; }
    void removeLightSource();
    void setLightLevel(unsigned char light_level);
    inline unsigned char getLightLevel() { return block_data->light_level; }
    inline void scheduleLightUpdate() { block_data->update_light = true; }
    inline bool hasScheduledLightUpdate() { return block_data->update_light; }
    inline bool hasLightChanged() { return block_data->has_changed_light; }
    inline void markLightUnchanged() { block_data->has_changed_light = false; }
};

class Blocks {
    MapBlock *blocks = nullptr;
    unsigned short width, height;
    
public:
    Block getBlock(unsigned short x, unsigned short y);
    
    void createWorld(unsigned short width, unsigned short height);
    
    void setNaturalLight();
    void removeNaturalLight(unsigned short x);
    void setNaturalLight(unsigned short x);
    
    Biome *biomes = nullptr;
    
    int getSpawnX();
    int getSpawnY();
    
    void saveTo(std::string path);
    void loadFrom(std::string path);
    
    inline unsigned short getHeight() { return height; }
    inline unsigned short getWidth() { return width; }
    
    ~Blocks();
};



class ServerBlockChangeEvent : public Event<ServerBlockChangeEvent> {
public:
    ServerBlockChangeEvent(Block block, BlockType type) : block(block), type(type) {}
    Block block;
    BlockType type;
};

class ServerBlockBreakEvent : public Event<ServerBlockBreakEvent> {
public:
    ServerBlockBreakEvent(Block block) : block(block) {}
    Block block;
};

class ServerBlockUpdateEvent : public Event<ServerBlockUpdateEvent> {
public:
    ServerBlockUpdateEvent(Block block) : block(block) {}
    Block block;
};

class ServerBlockBreakStageChangeEvent : public Event<ServerBlockBreakStageChangeEvent> {
public:
    ServerBlockBreakStageChangeEvent(Block block, unsigned char break_stage) : block(block), break_stage(break_stage) {}
    Block block;
    unsigned char break_stage;
};

class ServerLightChangeEvent : public Event<ServerLightChangeEvent> {
public:
    ServerLightChangeEvent(Block block, unsigned char light_level) : block(block), light_level(light_level) {}
    Block block;
    unsigned char light_level;
};

class ServerLiquidChangeEvent : public Event<ServerLiquidChangeEvent> {
public:
    ServerLiquidChangeEvent(Block block, LiquidType liquid_type, unsigned char liquid_level) : block(block), liquid_type(liquid_type), liquid_level(liquid_level) {}
    Block block;
    LiquidType liquid_type;
    unsigned char liquid_level;
};

#endif /* blocks_hpp */
