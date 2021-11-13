#pragma once
#include "entities.hpp"

#define ITEM_WIDTH 8

class ItemType {
public:
    ItemType() = default;
    ItemType(std::string name, int stack_size, BlockType* places);
    
    std::string name;
    int stack_size;
    BlockType* places;
    int id;
};

namespace ItemTypes {
    inline ItemType nothing(/*name*/"nothing", /*max_stack*/0,  /*places*/&BlockTypes::air);
}

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

class Items {
    Entities* entities;
    Blocks* blocks;
    
    std::vector<ItemType*> item_types;
    std::vector<ItemType*> drops;
    
public:
    explicit Items(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks) { registerNewItemType(&ItemTypes::nothing); }
    Item* spawnItem(ItemType* type, int x, int y);
    
    void registerNewItemType(ItemType* item_type);
    ItemType* getItemTypeById(int item_id);
    ItemType* getItemTypeByName(const std::string& name);
    int getNumItemTypes();
    
    void setBlockDrop(BlockType* block_type, ItemType* item_type);
    ItemType* getBlockDrop(BlockType* block_type);
    
    EventSender<ItemCreationEvent> item_creation_event;
};
