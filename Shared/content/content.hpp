#pragma once
#include "liquids.hpp"
#include "inventory.hpp"
#include "serverPlayers.hpp"
#include "walls.hpp"

class ItemTypes;
class BlockTypes;

class WoodBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
    BlockTypes* blocks_;
public:
    WoodBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class CactusBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
    BlockTypes* blocks_;
public:
    CactusBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class LeavesBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
    BlockTypes* blocks_;
public:
    LeavesBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class CanopyBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
    BlockTypes* blocks_;
public:
    CanopyBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class BranchBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
    BlockTypes* blocks_;
public:
    BranchBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class GrassBlockBehaviour : public BlockBehaviour {
    void onLeftClick(int x, int y, ServerPlayer* player) override;
    BlockTypes* blocks_;
public:
    GrassBlockBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class SnowyGrassBlockBehaviour : public BlockBehaviour {
    void onLeftClick(int x, int y, ServerPlayer* player) override;
    BlockTypes* blocks_;
public:
    SnowyGrassBlockBehaviour(BlockTypes* blocks_, Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids), blocks_(blocks_) {}
};

class StoneBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
public:
    StoneBehaviour(Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids) {}
};

class GrassBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
public:
    GrassBehaviour(Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids) {}
};

class TorchBehaviour : public BlockBehaviour {
    void onUpdate(int x, int y) override;
public:
    TorchBehaviour(Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids) {}
};

class WoodType : public BlockType{
public:
    int updateState(Blocks* blocks, int x, int y) override;
    explicit WoodType(const std::string& name) : BlockType(name){}
};

class FurnaceType : public BlockType{
public:
    int updateState(Blocks* blocks, int x, int y) override;
    FurnaceType(const std::string& name) : BlockType(name){}
};

class FurnaceBehaviour : public BlockBehaviour{
    void onRightClick(int x, int y, ServerPlayer* player) override;
public:
    FurnaceBehaviour(Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids) {}
};


class BlockTypes {
    WoodBehaviour wood_behaviour;
    CactusBehaviour cactus_behaviour;
    LeavesBehaviour leaves_behaviour;
    CanopyBehaviour canopy_behaviour;
    BranchBehaviour branch_behaviour;
    GrassBlockBehaviour grass_block_behaviour;
    SnowyGrassBlockBehaviour snowy_grass_block_behaviour;
    StoneBehaviour stone_behaviour;
    GrassBehaviour grass_behaviour;
    TorchBehaviour torch_behaviour;
    FurnaceBehaviour furnace_behaviour;
    
    std::vector<BlockType*> block_types = {&dirt, &stone_block, &grass_block, &stone, &wood, &leaves, &canopy, &branch, &sand, &snowy_grass_block, &snow_block, &ice_block, &iron_ore, &copper_ore, &grass, &torch, &cactus, &unlit_torch, &furnace, &wood_planks, &wood_platform};
public:
    BlockTypes(Blocks* blocks, Walls* walls, Liquids* liquids);
    void loadContent(Blocks* blocks, Items *items, ItemTypes *item_types, const std::string& resource_path);
    void addBlockBehaviour(ServerPlayers* players);
    
    bool isBlockTree(Blocks* blocks, int x, int y);
    bool isBlockWood(Blocks* blocks, int x, int y);
    
    BlockType dirt{"dirt"};
    BlockType stone_block{"stone_block"};
    BlockType grass_block{"grass_block"};
    BlockType stone{"stone"};
    WoodType wood{"wood"};
    BlockType leaves{"leaves"};
    BlockType canopy{"canopy"};
    BlockType branch{"branch"};
    BlockType sand{"sand"};
    BlockType snowy_grass_block{"snowy_grass_block"};
    BlockType snow_block{"snow_block"};
    BlockType ice_block{"ice_block"};
    BlockType iron_ore{"iron_ore"};
    BlockType copper_ore{"copper_ore"};
    BlockType grass{"grass"};
    BlockType torch{"torch"};
    BlockType unlit_torch{"unlit_torch"};
    BlockType cactus{"cactus"};
    FurnaceType furnace{"furnace"};
    BlockType wood_planks{"wood_planks"};
    BlockType wood_platform{"wood_platform"};
};

class WallTypes {
    std::vector<WallType*> wall_types = {&dirt, &wood};
public:
    explicit WallTypes(Walls* walls);
    void loadContent(Walls* walls, Items* items, const std::string& resource_path);
    
    WallType dirt{"dirt"};
    WallType wood{"wood"};
};

class LiquidTypes {
    std::vector<LiquidType*> liquid_types = {&water};
public:
    explicit LiquidTypes(Liquids* liquids);
    void loadContent(Liquids* liquids, const std::string& resource_path);
    
    LiquidType water;
};

class ItemTypes {
    std::vector<ItemType*> item_types = {&stone, &dirt, &stone_block, &wood_planks, &iron_ore, &copper_ore, &fiber, &hatchet, &dirt_wall, &hammer, &torch, &stick, &branch, &shovel, &furnace, &wood_platform, &wood_wall};
public:
    explicit ItemTypes(Items* items);
    void loadContent(Items* items, Blocks* blocks, Walls* walls, const std::string& resource_path);
    
    ItemType stone{"stone"};
    ItemType dirt{"dirt"};
    ItemType stone_block{"stone_block"};
    ItemType wood_planks{"wood_planks"};
    ItemType iron_ore{"iron_ore"};
    ItemType copper_ore{"copper_ore"};
    ItemType fiber{"fiber"};
    ItemType hatchet{"hatchet"};
    ItemType dirt_wall{"dirt_wall"};
    ItemType hammer{"hammer"};
    ItemType torch{"torch"};
    ItemType stick{"stick"};
    ItemType branch{"branch"};
    ItemType shovel{"shovel"};
    ItemType furnace{"furnace"};
    ItemType wood_platform{"wood_platform"};
    ItemType wood_wall{"wood_wall"};
};

class GameContent : public NonCopyable {
    void addRecipes(Recipes* recipes);
public:
    GameContent(Blocks* blocks_, Walls* walls_, Liquids* liquids_, Items* items_) : blocks(blocks_, walls_, liquids_), walls(walls_), liquids(liquids_), items(items_), axe("axe") { blocks_->registerNewToolType(&axe); }
    
    void loadContent(Blocks* blocks_, Walls* walls_, Liquids* liquids_, Items* items_, Recipes* recipes, const std::string& resource_path);
    
    BlockTypes blocks;
    
    Tool axe;
    WallTypes walls;
    LiquidTypes liquids;
    ItemTypes items;
};

