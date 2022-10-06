#include <filesystem>
#include "configManager.hpp"
#include "content.hpp"
#include "blockData.hpp"

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
    recipe->ingredients[&items.fiber] = 2;
    recipe->ingredients[&items.stone] = 1;
    recipe->ingredients[&items.stick] = 1;
    recipes->registerARecipe(recipe);
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.hammer, 1);
    recipe->ingredients[&items.fiber] = 2;
    recipe->ingredients[&items.stone] = 1;
    recipe->ingredients[&items.stick] = 1;
    recipes->registerARecipe(recipe);
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.torch, 1);
    recipe->ingredients[&items.wood_planks] = 1;
    recipes->registerARecipe(recipe);
    
    recipe = new Recipe;
    recipe->result = ItemStack(&items.stick, 1);
    recipe->ingredients[&items.branch] = 1;
    recipes->registerARecipe(recipe);

    recipe = new Recipe;
    recipe->result = ItemStack(&items.shovel, 1);
    recipe->ingredients[&items.fiber] = 2;
    recipe->ingredients[&items.stone] = 1;
    recipe->ingredients[&items.stick] = 1;
    recipe->crafting_block = &blocks.branch;
    recipes->registerARecipe(recipe);

    recipe = new Recipe;
    recipe->result = ItemStack(&items.furnace, 1);
    recipe->ingredients[&items.stone_block] = 4;
    recipe->ingredients[&items.branch] = 1;
    recipes->registerARecipe(recipe);
}

BlockTypes::BlockTypes(Blocks* blocks, Walls* walls, Liquids* liquids) :
    wood_behaviour(this, blocks, walls, liquids),
    torch_behaviour(blocks, walls, liquids),
    furnace_behaviour(blocks, walls, liquids),
    cactus_behaviour(this, blocks, walls, liquids),
    leaves_behaviour(this, blocks, walls, liquids),
    canopy_behaviour(this, blocks, walls, liquids),
    branch_behaviour(this, blocks, walls, liquids),
    grass_block_behaviour(this, blocks, walls, liquids),
    snowy_grass_block_behaviour(this, blocks, walls, liquids),
    stone_behaviour(blocks, walls, liquids),
    grass_behaviour(blocks, walls, liquids) {
    for(BlockType* block_type : block_types)
        blocks->registerNewBlockType(block_type);
}

void BlockTypes::loadContent(Blocks* blocks, Items *items, ItemTypes *item_types, const std::string& resource_path) {
    auto block_data_deliverer = new dataDeliverer;
    blocks->setDataDeliverer(block_data_deliverer);
    for(BlockType* block_type : block_types) {
        ConfigFile block_properties(resource_path + "blockinfos/" + block_type->name + ".txt");
        
        if(block_properties.getStr("break_time") == "UNBREAKABLE")
            block_type->break_time = UNBREAKABLE;
        else
            block_type->break_time = block_properties.getInt("break_time");
        
        block_type->ghost = block_properties.getStr("ghost") == "true";
        block_type->transparent = block_properties.getStr("transparent") == "true";
        block_type->can_update_states = block_properties.getStr("can_update_states") == "true";
        
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
        
        block_type->light_emission_r = block_properties.getInt("light_emission_r");
        block_type->light_emission_g = block_properties.getInt("light_emission_g");
        block_type->light_emission_b = block_properties.getInt("light_emission_b");
        
        if(block_properties.keyExists("width") && block_properties.keyExists("height")) {
            block_type->width = block_properties.getInt("width");
            block_type->height = block_properties.getInt("height");
        }

        if(block_properties.keyExists("additional_data_type")){
            std::string data_type = block_properties.getStr("additional_data_type");
            for(int i = 0; i < block_data_deliverer->names.size(); i++){
                if(block_data_deliverer->names[i] == data_type){
                    block_type->block_data_index = i;
                }
            }
        }
    }
}

void BlockTypes::addBlockBehaviour(ServerPlayers* players) {
    players->getBlockBehaviour(&wood) = &wood_behaviour;
    players->getBlockBehaviour(&cactus) = &cactus_behaviour;
    players->getBlockBehaviour(&leaves) = &leaves_behaviour;
    players->getBlockBehaviour(&canopy) = &canopy_behaviour;
    players->getBlockBehaviour(&branch) = &branch_behaviour;
    players->getBlockBehaviour(&grass_block) = &grass_block_behaviour;
    players->getBlockBehaviour(&snowy_grass_block) = &snowy_grass_block_behaviour;
    players->getBlockBehaviour(&stone) = &stone_behaviour;
    players->getBlockBehaviour(&grass) = &grass_behaviour;
    players->getBlockBehaviour(&torch) = &torch_behaviour;
    players->getBlockBehaviour(&furnace) = &furnace_behaviour;
}

bool BlockTypes::isBlockTree(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && (blocks->getBlockType(x, y) == &wood || blocks->getBlockType(x, y) == &leaves);
}

bool BlockTypes::isBlockWood(Blocks* blocks, int x, int y) {
    return x >= 0 && y >= 0 && x < blocks->getWidth() && y < blocks->getHeight() && (blocks->getBlockType(x, y) == &wood || blocks->getBlockType(x, y) == &branch);
}

