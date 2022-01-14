#include "configManager.hpp"
#include "content.hpp"

void GameContent::loadContent(Blocks* blocks_, Walls* walls_, Liquids* liquids_, Items* items_, Recipes* recipes, const std::string& resource_path) {
    blocks.loadContent(blocks_, items_, &items, resource_path);
    walls.loadContent(walls_, items_, resource_path);
    liquids.loadContent(liquids_, resource_path);
    items.loadContent(items_, blocks_, walls_, resource_path);
    addRecipes(recipes);
}

void GameContent::addRecipes(Recipes* recipes) {
    Recipe* recipe;
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.hatchet, 1);
    recipe->ingredients[&items.fiber] = 4;
    recipe->ingredients[&items.stone] = 1;
    recipes->registerARecipe(recipe);
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.hammer, 1);
    recipe->ingredients[&items.fiber] = 4;
    recipe->ingredients[&items.stone] = 1;
    recipes->registerARecipe(recipe);
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.torch, 1);
    recipe->ingredients[&items.wood_planks] = 1;
    recipes->registerARecipe(recipe);
}

BlockTypes::BlockTypes(Blocks* blocks, Walls* walls, Liquids* liquids) :
    wood_behaviour(this, blocks, walls, liquids),
    leaves_behaviour(this, blocks, walls, liquids),
    grass_block_behaviour(this, blocks, walls, liquids),
    snowy_grass_block_behaviour(this, blocks, walls, liquids),
    stone_behaviour(blocks, walls, liquids),
    grass_behaviour(blocks, walls, liquids) {
    for(BlockType* block_type : block_types)
        blocks->registerNewBlockType(block_type);
}

void BlockTypes::loadContent(Blocks* blocks, Items *items, ItemTypes *item_types, const std::string& resource_path) {
    for(BlockType* block_type : block_types) {
        ConfigFile block_properties(resource_path + "blockinfos/" + block_type->name + ".txt");
        
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
        if(drop != &items->nothing)
            items->setBlockDrop(block_type, TileDrop(drop, block_properties.getInt("drop_chance") / 100.f));
        
        if(block_properties.keyExists("effective_tool"))
            block_type->effective_tool = blocks->getToolTypeByName(block_properties.getStr("effective_tool"));
        else
            block_type->effective_tool = &blocks->hand;
        
        if(block_properties.keyExists("effective_tool_power"))
            block_type->required_tool_power = block_properties.getInt("effective_tool_power");
        else
            block_type->required_tool_power = 0;
        
        block_type->light_emission_r = block_properties.getInt("light_emmision_r");
        block_type->light_emission_g = block_properties.getInt("light_emmision_g");
        block_type->light_emission_b = block_properties.getInt("light_emmision_b");
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

void WoodBehaviour::onUpdate(int x, int y) {
    if((y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1) == &blocks->air &&
       (!blocks_->isBlockTree(blocks, x - 1, y) || !blocks_->isBlockTree(blocks, x + 1, y)))
       || (!blocks_->isBlockWood(blocks, x + 1, y) && !blocks_->isBlockWood(blocks, x - 1, y) && !blocks_->isBlockWood(blocks, x, y + 1) && !blocks_->isBlockWood(blocks, x, y - 1)))
        blocks->breakBlock(x, y);
}

void LeavesBehaviour::onUpdate(int x, int y) {
    if(!blocks_->isBlockWood(blocks, x, y + 1) &&
       (!blocks_->isBlockLeaves(blocks, x, y - 1) || !blocks_->isBlockLeaves(blocks, x + 1, y) || blocks_->isBlockLeaves(blocks, x - 1, y)) &&
       (!blocks_->isBlockLeaves(blocks, x, y - 1) || !blocks_->isBlockLeaves(blocks, x - 1, y) || blocks_->isBlockLeaves(blocks, x + 1, y)))
        blocks->breakBlock(x, y);
}

void GrassBlockBehaviour::onLeftClick(int x, int y, ServerPlayer* player) {
    blocks->setBlockType(x, y, &blocks_->dirt);
}

void SnowyGrassBlockBehaviour::onLeftClick(int x, int y, ServerPlayer* player) {
    blocks->setBlockType(x, y, &blocks_->dirt);
}

void StoneBehaviour::onUpdate(int x, int y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
        blocks->breakBlock(x, y);
}

void GrassBehaviour::onUpdate(int x, int y) {
    if(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1)->transparent)
        blocks->breakBlock(x, y);
}

WallTypes::WallTypes(Walls* walls) :
dirt("dirt")
{
    for(WallType* wall_type : wall_types)
        walls->registerNewWallType(wall_type);
}
void WallTypes::loadContent(Walls* walls, Items* items, const std::string& resource_path) {
    for(WallType* wall_type : wall_types) {
        ConfigFile wall_properties(resource_path + "wallinfos/" + wall_type->name + ".txt");
        if(wall_properties.getStr("break_time") == "UNBREAKABLE")
            wall_type->break_time = UNBREAKABLE;
        else
            wall_type->break_time = wall_properties.getInt("break_time");
        
        items->setWallDrop(wall_type, TileDrop(items->getItemTypeByName(wall_properties.getStr("drops"))));
    }
}

LiquidTypes::LiquidTypes(Liquids* liquids) :
water("water")
{
    for(LiquidType* liquid_type : liquid_types)
        liquids->registerNewLiquidType(liquid_type);
}

void LiquidTypes::loadContent(Liquids* liquids, const std::string& resource_path) {
    for(LiquidType* liquid_type : liquid_types) {
        ConfigFile liquid_properties(resource_path + "liquidinfos/" + liquid_type->name + ".txt");
        liquid_type->flow_time = liquid_properties.getInt("flow_time");
        liquid_type->speed_multiplier = liquid_properties.getInt("speed_multiplier") / 100.f;
    }
}

ItemTypes::ItemTypes(Items* items) {
    for(ItemType* item_type : item_types)
        items->registerNewItemType(item_type);
}

void ItemTypes::loadContent(Items* items, Blocks* blocks, Walls* walls, const std::string& resource_path) {
    for(ItemType* item_type : item_types) {
        ConfigFile item_properties(resource_path + "iteminfos/" + item_type->name + ".txt");
        item_type->display_name = item_properties.getStr("display_name");
        item_type->max_stack = item_properties.getInt("max_stack");
        item_type->places_block = blocks->getBlockTypeByName(item_properties.getStr("places_block"));
        item_type->places_wall = walls->getWallTypeByName(item_properties.getStr("places_wall"));
        
        std::vector<std::string> tool_properties_split_up = {""};
        std::string tool_properties_raw = item_properties.getStr("tool_properties");
        for(int i = 0; i < tool_properties_raw.size(); i++) {
            if(tool_properties_raw[i] == ' ')
                tool_properties_split_up.emplace_back();
            else
                tool_properties_split_up.back().push_back(tool_properties_raw[i]);
        }
        
        for(int i = 0; i + 1 < tool_properties_split_up.size(); i += 2)
            item_type->tool_powers[blocks->getToolTypeByName(tool_properties_split_up[i])] = std::stoi(tool_properties_split_up[i + 1]);
    }
}
