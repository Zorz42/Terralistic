#pragma once

#define UNBREAKABLE -1

#include <string>
#include <utility>
#include <vector>
#include <map>

enum class BlockType {NOTHING = -1, AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, SAND, SNOWY_GRASS_BLOCK, SNOW_BLOCK, ICE, IRON_ORE, COPPER_ORE, NUM_BLOCKS};
enum class ItemType {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS, IRON_ORE, COPPER_ORE, NUM_ITEMS};
enum class LiquidType {EMPTY, WATER, NUM_LIQUIDS};
enum class BiomeType {NO_BIOME = -1, ICY_SEAS, SNOWY_TUNDRA, COLD_HILLS, SNOWY_MOUNTAINS, SEA, PLAINS, FOREST, MOUNTAINS, WARM_OCEAN, DESERT, SAVANA, SAVANA_MOUNTAINS, NUM_BIOMES};

struct BlockInfo {
    BlockInfo() = default;
    BlockInfo(std::string name, bool ghost, bool transparent, short break_time, ItemType drop, std::vector<BlockType> connects_to);
    
    bool ghost, transparent;
    std::string name;
    std::vector<BlockType> connects_to;
    short break_time;
    ItemType drop;
};

struct ItemInfo {
    ItemInfo() = default;
    ItemInfo(std::string name, unsigned short stack_size, BlockType places);
    
    std::string name;
    unsigned short stack_size;
    BlockType places;
};

struct LiquidInfo {
    LiquidInfo() = default;
    LiquidInfo(std::string name, unsigned short flow_time, float speed_multiplier);
    
    std::string name;
    unsigned short flow_time;
    float speed_multiplier;
};

struct Recipe {
    std::map<ItemType, unsigned short> ingredients;
    unsigned short result_stack;
    ItemType result_type;
};

void initProperties();

BlockType getBlockTypeByName(const std::string& name);
ItemType getItemTypeByName(const std::string& name);
const BlockInfo& getBlockInfo(BlockType type);
const ItemInfo& getItemInfo(ItemType type);
const LiquidInfo& getLiquidInfo(LiquidType type);
const std::vector<Recipe>& getRecipes();
unsigned short getRecipeIndex(const Recipe* recipe);

