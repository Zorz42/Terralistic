#include "content.hpp"

void GameContent::addContent(Blocks* blocks_, Liquids* liquids_, Items* items_, Recipes* recipes, const std::string& resource_path) {
    blocks.addContent(blocks_, items_, &items, resource_path);
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
grass            ("grass",             /*ghost*/true,  /*transparent*/true,  /*break_time*/1,           /*connects_to*/{                                 }, /*color*/{50,  203, 50} )
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

void BlockTypes::addContent(Blocks* blocks, Items* items, ItemTypes* item_types, const std::string& resource_path) {
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

bool BlockTypes::isBlockTree(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && (blocks->getBlockType(x, y) == &wood || blocks->getBlockType(x, y) == &leaves);
}

bool BlockTypes::isBlockWood(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == &wood;
}

bool BlockTypes::isBlockLeaves(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && blocks->getBlockType(x, y) == &leaves;
}

void WoodBehaviour::onUpdate(Blocks* blocks, int x, int y) {
    if(
       (!blocks_->isBlockTree(blocks, x, y + 1) && !blocks_->isBlockTree(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x + 1, y)) ||
       (blocks_->isBlockWood(blocks, x, y - 1) && blocks_->isBlockWood(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (blocks_->isBlockWood(blocks, x, y - 1) && blocks_->isBlockWood(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (blocks_->isBlockLeaves(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x, y - 1) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (blocks_->isBlockLeaves(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x, y - 1) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (!blocks_->isBlockTree(blocks, x, y + 1) && blocks_->isBlockLeaves(blocks, x - 1, y) && blocks_->isBlockLeaves(blocks, x + 1, y) && blocks_->isBlockLeaves(blocks, x, y - 1))
       )
        blocks->breakBlock(x, y);
}

void LeavesBehaviour::onUpdate(Blocks* blocks, int x, int y) {
    if(
       (!blocks_->isBlockTree(blocks, x, y + 1) && !blocks_->isBlockTree(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x + 1, y)) ||
       (blocks_->isBlockWood(blocks, x, y - 1) && blocks_->isBlockWood(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (blocks_->isBlockWood(blocks, x, y - 1) && blocks_->isBlockWood(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (blocks_->isBlockLeaves(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x, y - 1) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (blocks_->isBlockLeaves(blocks, x + 1, y) && !blocks_->isBlockTree(blocks, x - 1, y) && !blocks_->isBlockTree(blocks, x, y - 1) && !blocks_->isBlockTree(blocks, x, y + 1)) ||
       (!blocks_->isBlockTree(blocks, x, y + 1) && blocks_->isBlockLeaves(blocks, x - 1, y) && blocks_->isBlockLeaves(blocks, x + 1, y) && blocks_->isBlockLeaves(blocks, x, y - 1))
       )
        blocks->breakBlock(x, y);
}

void GrassBlockBehaviour::onLeftClick(Blocks* blocks, int x, int y, ServerPlayer* player) {
    blocks->setBlockType(x, y, &blocks_->dirt);
}

void SnowyGrassBlockBehaviour::onLeftClick(Blocks* blocks, int x, int y, ServerPlayer* player) {
    blocks->setBlockType(x, y, &blocks_->dirt);
}

void StoneBehaviour::onUpdate(Blocks* blocks, int x, int y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
        blocks->breakBlock(x, y);
}

void GrassBehaviour::onUpdate(Blocks* blocks, int x, int y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
        blocks->breakBlock(x, y);
}

void GameContent::addBlockBehaviour(ServerPlayers* players) {
    players->getBlockBehaviour(&blocks.wood) = &wood_behaviour;
    players->getBlockBehaviour(&blocks.leaves) = &leaves_behaviour;
    players->getBlockBehaviour(&blocks.grass_block) = &grass_block_behaviour;
    players->getBlockBehaviour(&blocks.snowy_grass_block) = &snowy_grass_block_behaviour;
    players->getBlockBehaviour(&blocks.stone) = &stone_behaviour;
    players->getBlockBehaviour(&blocks.grass) = &grass_behaviour;
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
