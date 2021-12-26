#pragma once
#include "liquids.hpp"
#include "inventory.hpp"
#include "serverPlayers.hpp"
#include "walls.hpp"

class ItemTypes;
class BlockTypes;

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

class BlockTypes {
    WoodBehaviour wood_behaviour{this};
    LeavesBehaviour leaves_behaviour{this};
    GrassBlockBehaviour grass_block_behaviour{this};
    SnowyGrassBlockBehaviour snowy_grass_block_behaviour{this};
    StoneBehaviour stone_behaviour;
    GrassBehaviour grass_behaviour;
    
    std::vector<BlockType*> block_types = {&dirt, &stone_block, &grass_block, &stone, &wood, &leaves, &sand, &snowy_grass_block, &snow_block, &ice_block, &iron_ore, &copper_ore, &grass};
public:
    BlockTypes(Blocks* blocks);
    void loadContent(Blocks* blocks, Items *items, ItemTypes *item_types, const std::string& resource_path);
    void addBlockBehaviour(ServerPlayers* players);
    
    bool isBlockTree(Blocks* blocks, int x, int y);
    bool isBlockWood(Blocks* blocks, int x, int y);
    bool isBlockLeaves(Blocks* blocks, int x, int y);
    
    BlockType dirt{"dirt"};
    BlockType stone_block{"stone_block"};
    BlockType grass_block{"grass_block"};
    BlockType stone{"stone"};
    BlockType wood{"wood"};
    BlockType leaves{"leaves"};
    BlockType sand{"sand"};
    BlockType snowy_grass_block{"snowy_grass_block"};
    BlockType snow_block{"snow_block"};
    BlockType ice_block{"ice_block"};
    BlockType iron_ore{"iron_ore"};
    BlockType copper_ore{"copper_ore"};
    BlockType grass{"grass"};
};

class WallTypes {
    std::vector<WallType*> wall_types = {&dirt};
public:
    WallTypes(Walls* walls);
    void loadContent(Walls* walls, const std::string& resource_path);
    
    WallType dirt;
};

class LiquidTypes {
    std::vector<LiquidType*> liquid_types = {&water};
public:
    LiquidTypes(Liquids* liquids);
    void loadContent(Liquids* liquids, const std::string& resource_path);
    
    LiquidType water;
};

class ItemTypes {
    std::vector<ItemType*> item_types = {&stone, &dirt, &stone_block, &wood_planks, &iron_ore, &copper_ore, &fiber, &hatchet};
public:
    ItemTypes(BlockTypes* blocks, Blocks* blocks_, Items* items);
    void loadContent(Items* items, Blocks* blocks, const std::string& resource_path);
    
    ItemType stone{"stone"};
    ItemType dirt{"dirt"};
    ItemType stone_block{"stone_block"};
    ItemType wood_planks{"wood_planks"};
    ItemType iron_ore{"iron_ore"};
    ItemType copper_ore{"copper_ore"};
    ItemType fiber{"fiber"};
    ItemType hatchet{"hatchet"};
};

class GameContent {
    void addRecipes(Recipes* recipes);
public:
    GameContent(Blocks* blocks_, Walls* walls_, Liquids* liquids_, Items* items_) : blocks(blocks_), walls(walls_), liquids(liquids_), items(&blocks, blocks_, items_) {}
    
    void loadContent(Blocks* blocks_, Walls* walls_, Liquids* liquids_, Items* items_, Recipes* recipes, const std::string& resource_path);
    
    BlockTypes blocks;
    WallTypes walls;
    LiquidTypes liquids;
    ItemTypes items;
};

