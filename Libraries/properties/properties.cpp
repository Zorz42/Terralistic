//
//  properties.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#include "properties.hpp"
#include "assert.hpp"

static uniqueBlock unique_blocks[(int)blockType::NUM_BLOCKS];
static uniqueItem unique_items[(int)itemType::NUM_ITEMS];
static uniqueLiquid unique_liquids[(int)liquidType::NUM_LIQUIDS];

const uniqueBlock& getUniqueBlock(blockType type) {
    ASSERT((int)type >= 0 && type < blockType::NUM_BLOCKS, "block id is not valid")
    return unique_blocks[(int)type];
}

const uniqueItem& getUniqueItem(itemType type) {
    ASSERT((int)type >= 0 && type < itemType::NUM_ITEMS, "item id is not valid")
    return unique_items[(int)type];
}

const uniqueLiquid& getUniqueLiquid(liquidType type) {
    ASSERT((int)type >= 0 && type < liquidType::NUM_LIQUIDS, "item id is not valid")
    return unique_liquids[(int)type];
}

void initProperties() {
    // unique_blocks
    unique_blocks[(int)blockType::AIR] =
    uniqueBlock("air",               /*ghost*/true,  /*transparent*/true,  /*break_time*/UNBREAKABLE,
                /*connects_to*/{                                                               });
    unique_blocks[(int)blockType::DIRT] =
    uniqueBlock("dirt",              /*ghost*/false, /*transparent*/false, /*break_time*/1000,
                /*connects_to*/{blockType::GRASS_BLOCK, blockType::SNOWY_GRASS_BLOCK           });
    unique_blocks[(int)blockType::STONE_BLOCK] =
    uniqueBlock("stone_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,
                /*connects_to*/{blockType::SNOWY_GRASS_BLOCK                                   });
    unique_blocks[(int)blockType::GRASS_BLOCK] =
    uniqueBlock("grass_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,
                /*connects_to*/{blockType::DIRT, blockType::SNOWY_GRASS_BLOCK                  });
    unique_blocks[(int)blockType::STONE] =
    uniqueBlock("stone",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1500,
                /*connects_to*/{                                                               });
    unique_blocks[(int)blockType::WOOD] =
    uniqueBlock("wood",              /*ghost*/true,  /*transparent*/false, /*break_time*/1000,
                /*connects_to*/{blockType::GRASS_BLOCK, blockType::LEAVES                      });
    unique_blocks[(int)blockType::LEAVES] =
    uniqueBlock("leaves",            /*ghost*/true,  /*transparent*/false, /*break_time*/UNBREAKABLE,
                /*connects_to*/{                                                               });
    unique_blocks[(int)blockType::SAND] =
    uniqueBlock("sand",              /*ghost*/false, /*transparent*/false, /*break_time*/500,
                /*connects_to*/{blockType::DIRT, blockType::GRASS_BLOCK, blockType::STONE_BLOCK});
    unique_blocks[(int)blockType::SNOWY_GRASS_BLOCK] =
    uniqueBlock("snowy_grass_block", /*ghost*/false, /*transparent*/false, /*break_time*/1000,
                /*connects_to*/{blockType::DIRT, blockType::GRASS_BLOCK, blockType::STONE_BLOCK});
    unique_blocks[(int)blockType::SNOW_BLOCK] =
    uniqueBlock("snow_block",        /*ghost*/false, /*transparent*/false, /*break_time*/500,
                /*connects_to*/{blockType::SNOWY_GRASS_BLOCK, blockType::ICE                   });
    unique_blocks[(int)blockType::ICE] =
    uniqueBlock("ice_block",         /*ghost*/false, /*transparent*/false, /*break_time*/500,
                /*connects_to*/{blockType::SNOW_BLOCK                                          });
    
    // unique_items
    unique_items[(int)itemType::NOTHING] =     uniqueItem(/*name*/"nothing",     /*max_stack*/0,  /*places*/blockType::AIR        );
    unique_items[(int)itemType::STONE] =       uniqueItem(/*name*/"stone",       /*max_stack*/99, /*places*/blockType::STONE      );
    unique_items[(int)itemType::DIRT] =        uniqueItem(/*name*/"dirt",        /*max_stack*/99, /*places*/blockType::DIRT       );
    unique_items[(int)itemType::STONE_BLOCK] = uniqueItem(/*name*/"stone_block", /*max_stack*/99, /*places*/blockType::STONE_BLOCK);
    unique_items[(int)itemType::WOOD_PLANKS] = uniqueItem(/*name*/"wood_planks", /*max_stack*/99, /*places*/blockType::AIR        );
    
    // unqiue_liquids
    unique_liquids[(int)liquidType::EMPTY] = uniqueLiquid(/*name*/"empty", /*flow_time*/0, /*speed_multiplier*/1  );
    unique_liquids[(int)liquidType::WATER] = uniqueLiquid(/*name*/"water", /*flow_time*/0, /*speed_multiplier*/0.5);
}