void WoodBehaviour::onUpdate(int x, int y) {
    if((y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1) == &blocks->air &&
       (!blocks_->isBlockTree(blocks, x - 1, y) || !blocks_->isBlockTree(blocks, x + 1, y)))
       || (!blocks_->isBlockWood(blocks, x + 1, y) && !blocks_->isBlockWood(blocks, x - 1, y) && !blocks_->isBlockWood(blocks, x, y + 1) && !blocks_->isBlockWood(blocks, x, y - 1)))
        blocks->breakBlock(x, y);
}

void CactusBehaviour::onUpdate(int x, int y) {
    if(!(y > 0 && y < blocks->getHeight() - 1 && x > 0 && x < blocks->getWidth() - 1) ||
       !(blocks->getBlockType(x, y + 1)->id == blocks_->sand.id || blocks->getBlockType(x, y + 1)->id == blocks_->cactus.id ||
        ((blocks->getBlockType(x + 1, y)->id == blocks_->sand.id || blocks->getBlockType(x + 1, y)->id == blocks_->cactus.id) && (blocks->getBlockType(x + 1, y + 1)->id == blocks_->sand.id || blocks->getBlockType(x + 1, y + 1)->id == blocks_->cactus.id)) ||
        ((blocks->getBlockType(x - 1, y)->id == blocks_->sand.id || blocks->getBlockType(x - 1, y)->id == blocks_->cactus.id) && (blocks->getBlockType(x - 1, y + 1)->id == blocks_->sand.id || blocks->getBlockType(x - 1, y + 1)->id == blocks_->cactus.id))))
        blocks->breakBlock(x, y);
}

void BranchBehaviour::onUpdate(int x, int y) {
    if(!blocks_->isBlockWood(blocks, x + 1, y) && !blocks_->isBlockWood(blocks, x - 1, y))
        blocks->breakBlock(x, y);
}

void LeavesBehaviour::onUpdate(int x, int y) {
    if(!blocks_->isBlockWood(blocks, x + 1, y) && !blocks_->isBlockWood(blocks, x - 1, y))
        blocks->breakBlock(x, y);
}

void CanopyBehaviour::onUpdate(int x, int y) {
    if(blocks->getBlockXFromMain(x, y) == 2 && blocks->getBlockYFromMain(x, y) == 4 && !blocks_->isBlockWood(blocks, x, y + 1))
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

void TorchBehaviour::onUpdate(int x, int y) {
    if(liquids->getLiquidLevel(x, y) >= 50)
        blocks->setBlockType(x, y, blocks->getBlockTypeById(18));
}

int WoodType::updateState(Blocks* blocks, int x, int y){
    int state = 0;

    if(y - 1 >= blocks->getHeight() || y - 1 < 0 ||
              blocks->getBlockType(x, y - 1) == blocks->getBlockType(x, y) ||
              std::count(blocks->getBlockType(x, y)->connects_to.begin(), blocks->getBlockType(x, y)->connects_to.end(), blocks->getBlockType(x, y - 1))){
        state += 1 << 0;
    }
    if(x + 1 >= blocks->getWidth() || x + 1 < 0 ||
       blocks->getBlockType(x + 1, y) == blocks->getBlockType(x, y) ||
       std::count(blocks->getBlockType(x, y)->connects_to.begin(), blocks->getBlockType(x, y)->connects_to.end(), blocks->getBlockType(x + 1, y))){
        state += 1 << 1;
    }
    if(y + 1 >= blocks->getHeight() || y + 1 < 0 ||
       blocks->getBlockType(x, y + 1) == blocks->getBlockType(x, y) ||
       std::count(blocks->getBlockType(x, y)->connects_to.begin(), blocks->getBlockType(x, y)->connects_to.end(), blocks->getBlockType(x, y + 1))){
        state += 1 << 2;
    }
    if(x - 1 >= blocks->getWidth() || x - 1 < 0 ||
       blocks->getBlockType(x - 1, y) == blocks->getBlockType(x, y) ||
       std::count(blocks->getBlockType(x, y)->connects_to.begin(), blocks->getBlockType(x, y)->connects_to.end(), blocks->getBlockType(x - 1, y))){
        state += 1 << 3;
    }
    return state;
}

int FurnaceType::updateState(Blocks *blocks, int x, int y) {
    int state = 0;
    auto data = (FurnaceData*)blocks->getBlockData(x, y);
    if(data != nullptr) {
        if (data->fuel.stack > 0)
            state++;
        if (data->heated_items.stack > 0)
            state += 2;
    }
    return state;
}

void FurnaceBehaviour::onRightClick(int x, int y, ServerPlayer *player) {
    auto data = (FurnaceData*)blocks->getBlockData(x, y);
    data->fuel.stack++;
    //open ui somehow
    //make inventory change functions ->
        //fuel.type and heated_items.type should be nullptr when empty, otherwise a pointer to item type
        //fuel.stack and heated_items.stack should obviously be 0 when empty
    //when done and if anything changed call:
    BlockUpdateEvent event(x, y);
    blocks->block_update_event.call(event);
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
