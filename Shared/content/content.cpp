#include "content.hpp"

void GameContent::addContent(Blocks* blocks_, Liquids* liquids_, Items* items_, Recipes* recipes) {
    blocks.addContent(blocks_, items_, &items);
    liquids.addContent(liquids_);
    items.addContent(items_);
    addRecipes(recipes);
}

BlockTypes::BlockTypes() :
dirt             ("dirt",              /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&grass_block, &snowy_grass_block }, /*color*/{115, 77,  38} ),
stone_block      ("stone_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&snowy_grass_block               }, /*color*/{128, 128, 128}),
grass_block      ("grass_block",       /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&dirt, &snowy_grass_block        }, /*color*/{0,   153, 0}  ),
stone            ("stone",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1500,        /*connects_to*/{                                 }, /*color*/{128, 128, 128}),
wood             ("wood",              /*ghost*/true,  /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&grass_block, &leaves            }, /*color*/{128, 85,  0}  ),
leaves           ("leaves",            /*ghost*/true,  /*transparent*/false, /*break_time*/UNBREAKABLE, /*connects_to*/{                                 }, /*color*/{0,   179, 0}  ),
sand             ("sand",              /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*connects_to*/{&dirt, &grass_block, &stone_block}, /*color*/{210, 170, 109}),
snowy_grass_block("snowy_grass_block", /*ghost*/false, /*transparent*/false, /*break_time*/1000,        /*connects_to*/{&dirt, &grass_block, &stone_block}, /*color*/{217, 217, 217}),
snow_block       ("snow_block",        /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*connects_to*/{&snowy_grass_block, &ice_block   }, /*color*/{242, 242, 242}),
ice_block        ("ice_block",         /*ghost*/false, /*transparent*/false, /*break_time*/500,         /*connects_to*/{&snow_block                      }, /*color*/{179, 217, 255}),
iron_ore         ("iron_ore",          /*ghost*/false, /*transparent*/false, /*break_time*/1500,        /*connects_to*/{                                 }, /*color*/{160, 160, 160}),
copper_ore       ("copper_ore",        /*ghost*/false, /*transparent*/false, /*break_time*/1500,        /*connects_to*/{                                 }, /*color*/{200, 109, 61} ),
grass            ("grass",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1,           /*connects_to*/{                                 }, /*color*/{50,   203, 50})
{}

LiquidTypes::LiquidTypes() :
water(/*name*/"water", /*flow_time*/100, /*speed_multiplier*/0.5, /*color*/{0, 92, 230, 150})
{}

ItemTypes::ItemTypes(BlockTypes* blocks, Blocks* blocks_) :
stone      (/*name*/"stone",       /*max_stack*/99, /*places*/&blocks->stone      ),
dirt       (/*name*/"dirt",        /*max_stack*/99, /*places*/&blocks->dirt       ),
stone_block(/*name*/"stone_block", /*max_stack*/99, /*places*/&blocks->stone_block),
wood_planks(/*name*/"wood_planks", /*max_stack*/99, /*places*/&blocks_->air       ),
iron_ore   (/*name*/"iron_ore",    /*max_stack*/99, /*places*/&blocks->iron_ore   ),
copper_ore (/*name*/"copper_ore",  /*max_stack*/99, /*places*/&blocks->copper_ore ),
fiber      (/*name*/"fiber",       /*max_stack*/99, /*places*/&blocks_->air       ),
hatchet    (/*name*/"hatchet",     /*max_stack*/1,  /*places*/&blocks_->air       )
{}

void BlockTypes::addContent(Blocks* blocks, Items* items, ItemTypes* item_types) {
    blocks->registerNewBlockType(&dirt);
    items->setBlockDrop(&dirt, BlockDrop(&item_types->dirt));
    blocks->registerNewBlockType(&stone_block);
    items->setBlockDrop(&stone_block, BlockDrop(&item_types->stone_block));
    blocks->registerNewBlockType(&grass_block);
    blocks->registerNewBlockType(&stone);
    items->setBlockDrop(&stone, BlockDrop(&item_types->stone));
    blocks->registerNewBlockType(&wood);
    items->setBlockDrop(&wood, BlockDrop(&item_types->wood_planks));
    blocks->registerNewBlockType(&leaves);
    blocks->registerNewBlockType(&sand);
    blocks->registerNewBlockType(&snowy_grass_block);
    blocks->registerNewBlockType(&snow_block);
    blocks->registerNewBlockType(&ice_block);
    blocks->registerNewBlockType(&iron_ore);
    items->setBlockDrop(&iron_ore, BlockDrop(&item_types->iron_ore));
    blocks->registerNewBlockType(&copper_ore);
    items->setBlockDrop(&copper_ore, BlockDrop(&item_types->copper_ore));
    blocks->registerNewBlockType(&grass);
    items->setBlockDrop(&grass, BlockDrop(&item_types->fiber, 0.3));
}

