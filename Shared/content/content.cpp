#include "content.hpp"

void addBlocks(Blocks* blocks, Items* items);
void addLiquids(Liquids* liquids);
void addItems(Items* items);
void addRecipes(Recipes* recipes);

void addContent(Blocks* blocks, Liquids* liquids, Items* items, Recipes* recipes) {
    addBlocks(blocks, items);
    addLiquids(liquids);
    addItems(items);
    addRecipes(recipes);
}

namespace BlockTypes {
    BlockType dirt             ("dirt",              /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&BlockTypes::grass_block, &BlockTypes::snowy_grass_block             }, /*color*/{115, 77,  38} );
    BlockType stone_block      ("stone_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&BlockTypes::snowy_grass_block                                       }, /*color*/{128, 128, 128});
    BlockType grass_block      ("grass_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&BlockTypes::dirt, &BlockTypes::snowy_grass_block                    }, /*color*/{0,   153, 0}  );
    BlockType stone            ("stone",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1500,        /*connects_to*/{                                                                     }, /*color*/{128, 128, 128});
    BlockType wood             ("wood",              /*ghost*/true,  /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&BlockTypes::grass_block, &BlockTypes::leaves                        }, /*color*/{128, 85,  0}  );
    BlockType leaves           ("leaves",            /*ghost*/true,  /*transparent*/false, /*break_time*/UNBREAKABLE, /*connects_to*/{                                                                     }, /*color*/{0,   179, 0}  );
    BlockType sand             ("sand",              /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*connects_to*/{&BlockTypes::dirt, &BlockTypes::grass_block, &BlockTypes::stone_block}, /*color*/{210, 170, 109});
    BlockType snowy_grass_block("snowy_grass_block", /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&BlockTypes::dirt, &BlockTypes::grass_block, &BlockTypes::stone_block}, /*color*/{217, 217, 217});
    BlockType snow_block       ("snow_block",        /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*connects_to*/{&BlockTypes::snowy_grass_block, &BlockTypes::ice_block               }, /*color*/{242, 242, 242});
    BlockType ice_block        ("ice_block",         /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*connects_to*/{&BlockTypes::snow_block                                              }, /*color*/{179, 217, 255});
    BlockType iron_ore         ("iron_ore",          /*ghost*/false, /*transparent*/false, /*break_time*/1500,        /*connects_to*/{                                                                     }, /*color*/{160, 160, 160});
    BlockType copper_ore       ("copper_ore",        /*ghost*/false, /*transparent*/false, /*break_time*/1500,        /*connects_to*/{                                                                     }, /*color*/{200, 109, 61} );
}

void addBlocks(Blocks* blocks, Items* items) {
    blocks->registerNewBlockType(&BlockTypes::dirt);
    items->setBlockDrop(&BlockTypes::dirt, &ItemTypes::dirt);
    blocks->registerNewBlockType(&BlockTypes::stone_block);
    items->setBlockDrop(&BlockTypes::stone_block, &ItemTypes::stone_block);
    blocks->registerNewBlockType(&BlockTypes::grass_block);
    blocks->registerNewBlockType(&BlockTypes::stone);
    items->setBlockDrop(&BlockTypes::stone, &ItemTypes::stone);
    blocks->registerNewBlockType(&BlockTypes::wood);
    items->setBlockDrop(&BlockTypes::wood, &ItemTypes::wood_planks);
    blocks->registerNewBlockType(&BlockTypes::leaves);
    blocks->registerNewBlockType(&BlockTypes::sand);
    blocks->registerNewBlockType(&BlockTypes::snowy_grass_block);
    blocks->registerNewBlockType(&BlockTypes::snow_block);
    blocks->registerNewBlockType(&BlockTypes::ice_block);
    blocks->registerNewBlockType(&BlockTypes::iron_ore);
    items->setBlockDrop(&BlockTypes::iron_ore, &ItemTypes::iron_ore);
    blocks->registerNewBlockType(&BlockTypes::copper_ore);
    items->setBlockDrop(&BlockTypes::copper_ore, &ItemTypes::copper_ore);
}

void addLiquids(Liquids* liquids) {
    liquids->registerNewLiquidType(&LiquidTypes::water);
}

void addItems(Items* items) {
    items->registerNewItemType(&ItemTypes::stone);
    items->registerNewItemType(&ItemTypes::dirt);
    items->registerNewItemType(&ItemTypes::stone_block);
    items->registerNewItemType(&ItemTypes::wood_planks);
    items->registerNewItemType(&ItemTypes::iron_ore);
    items->registerNewItemType(&ItemTypes::copper_ore);
}

inline Recipe stone_to_dirt_recipe;
inline Recipe wood_planks_to_dirt_recipe;
inline Recipe stone_and_dirt_to_stone;

void addRecipes(Recipes* recipes) {
    stone_to_dirt_recipe.result = ItemStack(&ItemTypes::dirt, 2);
    stone_to_dirt_recipe.ingredients[&ItemTypes::stone_block] = 1;
    recipes->registerARecipe(&stone_to_dirt_recipe);

    wood_planks_to_dirt_recipe.result = ItemStack(&ItemTypes::dirt, 1);
    wood_planks_to_dirt_recipe.ingredients[&ItemTypes::wood_planks] = 1;
    recipes->registerARecipe(&wood_planks_to_dirt_recipe);

    stone_and_dirt_to_stone.result = ItemStack(&ItemTypes::stone_block, 1);
    stone_and_dirt_to_stone.ingredients[&ItemTypes::stone_block] = 2;
    stone_and_dirt_to_stone.ingredients[&ItemTypes::dirt] = 2;
    recipes->registerARecipe(&stone_and_dirt_to_stone);
}
