#ifndef blocks_hpp
#define blocks_hpp

#include <string>
#include <chrono>
#include "properties.hpp"
#include "events.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100

enum class FlowDirection {NONE, LEFT, RIGHT, BOTH = LEFT | RIGHT};

class Blocks;

struct MapBlock {
    MapBlock(BlockType block_id=BlockType::AIR, LiquidType liquid_id=LiquidType::EMPTY) : block_id(block_id), liquid_id(liquid_id) {}

    BlockType block_id;
    unsigned short break_progress = 0;
    unsigned char break_stage = 0;
    
    bool light_source = false, update_light = true;
    unsigned char light_level = 0;
    
    LiquidType liquid_id = LiquidType::EMPTY;
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
    
    void setTypeWithoutProcessing(BlockType block_id);
    void setType(BlockType block_id);
    void setBreakProgress(unsigned short ms);
    inline unsigned short getBreakProgress() { return block_data->break_progress; }
    inline unsigned char getBreakStage() { return block_data->break_stage; }
    inline BlockType getBlockType() { return block_data->block_id; }
    const BlockInfo& getUniqueBlock();
    
    void setTypeWithoutProcessing(LiquidType liquid_id);
    void setType(LiquidType liquid_id);
    void liquidUpdate();
    inline LiquidType getLiquidType() { return block_data->liquid_id; }
    inline bool canUpdateLiquid() { return block_data->when_to_update_liquid != 0 && (unsigned int)std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count() > block_data->when_to_update_liquid; }
    void setLiquidLevel(unsigned char level);
    inline unsigned char getLiquidLevel() { return block_data->liquid_level; }
    inline FlowDirection getFlowDirection() { return block_data->flow_direction; }
    inline void setFlowDirection(FlowDirection flow_direction) { block_data->flow_direction = flow_direction; }
    const LiquidInfo& getUniqueLiquid();
    
    void lightUpdate();
    void setLightSource(unsigned char power);
    inline bool isLightSource() { return block_data->light_source; }
    void removeLightSource();
    void setLightLevel(unsigned char light_level);
    inline unsigned char getLightLevel() { return block_data->light_level; }
    inline void scheduleLightUpdate() { block_data->update_light = true; }
    inline bool hasScheduledLightUpdate() { return block_data->update_light; }
};

class ServerBlockChangeEvent : public Event<ServerBlockChangeEvent> {
public:
    ServerBlockChangeEvent(Block block, BlockType type) : block(block), type(type) {}
    Block block;
    BlockType type;
};

class ServerBlockBreakStageChangeEvent : public Event<ServerBlockBreakStageChangeEvent> {
public:
    ServerBlockBreakStageChangeEvent(Block block, unsigned char break_stage) : block(block), break_stage(break_stage) {}
    Block block;
    unsigned char break_stage;
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

#endif /* blocks_hpp */
