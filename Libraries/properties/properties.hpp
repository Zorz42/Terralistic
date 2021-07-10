#ifndef properties_hpp
#define properties_hpp

#define UNBREAKABLE -1

#include <string>
#include <vector>

enum class blockType {NOTHING = -1, AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, SAND, SNOWY_GRASS_BLOCK, SNOW_BLOCK, ICE, NUM_BLOCKS};
enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS, NUM_ITEMS};
enum class liquidType {EMPTY, WATER, NUM_LIQUIDS};
enum class flowDirection {NONE, LEFT, RIGHT, BOTH = LEFT | RIGHT};
enum class biome {NO_BIOME, ICY_SEAS, SNOWY_TUNDRA, COLD_HILLS, SNOWY_MOUNTAINS, SEA, PLAINS, FOREST, MOUNTAINS, WARM_OCEAN, DESERT, SAVANA, SAVANA_MOUNTAINS, NUM_BIOMES};

struct BlockInfo {
    BlockInfo() = default;
    BlockInfo(std::string name, bool ghost, bool transparent, short break_time, itemType drop, std::vector<blockType> connects_to);
    
    bool ghost, transparent;
    std::string name;
    std::vector<blockType> connects_to;
    short break_time;
    itemType drop;
};

struct ItemInfo {
    ItemInfo() = default;
    ItemInfo(std::string  name, unsigned short stack_size, blockType places);
    
    std::string name;
    unsigned short stack_size;
    blockType places;
};

struct LiquidInfo {
    LiquidInfo() = default;
    LiquidInfo(std::string name, unsigned short flow_time, float speed_multiplier);
    
    std::string name;
    unsigned short flow_time;
    float speed_multiplier;
};

void initProperties();
const BlockInfo& getBlockInfo(blockType type);
const ItemInfo& getItemInfo(itemType type);
const LiquidInfo& getLiquidInfo(liquidType type);

#endif /* properties_hpp */
