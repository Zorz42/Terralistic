#include <cassert>
#include <utility>
#include "properties.hpp"

static ItemInfo item_infos[(int)ItemType::NUM_ITEMS];
static LiquidInfo liquid_infos[(int)LiquidType::NUM_LIQUIDS];
static std::vector<Recipe> recipes;

BlockInfo::BlockInfo(std::string name, bool ghost, bool transparent, bool only_on_floor, short break_time, ItemType drop, std::vector<BlockType> connects_to) : ghost(ghost), transparent(transparent), only_on_floor(only_on_floor), name(std::move(name)), break_time(break_time), drop(drop), connects_to(std::move(connects_to)) {}

ItemInfo::ItemInfo(std::string  name, unsigned short stack_size, BlockType places) : name(std::move(name)), stack_size(stack_size), places(places) {}

LiquidInfo::LiquidInfo(std::string name, unsigned short flow_time, float speed_multiplier) : name(std::move(name)), flow_time(flow_time), speed_multiplier(speed_multiplier) {}

const BlockInfo& getBlockInfo(BlockType type) {
    return block_infos[(int)type];
}

const ItemInfo& getItemInfo(ItemType type) {
    assert((int)type >= 0 && type < ItemType::NUM_ITEMS);
    return item_infos[(int)type];
}

const LiquidInfo& getLiquidInfo(LiquidType type) {
    return liquid_infos[(int)type];
}

const std::vector<Recipe>& getRecipes() {
    return recipes;
}

unsigned short getRecipeIndex(const Recipe* recipe) {
    return recipe - &recipes[0];
}

void initProperties() {
    // unique_blocks
    block_infos[(int)BlockType::AIR] =
    BlockInfo("air",               /*ghost*/true,  /*transparent*/true,  /*only_on_floor*/false, /*break_time*/UNBREAKABLE, /*drops*/ItemType::NOTHING,     /*connects_to*/ {                                                              });
    block_infos[(int)BlockType::DIRT] =
    BlockInfo("dirt",              /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/1000,        /*drops*/ItemType::DIRT,        /*connects_to*/{BlockType::GRASS_BLOCK, BlockType::SNOWY_GRASS_BLOCK           });
    block_infos[(int)BlockType::STONE_BLOCK] =
    BlockInfo("stone_block",       /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/1000,        /*drops*/ItemType::STONE_BLOCK, /*connects_to*/{BlockType::SNOWY_GRASS_BLOCK                                   });
    block_infos[(int)BlockType::GRASS_BLOCK] =
    BlockInfo("grass_block",       /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/1000,        /*drops*/ItemType::NOTHING,     /*connects_to*/{BlockType::DIRT, BlockType::SNOWY_GRASS_BLOCK                  });
    block_infos[(int)BlockType::STONE] =
    BlockInfo("stone",             /*ghost*/true,  /*transparent*/true,  /*only_on_floor*/true,  /*break_time*/1500,        /*drops*/ItemType::STONE,       /*connects_to*/{                                                               });
    block_infos[(int)BlockType::WOOD] =
    BlockInfo("wood",              /*ghost*/true,  /*transparent*/false, /*only_on_floor*/false, /*break_time*/1000,        /*drops*/ItemType::WOOD_PLANKS, /*connects_to*/{BlockType::GRASS_BLOCK, BlockType::LEAVES                      });
    block_infos[(int)BlockType::LEAVES] =
    BlockInfo("leaves",            /*ghost*/true,  /*transparent*/false, /*only_on_floor*/false, /*break_time*/UNBREAKABLE, /*drops*/ItemType::NOTHING,     /*connects_to*/{                                                               });
    block_infos[(int)BlockType::SAND] =
    BlockInfo("sand",              /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/500,         /*drops*/ItemType::NOTHING,     /*connects_to*/{BlockType::DIRT, BlockType::GRASS_BLOCK, BlockType::STONE_BLOCK});
    block_infos[(int)BlockType::SNOWY_GRASS_BLOCK] =
    BlockInfo("snowy_grass_block", /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/1000,        /*drops*/ItemType::NOTHING,     /*connects_to*/{BlockType::DIRT, BlockType::GRASS_BLOCK, BlockType::STONE_BLOCK});
    block_infos[(int)BlockType::SNOW_BLOCK] =
    BlockInfo("snow_block",        /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/500,         /*drops*/ItemType::NOTHING,     /*connects_to*/{BlockType::SNOWY_GRASS_BLOCK, BlockType::ICE                   });
    block_infos[(int)BlockType::ICE] =
    BlockInfo("ice_block",         /*ghost*/false, /*transparent*/false, /*only_on_floor*/false, /*break_time*/500,         /*drops*/ItemType::NOTHING,     /*connects_to*/{BlockType::SNOW_BLOCK                                          });
    
    // unique_items
    item_infos[(int)ItemType::NOTHING] =     ItemInfo(/*name*/"nothing",     /*max_stack*/0,  /*places*/BlockType::AIR        );
    item_infos[(int)ItemType::STONE] =       ItemInfo(/*name*/"stone",       /*max_stack*/99, /*places*/BlockType::STONE      );
    item_infos[(int)ItemType::DIRT] =        ItemInfo(/*name*/"dirt",        /*max_stack*/99, /*places*/BlockType::DIRT       );
    item_infos[(int)ItemType::STONE_BLOCK] = ItemInfo(/*name*/"stone_block", /*max_stack*/99, /*places*/BlockType::STONE_BLOCK);
    item_infos[(int)ItemType::WOOD_PLANKS] = ItemInfo(/*name*/"wood_planks", /*max_stack*/99, /*places*/BlockType::AIR        );
    
    // unqiue_liquids
    liquid_infos[(int)LiquidType::EMPTY] = LiquidInfo(/*name*/"empty", /*flow_time*/0,   /*speed_multiplier*/1  );
    liquid_infos[(int)LiquidType::WATER] = LiquidInfo(/*name*/"water", /*flow_time*/100, /*speed_multiplier*/0.5);
    
    // recipes
    recipes = {
        Recipe({{ItemType::STONE_BLOCK, 1}},                {ItemType::DIRT, 2}),
        Recipe({{ItemType::WOOD_PLANKS, 4}},                {ItemType::DIRT, 1}),
        Recipe({{ItemType::STONE, 2}, {ItemType::DIRT, 2}}, {ItemType::STONE_BLOCK, 1}),
    };
}

BlockType getBlockTypeByName(const std::string& name) {
    for(int i = 0; i < (int)BlockType::NUM_BLOCKS; i++)
        if(getBlockInfo((BlockType)i).name == name)
            return (BlockType)i;
    return BlockType::NOTHING;
}

ItemType getItemTypeByName(const std::string& name){
    for(int i = 0; i < (int)ItemType::NUM_ITEMS; i++)
        if(getItemInfo((ItemType)i).name == name)
            return (ItemType)i;
    return ItemType::NOTHING;
}
