#pragma once
#include "entities.hpp"
#include "walls.hpp"

#define ITEM_WIDTH 8

class ItemType {
public:
    ItemType(std::string name) : name(std::move(name)) {}
    
    std::string name, display_name;
    int max_stack;
    BlockType* places_block;
    WallType* places_wall;
    int id;
};

class Item : public Entity {
    ItemType* type;
public:
    Item(ItemType* type, int x, int y, int id=0);
    ItemType* getType() const;
    
    int getWidth() override { return ITEM_WIDTH * 2; }
    int getHeight() override { return ITEM_WIDTH * 2; }
};

class ItemCreationEvent {
public:
    ItemCreationEvent(const Item* item) : item(item) {}
    const Item* item;
};

class BlockDrop {
public:
    BlockDrop() : drop(nullptr) {}
    explicit BlockDrop(ItemType* drop, float chance=1) : drop(drop), chance(chance) {}
    ItemType* drop;
    float chance;
};

class Items {
    Entities* entities;
    Blocks* blocks;
    
    std::vector<ItemType*> item_types;
    std::vector<BlockDrop> drops;
    
public:
    Items(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks), nothing("nothing") { nothing.places_block = &blocks->air; nothing.max_stack = 0; nothing.display_name = "Nothing"; registerNewItemType(&nothing); }
    Item* spawnItem(ItemType* type, int x, int y);
    
    ItemType nothing;
    
    void registerNewItemType(ItemType* item_type);
    ItemType* getItemTypeById(int item_id);
    ItemType* getItemTypeByName(const std::string& name);
    int getNumItemTypes();
    
    void setBlockDrop(BlockType* block_type, BlockDrop block_drop);
    BlockDrop getBlockDrop(BlockType* block_type);
    
    EventSender<ItemCreationEvent> item_creation_event;
};
