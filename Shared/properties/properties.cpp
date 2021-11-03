#include <cassert>
#include <utility>
#include "properties.hpp"

static BlockInfoOld block_infos[(int)BlockTypeOld::NUM_BLOCKS];
static ItemInfoOld item_infos[(int)ItemTypeOld::NUM_ITEMS];
static LiquidInfoOld liquid_infos[(int)LiquidTypeOld::NUM_LIQUIDS];
static std::vector<RecipeOld> recipes;

BlockInfoOld::BlockInfoOld(std::string name, bool ghost, bool transparent, short break_time, ItemTypeOld drop, std::vector<BlockTypeOld> connects_to, gfx::Color color) : ghost(ghost), transparent(transparent), name(std::move(name)), break_time(break_time), drop(drop), connects_to(std::move(connects_to)), color(color) {}

ItemInfoOld::ItemInfoOld(std::string  name, unsigned short stack_size, BlockTypeOld places) : name(std::move(name)), stack_size(stack_size), places(places) {}

LiquidInfoOld::LiquidInfoOld(std::string name, unsigned short flow_time, float speed_multiplier, gfx::Color color) : name(std::move(name)), flow_time(flow_time), speed_multiplier(speed_multiplier), color(color) {}

const BlockInfoOld& getBlockInfoOld(BlockTypeOld type) {
    return block_infos[(int)type];
}

const ItemInfoOld& getItemInfoOld(ItemTypeOld type) {
    assert((int)type >= 0 && type < ItemTypeOld::NUM_ITEMS);
    return item_infos[(int)type];
}

const LiquidInfoOld& getLiquidInfoOld(LiquidTypeOld type) {
    return liquid_infos[(int)type];
}

const std::vector<RecipeOld>& getRecipesOld() {
    return recipes;
}

unsigned short getRecipeIndexOld(const RecipeOld* recipe) {
    return recipe - &recipes[0];
}

