#pragma once
#include "liquids.hpp"
#include "inventory.hpp"
#include "serverPlayers.hpp"

class ItemTypes;

class BlockTypes {
public:
    BlockTypes();
    void addContent(Blocks* blocks, Items* items, ItemTypes* item_types, const std::string& resource_path);
    
    bool isBlockTree(Blocks* blocks, int x, int y);
    bool isBlockWood(Blocks* blocks, int x, int y);
    bool isBlockLeaves(Blocks* blocks, int x, int y);
    
    BlockType dirt;
    BlockType stone_block;
    BlockType grass_block;
    BlockType stone;
    BlockType wood;
    BlockType leaves;
    BlockType sand;
    BlockType snowy_grass_block;
    BlockType snow_block;
    BlockType ice_block;
    BlockType iron_ore;
    BlockType copper_ore;
    BlockType grass;
};

class LiquidTypes {
public:
    LiquidTypes();
    void addContent(Liquids* liquids);
    
    LiquidType water;
};

class ItemTypes {
public:
    ItemTypes(BlockTypes* blocks, Blocks* blocks_);
    void addContent(Items* items);
    
    ItemType stone;
    ItemType dirt;
    ItemType stone_block;
    ItemType wood_planks;
    ItemType iron_ore;
    ItemType copper_ore;
    ItemType fiber;
    ItemType hatchet;
};

class WoodBehaviour : public BlockBehaviour {
    void onUpdate(Blocks* blocks, int x, int y) override;
    BlockTypes* blocks_;
public:
    WoodBehaviour(BlockTypes* blocks) : blocks_(blocks) {}
};

class LeavesBehaviour : public BlockBehaviour {
    void onUpdate(Blocks* blocks, int x, int y) override;
    BlockTypes* blocks_;
public:
    LeavesBehaviour(BlockTypes* blocks) : blocks_(blocks) {}
};

class GrassBlockBehaviour : public BlockBehaviour {
    void onLeftClick(Blocks* blocks, int x, int y, ServerPlayer* player) override;
    BlockTypes* blocks_;
public:
    GrassBlockBehaviour(BlockTypes* blocks) : blocks_(blocks) {}
};

class SnowyGrassBlockBehaviour : public BlockBehaviour {
    void onLeftClick(Blocks* blocks, int x, int y, ServerPlayer* player) override;
    BlockTypes* blocks_;
public:
    SnowyGrassBlockBehaviour(BlockTypes* blocks) : blocks_(blocks) {}
};

class StoneBehaviour : public BlockBehaviour {
    void onUpdate(Blocks* blocks, int x, int y) override;
};

class GrassBehaviour : public BlockBehaviour {
    void onUpdate(Blocks* blocks, int x, int y) override;
};

class GameContent {
    void addRecipes(Recipes* recipes);
    
    WoodBehaviour wood_behaviour;
    LeavesBehaviour leaves_behaviour;
    GrassBlockBehaviour grass_block_behaviour;
    SnowyGrassBlockBehaviour snowy_grass_block_behaviour;
    StoneBehaviour stone_behaviour;
    GrassBehaviour grass_behaviour;
public:
    GameContent(Blocks* blocks_) : blocks(), liquids(), items(&blocks, blocks_), wood_behaviour(&blocks), leaves_behaviour(&blocks), grass_block_behaviour(&blocks), snowy_grass_block_behaviour(&blocks) {}
    
    void addContent(Blocks* blocks_, Liquids* liquids_, Items* items_, Recipes* recipes, const std::string& resource_path);
    void addBlockBehaviour(ServerPlayers* players);
    
    BlockTypes blocks;
    LiquidTypes liquids;
    ItemTypes items;
};

