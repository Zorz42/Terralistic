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
    BlockType grass            ("grass",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1,           /*connects_to*/{                                                                     }, /*color*/{50,   203, 50});
}

void addBlocks(Blocks* blocks, Items* items) {
    blocks->registerNewBlockType(&BlockTypes::dirt);
    items->setBlockDrop(&BlockTypes::dirt, BlockDrop(&ItemTypes::dirt));
    blocks->registerNewBlockType(&BlockTypes::stone_block);
    items->setBlockDrop(&BlockTypes::stone_block, BlockDrop(&ItemTypes::stone_block));
    blocks->registerNewBlockType(&BlockTypes::grass_block);
    blocks->registerNewBlockType(&BlockTypes::stone);
    items->setBlockDrop(&BlockTypes::stone, BlockDrop(&ItemTypes::stone));
    blocks->registerNewBlockType(&BlockTypes::wood);
    items->setBlockDrop(&BlockTypes::wood, BlockDrop(&ItemTypes::wood_planks));
    blocks->registerNewBlockType(&BlockTypes::leaves);
    blocks->registerNewBlockType(&BlockTypes::sand);
    blocks->registerNewBlockType(&BlockTypes::snowy_grass_block);
    blocks->registerNewBlockType(&BlockTypes::snow_block);
    blocks->registerNewBlockType(&BlockTypes::ice_block);
    blocks->registerNewBlockType(&BlockTypes::iron_ore);
    items->setBlockDrop(&BlockTypes::iron_ore, BlockDrop(&ItemTypes::iron_ore));
    blocks->registerNewBlockType(&BlockTypes::copper_ore);
    items->setBlockDrop(&BlockTypes::copper_ore, BlockDrop(&ItemTypes::copper_ore));
    blocks->registerNewBlockType(&BlockTypes::grass);
    items->setBlockDrop(&BlockTypes::grass, BlockDrop(&ItemTypes::fiber, 0.3));
}

static bool isBlockTree(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && (blocks->getBlockType(x, y) == &BlockTypes::wood || blocks->getBlockType(x, y) == &BlockTypes::leaves);
}

static bool isBlockWood(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == &BlockTypes::wood;
}

static bool isBlockLeaves(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == &BlockTypes::leaves;
}

static void stoneUpdate(Blocks* blocks, int x, int y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
        blocks->breakBlock(x, y);
}

static void treeUpdate(Blocks* blocks, int x, int y) {
    if(
       (!isBlockTree(blocks, x, y + 1) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y)) ||
       (isBlockWood(blocks, x, y - 1) && isBlockWood(blocks, x + 1, y) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockWood(blocks, x, y - 1) && isBlockWood(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockLeaves(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y) && !isBlockTree(blocks, x, y - 1) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockLeaves(blocks, x + 1, y) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x, y - 1) && !isBlockTree(blocks, x, y + 1)) ||
       (!isBlockTree(blocks, x, y + 1) && isBlockLeaves(blocks, x - 1, y) && isBlockLeaves(blocks, x + 1, y) && isBlockLeaves(blocks, x, y - 1))
       )
        blocks->breakBlock(x, y);
}

void addBlockBehaviour(ServerPlayers* players) {
    players->getBlockBehaviour(&BlockTypes::wood).onUpdate = &treeUpdate;
    players->getBlockBehaviour(&BlockTypes::leaves).onUpdate = &treeUpdate;
    players->getBlockBehaviour(&BlockTypes::grass_block).onLeftClick = [](Blocks* blocks_, int x, int y, ServerPlayer* player) {
        blocks_->setBlockType(x, y, &BlockTypes::dirt);
    };
    players->getBlockBehaviour(&BlockTypes::air).onRightClick = [](Blocks* blocks_, int x, int y, ServerPlayer* player) {
        BlockType* type = player->inventory.getSelectedSlot().type->places;
        if(type != &BlockTypes::air && player->inventory.decreaseStack(player->inventory.selected_slot, 1)) {
            blocks_->setBlockType(x, y, type);
        }
    };
    players->getBlockBehaviour(&BlockTypes::snowy_grass_block).onLeftClick = players->getBlockBehaviour(&BlockTypes::grass_block).onLeftClick;
    players->getBlockBehaviour(&BlockTypes::wood).onUpdate = &stoneUpdate;
    players->getBlockBehaviour(&BlockTypes::wood).onUpdate = &stoneUpdate;
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
    items->registerNewItemType(&ItemTypes::fiber);
    items->registerNewItemType(&ItemTypes::hatchet);
}

void addRecipes(Recipes* recipes) {
    Recipe* recipe;
    
    recipe = new Recipe;
    recipe->result = ItemStack(&ItemTypes::hatchet, 1);
    recipe->ingredients[&ItemTypes::fiber] = 4;
    recipe->ingredients[&ItemTypes::stone] = 1;
    recipes->registerARecipe(recipe);
}