static bool isBlockTree(Blocks* blocks_, BlockTypes* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks_->getWidth() && y < blocks_->getHeight() && (blocks_->getBlockType(x, y) == &blocks->wood || blocks_->getBlockType(x, y) == &blocks->leaves);
}

static bool isBlockWood(Blocks* blocks_, BlockTypes* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks_->getWidth() && y < blocks_->getHeight() && blocks_->getBlockType(x, y) == &blocks->wood;
}

static bool isBlockLeaves(Blocks* blocks_, BlockTypes* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks_->getWidth() && y < blocks_->getHeight() && blocks_->getBlockType(x, y) == &blocks->leaves;
}

/*static void stoneUpdate(Blocks* blocks, int x, int y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
        blocks->breakBlock(x, y);
}*/

/*static void treeUpdate(Blocks* blocks, int x, int y) {
    if(
       (!isBlockTree(blocks, x, y + 1) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y)) ||
       (isBlockWood(blocks, x, y - 1) && isBlockWood(blocks, x + 1, y) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockWood(blocks, x, y - 1) && isBlockWood(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockLeaves(blocks, x - 1, y) && !isBlockTree(blocks, x + 1, y) && !isBlockTree(blocks, x, y - 1) && !isBlockTree(blocks, x, y + 1)) ||
       (isBlockLeaves(blocks, x + 1, y) && !isBlockTree(blocks, x - 1, y) && !isBlockTree(blocks, x, y - 1) && !isBlockTree(blocks, x, y + 1)) ||
       (!isBlockTree(blocks, x, y + 1) && isBlockLeaves(blocks, x - 1, y) && isBlockLeaves(blocks, x + 1, y) && isBlockLeaves(blocks, x, y - 1))
       )
        blocks->breakBlock(x, y);
}*/

void treeUpdate(GameContent* content, Blocks *blocks, int x, int y) {
    
}

void GameContent::addBlockBehaviour(ServerPlayers* players) {
    //players->getBlockBehaviour(&blocks.wood).onUpdate = [](Blocks* blocks, int x, int y) {
        
    //};
    //players->getBlockBehaviour(&blocks.leaves).onUpdate = players->getBlockBehaviour(&blocks.wood).onUpdate;
    
    //players->getBlockBehaviour(&blocks.grass_block).onLeftClick = [](Blocks* blocks, int x, int y, ServerPlayer* player) {
        //blocks_->setBlockType(x, y, &blocks.dirt);
    //};
    
    /*players->getBlockBehaviour(&BlockTypes::air).onRightClick = [](Blocks* blocks_, int x, int y, ServerPlayer* player) {
        BlockType* type = player->inventory.getSelectedSlot().type->places;
        if(type != &BlockTypes::air && player->inventory.decreaseStack(player->inventory.selected_slot, 1)) {
            blocks_->setBlockType(x, y, type);
        }
    };*/
    
    //players->getBlockBehaviour(&BlockTypes::snowy_grass_block).onLeftClick = players->getBlockBehaviour(&BlockTypes::grass_block).onLeftClick;
    
    /*players->getBlockBehaviour(&BlockTypes::wood).onUpdate = [this](Blocks* blocks, int x, int y) {
        if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
            blocks->breakBlock(x, y);
    };*/
    
    //players->getBlockBehaviour(&BlockTypes::wood).onUpdate = players->getBlockBehaviour(&BlockTypes::wood).onUpdate;
}

void LiquidTypes::addContent(Liquids* liquids) {
    liquids->registerNewLiquidType(&water);
}

void ItemTypes::addContent(Items* items) {
    items->registerNewItemType(&stone);
    items->registerNewItemType(&dirt);
    items->registerNewItemType(&stone_block);
    items->registerNewItemType(&wood_planks);
    items->registerNewItemType(&iron_ore);
    items->registerNewItemType(&copper_ore);
    items->registerNewItemType(&fiber);
    items->registerNewItemType(&hatchet);
}

void GameContent::addRecipes(Recipes* recipes) {
    Recipe* recipe;
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.hatchet, 1);
    recipe->ingredients[&items.fiber] = 4;
    recipe->ingredients[&items.stone] = 1;
    recipes->registerARecipe(recipe);
}
