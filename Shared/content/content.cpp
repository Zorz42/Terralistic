#include "configManager.hpp"
#include "content.hpp"

void GameContent::addContent(Blocks* blocks_, Liquids* liquids_, Items* items_, Recipes* recipes, const std::string& resource_path) {
    blocks.addContent(blocks_, items_, &items, resource_path);
    liquids.addContent(liquids_);
    items.addContent(items_, blocks_, resource_path);
    addRecipes(recipes);
}

void GameContent::addRecipes(Recipes* recipes) {
    Recipe* recipe;
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.hatchet, 1);
    recipe->ingredients[&items.fiber] = 4;
    recipe->ingredients[&items.stone] = 1;
    recipes->registerARecipe(recipe);
}

BlockTypes::BlockTypes(Blocks* blocks) {
    for(BlockType* block_type : block_types)
        blocks->registerNewBlockType(block_type);
}

void BlockTypes::addContent(Blocks* blocks, Items *items, ItemTypes *item_types, const std::string& resource_path) {
    for(BlockType* block_type : block_types) {
        ConfigFile block_properties(resource_path + "blockinfos/" + block_type->name + ".txt");
        
        block_type->color = {(unsigned char)block_properties.getInt("color_r"), (unsigned char)block_properties.getInt("color_g"), (unsigned char)block_properties.getInt("color_b")};
        
        if(block_properties.getStr("break_time") == "UNBREAKABLE")
            block_type->break_time = UNBREAKABLE;
        else
            block_type->break_time = block_properties.getInt("break_time");
        
        block_type->ghost = block_properties.getStr("ghost") == "true";
        block_type->transparent = block_properties.getStr("transparent") == "true";
        
        std::string connects_to = block_properties.getStr("connects_to");
        while(!connects_to.empty()) {
            int iter = (int)connects_to.find(' ');
            std::string name = iter == -1 ? connects_to : connects_to.substr(0, iter);
            if(iter == -1)
                connects_to.clear();
            else
                connects_to.erase(connects_to.begin(), connects_to.begin() + iter + 1);

            block_type->connects_to.push_back(blocks->getBlockTypeByName(name));
        }
        
        ItemType* drop = items->getItemTypeByName(block_properties.getStr("drop"));
        if(drop != &items->nothing) {
            items->setBlockDrop(block_type, BlockDrop(drop, block_properties.getInt("drop_chance") / 100.f));
        }
    }
}

void BlockTypes::addBlockBehaviour(ServerPlayers* players) {
    players->getBlockBehaviour(&wood) = &wood_behaviour;
    players->getBlockBehaviour(&leaves) = &leaves_behaviour;
    players->getBlockBehaviour(&grass_block) = &grass_block_behaviour;
    players->getBlockBehaviour(&snowy_grass_block) = &snowy_grass_block_behaviour;
    players->getBlockBehaviour(&stone) = &stone_behaviour;
    players->getBlockBehaviour(&grass) = &grass_behaviour;
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

LiquidTypes::LiquidTypes() :
water(/*name*/"water", /*flow_time*/100, /*speed_multiplier*/0.5, /*color*/{0, 92, 230, 150})
{}

void LiquidTypes::addContent(Liquids* liquids) {
    liquids->registerNewLiquidType(&water);
}

ItemTypes::ItemTypes(BlockTypes* blocks, Blocks* blocks_, Items* items) {
    for(ItemType* item_type : item_types)
        items->registerNewItemType(item_type);
}

void ItemTypes::addContent(Items* items, Blocks* blocks, const std::string& resource_path) {
    for(ItemType* item_type : item_types) {
        ConfigFile item_properties(resource_path + "iteminfos/" + item_type->name + ".txt");
        item_type->max_stack = item_properties.getInt("max_stack");
        item_type->places = blocks->getBlockTypeByName(item_properties.getStr("places"));
    }
}
