#pragma once
#include <map>
#include "entities.hpp"
#include "walls.hpp"

#define ITEM_WIDTH 8

class ItemType {
public:
    explicit ItemType(std::string name) : name(std::move(name)) {}
    
    std::string name, display_name;
    int max_stack = 0;
    BlockType* places_block = nullptr;
    WallType* places_wall = nullptr;
    std::map<Tool*, int> tool_powers;
    int id = 0;
};

class Item : public Entity {
    ItemType* type;
public:
    Item(ItemType* type, int x, int y, int entity_item_count = 1, int id=0);
    ItemType* getType() const;
    int entity_item_count;
    int getWidth() override { return ITEM_WIDTH * 2; }
    int getHeight() override { return ITEM_WIDTH * 2; }
};

class ItemStack {
public:
    ItemStack(ItemType* type, int stack) : type(type), stack(stack) {}
    ItemStack() = default;
    ItemType* type = nullptr;
    int stack = 0;
};

class ItemCreationEvent {
public:
    explicit ItemCreationEvent(const Item* item) : item(item) {}
    const Item* item;
};

class TileDrop {
public:
    TileDrop() : drop(nullptr) {}
    explicit TileDrop(ItemType* drop, float chance=1) : drop(drop), chance(chance) {}
    ItemType* drop;
    float chance = 0;
};

class Items {
    Entities* entities;
    Blocks* blocks;
    
    std::vector<ItemType*> item_types;
    std::vector<TileDrop> block_drops;
    std::vector<TileDrop> wall_drops;
    
public:
    Items(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks), nothing("nothing") { nothing.places_block = &blocks->air; nothing.max_stack = 0; nothing.display_name = "Nothing"; registerNewItemType(&nothing); }
    Item* spawnItem(ItemType* type, int x, int y);
    
    ItemType nothing;
    
    void registerNewItemType(ItemType* item_type);
    ItemType* getItemTypeById(int item_id);
    ItemType* getItemTypeByName(const std::string& name);
    int getNumItemTypes();
    
    void setBlockDrop(BlockType* block_type, TileDrop drop);
    TileDrop getBlockDrop(BlockType* block_type);
    
    void setWallDrop(WallType* wall_type, TileDrop drop);
    TileDrop getWallDrop(WallType* wall_type);
    
    EventSender<ItemCreationEvent> item_creation_event;
};
