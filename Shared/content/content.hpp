#pragma once
#include "liquids.hpp"
#include "inventory.hpp"
#include "serverPlayers.hpp"

class ItemTypes;

class BlockTypes {
public:
    BlockTypes();
    void addContent(Blocks* blocks, Items* items, ItemTypes* item_types);
    
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

class GameContent {
    void addRecipes(Recipes* recipes);
public:
    GameContent(Blocks* blocks_) : blocks(), liquids(), items(&blocks, blocks_) {}
    
    void addContent(Blocks* blocks_, Liquids* liquids_, Items* items_, Recipes* recipes);
    void addBlockBehaviour(ServerPlayers* players);
    
    BlockTypes blocks;
    LiquidTypes liquids;
    ItemTypes items;
};