void initProperties() {
    // unique_blocks
    block_infos[(int)BlockTypeOld::AIR] =
    BlockInfoOld("air",               /*ghost*/true,  /*transparent*/true,  /*break_time*/UNBREAKABLE, /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/ {                                                              }, /*color*/{0, 0, 0, 0});
    block_infos[(int)BlockTypeOld::DIRT] =
    BlockInfoOld("dirt",              /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/ItemTypeOld::DIRT,        /*connects_to*/{BlockTypeOld::GRASS_BLOCK, BlockTypeOld::SNOWY_GRASS_BLOCK           }, /*color*/{115, 77, 38});
    block_infos[(int)BlockTypeOld::STONE_BLOCK] =
    BlockInfoOld("stone_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/ItemTypeOld::STONE_BLOCK, /*connects_to*/{BlockTypeOld::SNOWY_GRASS_BLOCK                                   }, /*color*/{128, 128, 128});
    block_infos[(int)BlockTypeOld::GRASS_BLOCK] =
    BlockInfoOld("grass_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/{BlockTypeOld::DIRT, BlockTypeOld::SNOWY_GRASS_BLOCK                  }, /*color*/{0, 153, 0});
    block_infos[(int)BlockTypeOld::STONE] =
    BlockInfoOld("stone",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1500,        /*drops*/ItemTypeOld::STONE,       /*connects_to*/{                                                               }, /*color*/{128, 128, 128});
    block_infos[(int)BlockTypeOld::WOOD] =
    BlockInfoOld("wood",              /*ghost*/true,  /*transparent*/false, /*break_time*/1000,        /*drops*/ItemTypeOld::WOOD_PLANKS, /*connects_to*/{BlockTypeOld::GRASS_BLOCK, BlockTypeOld::LEAVES                      }, /*color*/{128, 85, 0});
    block_infos[(int)BlockTypeOld::LEAVES] =
    BlockInfoOld("leaves",            /*ghost*/true,  /*transparent*/false, /*break_time*/UNBREAKABLE, /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/{                                                               }, /*color*/{0, 179, 0});
    block_infos[(int)BlockTypeOld::SAND] =
    BlockInfoOld("sand",              /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/{BlockTypeOld::DIRT, BlockTypeOld::GRASS_BLOCK, BlockTypeOld::STONE_BLOCK}, /*color*/{210, 170, 109});
    block_infos[(int)BlockTypeOld::SNOWY_GRASS_BLOCK] =
    BlockInfoOld("snowy_grass_block", /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/{BlockTypeOld::DIRT, BlockTypeOld::GRASS_BLOCK, BlockTypeOld::STONE_BLOCK}, /*color*/{217, 217, 217});
    block_infos[(int)BlockTypeOld::SNOW_BLOCK] =
    BlockInfoOld("snow_block",        /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/{BlockTypeOld::SNOWY_GRASS_BLOCK, BlockTypeOld::ICE                   }, /*color*/{242, 242, 242});
    block_infos[(int)BlockTypeOld::ICE] =
    BlockInfoOld("ice_block",         /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*drops*/ItemTypeOld::NOTHING,     /*connects_to*/{BlockTypeOld::SNOW_BLOCK                                          }, /*color*/{179, 217, 255});
    block_infos[(int)BlockTypeOld::IRON_ORE] =
    BlockInfoOld("iron_ore",          /*ghost*/false, /*transparent*/false, /*break_time*/1500,        /*drops*/ItemTypeOld::IRON_ORE,    /*connects_to*/{                                                               }, /*color*/{160, 160, 160});
    block_infos[(int)BlockTypeOld::COPPER_ORE] =
    BlockInfoOld("copper_ore",        /*ghost*/false, /*transparent*/false, /*break_time*/1500,        /*drops*/ItemTypeOld::COPPER_ORE,  /*connects_to*/{                                                               }, /*color*/{200, 109, 61});
    
    // unique_items
    item_infos[(int)ItemTypeOld::NOTHING] =     ItemInfoOld(/*name*/"nothing",     /*max_stack*/0,  /*places*/BlockTypeOld::AIR        );
    item_infos[(int)ItemTypeOld::STONE] =       ItemInfoOld(/*name*/"stone",       /*max_stack*/99, /*places*/BlockTypeOld::STONE      );
    item_infos[(int)ItemTypeOld::DIRT] =        ItemInfoOld(/*name*/"dirt",        /*max_stack*/99, /*places*/BlockTypeOld::DIRT       );
    item_infos[(int)ItemTypeOld::STONE_BLOCK] = ItemInfoOld(/*name*/"stone_block", /*max_stack*/99, /*places*/BlockTypeOld::STONE_BLOCK);
    item_infos[(int)ItemTypeOld::WOOD_PLANKS] = ItemInfoOld(/*name*/"wood_planks", /*max_stack*/99, /*places*/BlockTypeOld::AIR        );
    item_infos[(int)ItemTypeOld::IRON_ORE] =    ItemInfoOld(/*name*/"iron_ore",    /*max_stack*/99, /*places*/BlockTypeOld::IRON_ORE   );
    item_infos[(int)ItemTypeOld::COPPER_ORE] =  ItemInfoOld(/*name*/"copper_ore",  /*max_stack*/99, /*places*/BlockTypeOld::COPPER_ORE );
    
    // unqiue_liquids
    liquid_infos[(int)LiquidTypeOld::EMPTY] = LiquidInfoOld(/*name*/"empty", /*flow_time*/0,   /*speed_multiplier*/1  ,/*color*/{0, 0, 0, 0});
    liquid_infos[(int)LiquidTypeOld::WATER] = LiquidInfoOld(/*name*/"water", /*flow_time*/100, /*speed_multiplier*/0.5,/*color*/{0, 92, 230, 150});
    
    // recipes
    {
        RecipeOld recipe;
        recipe.result_type = ItemTypeOld::DIRT;
        recipe.result_stack = 2;
        recipe.ingredients[ItemTypeOld::STONE_BLOCK] = 1;
        recipes.push_back(recipe);
    }
    
    {
        RecipeOld recipe;
        recipe.result_type = ItemTypeOld::DIRT;
        recipe.result_stack = 1;
        recipe.ingredients[ItemTypeOld::WOOD_PLANKS] = 1;
        recipes.push_back(recipe);
    }
    
    {
        RecipeOld recipe;
        recipe.result_type = ItemTypeOld::STONE_BLOCK;
        recipe.result_stack = 1;
        recipe.ingredients[ItemTypeOld::STONE] = 2;
        recipe.ingredients[ItemTypeOld::DIRT] = 2;
        recipes.push_back(recipe);
    }
}

BlockTypeOld getBlockTypeByNameOld(const std::string& name) {
    for(int i = 0; i < (int)BlockTypeOld::NUM_BLOCKS; i++)
        if(getBlockInfoOld((BlockTypeOld)i).name == name)
            return (BlockTypeOld)i;
    return BlockTypeOld::NOTHING;
}

ItemTypeOld getItemTypeByNameOld(const std::string& name){
    for(int i = 0; i < (int)ItemTypeOld::NUM_ITEMS; i++)
        if(getItemInfoOld((ItemTypeOld)i).name == name)
            return (ItemTypeOld)i;
    return ItemTypeOld::NOTHING;
}
