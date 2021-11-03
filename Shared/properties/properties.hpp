#pragma once

#define UNBREAKABLE -1

#include <string>
#include <utility>
#include <vector>
#include <map>
#include "graphics.hpp"

enum class BlockTypeOld {NOTHING = -1, AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, SAND, SNOWY_GRASS_BLOCK, SNOW_BLOCK, ICE, IRON_ORE, COPPER_ORE, NUM_BLOCKS};
enum class ItemTypeOld {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS, IRON_ORE, COPPER_ORE, NUM_ITEMS};
enum class LiquidTypeOld {EMPTY, WATER, NUM_LIQUIDS};

struct BlockInfoOld {
    BlockInfoOld() = default;
    BlockInfoOld(std::string name, bool ghost, bool transparent, short break_time, ItemTypeOld drop, std::vector<BlockTypeOld> connects_to, gfx::Color color);
    
    bool ghost, transparent;
    std::string name;
    std::vector<BlockTypeOld> connects_to;
    short break_time;
    ItemTypeOld drop;
    gfx::Color color;
};

struct ItemInfoOld {
    ItemInfoOld() = default;
    ItemInfoOld(std::string name, unsigned short stack_size, BlockTypeOld places);
    
    std::string name;
    unsigned short stack_size;
    BlockTypeOld places;
};

struct LiquidInfoOld {
    LiquidInfoOld() = default;
    LiquidInfoOld(std::string name, unsigned short flow_time, float speed_multiplier, gfx::Color color);
    
    std::string name;
    unsigned short flow_time;
    float speed_multiplier;
    gfx::Color color;
};

struct RecipeOld {
    std::map<ItemTypeOld, unsigned short> ingredients;
    unsigned short result_stack;
    ItemTypeOld result_type;
};

void initProperties();

BlockTypeOld getBlockTypeByNameOld(const std::string& name);
ItemTypeOld getItemTypeByNameOld(const std::string& name);
const BlockInfoOld& getBlockInfoOld(BlockTypeOld type);
const ItemInfoOld& getItemInfoOld(ItemTypeOld type);
const LiquidInfoOld& getLiquidInfoOld(LiquidTypeOld type);
const std::vector<RecipeOld>& getRecipesOld();
unsigned short getRecipeIndexOld(const RecipeOld* recipe);

