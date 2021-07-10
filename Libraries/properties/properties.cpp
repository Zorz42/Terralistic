#include "properties.hpp"
#include "assert.hpp"

static BlockInfo block_infos[(int)blockType::NUM_BLOCKS];
static ItemInfo item_infos[(int)itemType::NUM_ITEMS];
static LiquidInfo liquid_infos[(int)liquidType::NUM_LIQUIDS];

BlockInfo::BlockInfo(std::string name, bool ghost, bool transparent, short break_time, itemType drop, std::vector<blockType> connects_to) : ghost(ghost), transparent(transparent), name(std::move(name)), break_time(break_time), drop(drop), connects_to(connects_to) {}

ItemInfo::ItemInfo(std::string  name, unsigned short stack_size, blockType places) : name(std::move(name)), stack_size(stack_size), places(places) {}

LiquidInfo::LiquidInfo(std::string name, unsigned short flow_time, float speed_multiplier) : name(name), flow_time(flow_time), speed_multiplier(speed_multiplier) {}

const BlockInfo& getBlockInfo(blockType type) {
    ASSERT((int)type >= 0 && type < blockType::NUM_BLOCKS, "block id is not valid")
    return block_infos[(int)type];
}

const ItemInfo& getItemInfo(itemType type) {
    ASSERT((int)type >= 0 && type < itemType::NUM_ITEMS, "item id is not valid")
    return item_infos[(int)type];
}

const LiquidInfo& getLiquidInfo(liquidType type) {
    ASSERT((int)type >= 0 && type < liquidType::NUM_LIQUIDS, "item id is not valid")
    return liquid_infos[(int)type];
}

void initProperties() {
    // unique_blocks
    block_infos[(int)blockType::AIR] =
    BlockInfo("air",               /*ghost*/true,  /*transparent*/true,  /*break_time*/UNBREAKABLE, /*drops*/itemType::NOTHING,     /*connects_to*/ {                                                               });
    block_infos[(int)blockType::DIRT] =
    BlockInfo("dirt",              /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/itemType::DIRT,        /*connects_to*/{blockType::GRASS_BLOCK, blockType::SNOWY_GRASS_BLOCK           });
    block_infos[(int)blockType::STONE_BLOCK] =
    BlockInfo("stone_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/itemType::STONE_BLOCK, /*connects_to*/{blockType::SNOWY_GRASS_BLOCK                                   });
    block_infos[(int)blockType::GRASS_BLOCK] =
    BlockInfo("grass_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/itemType::NOTHING,     /*connects_to*/{blockType::DIRT, blockType::SNOWY_GRASS_BLOCK                  });
    block_infos[(int)blockType::STONE] =
    BlockInfo("stone",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1500,        /*drops*/itemType::STONE,       /*connects_to*/{                                                               });
    block_infos[(int)blockType::WOOD] =
    BlockInfo("wood",              /*ghost*/true,  /*transparent*/false, /*break_time*/1000,        /*drops*/itemType::WOOD_PLANKS, /*connects_to*/{blockType::GRASS_BLOCK, blockType::LEAVES                      });
    block_infos[(int)blockType::LEAVES] =
    BlockInfo("leaves",            /*ghost*/true,  /*transparent*/false, /*break_time*/UNBREAKABLE, /*drops*/itemType::NOTHING,     /*connects_to*/{                                                               });
    block_infos[(int)blockType::SAND] =
    BlockInfo("sand",              /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*drops*/itemType::NOTHING,     /*connects_to*/{blockType::DIRT, blockType::GRASS_BLOCK, blockType::STONE_BLOCK});
    block_infos[(int)blockType::SNOWY_GRASS_BLOCK] =
    BlockInfo("snowy_grass_block", /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*drops*/itemType::NOTHING,     /*connects_to*/{blockType::DIRT, blockType::GRASS_BLOCK, blockType::STONE_BLOCK});
    block_infos[(int)blockType::SNOW_BLOCK] =
    BlockInfo("snow_block",        /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*drops*/itemType::NOTHING,     /*connects_to*/{blockType::SNOWY_GRASS_BLOCK, blockType::ICE                   });
    block_infos[(int)blockType::ICE] =
    BlockInfo("ice_block",         /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*drops*/itemType::NOTHING,     /*connects_to*/{blockType::SNOW_BLOCK                                          });
    
    // unique_items
    item_infos[(int)itemType::NOTHING] =     ItemInfo(/*name*/"nothing",     /*max_stack*/0,  /*places*/blockType::AIR        );
    item_infos[(int)itemType::STONE] =       ItemInfo(/*name*/"stone",       /*max_stack*/99, /*places*/blockType::STONE      );
    item_infos[(int)itemType::DIRT] =        ItemInfo(/*name*/"dirt",        /*max_stack*/99, /*places*/blockType::DIRT       );
    item_infos[(int)itemType::STONE_BLOCK] = ItemInfo(/*name*/"stone_block", /*max_stack*/99, /*places*/blockType::STONE_BLOCK);
    item_infos[(int)itemType::WOOD_PLANKS] = ItemInfo(/*name*/"wood_planks", /*max_stack*/99, /*places*/blockType::AIR        );
    
    // unqiue_liquids
    liquid_infos[(int)liquidType::EMPTY] = LiquidInfo(/*name*/"empty", /*flow_time*/0, /*speed_multiplier*/1  );
    liquid_infos[(int)liquidType::WATER] = LiquidInfo(/*name*/"water", /*flow_time*/0, /*speed_multiplier*/0.5);
}
