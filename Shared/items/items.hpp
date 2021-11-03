#pragma once

#include <vector>
#include "blocks.hpp"
#include "entities.hpp"

#define ITEM_WIDTH 8

class ItemType {
public:
    ItemType() = default;
    ItemType(std::string name, unsigned short stack_size, BlockTypeOld places);
    
    std::string name;
    unsigned short stack_size;
    BlockTypeOld places;
    unsigned char id;
};

namespace ItemTypes {
    inline ItemType nothing(/*name*/"nothing", /*max_stack*/0,  /*places*/BlockTypeOld::AIR);
}

class Item : public Entity {
    ItemTypeOld type;
public:
    Item(ItemTypeOld type, int x, int y, unsigned short id=0);
    ItemTypeOld getType() const;
    
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
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
    
public:
    explicit Items(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks) {}
    Item* spawnItem(ItemTypeOld type, int x, int y);
    
    void registerNewItemType(ItemType* item_type);
    ItemType* getItemTypeById(unsigned char item_id);
    unsigned char getNumItemTypes();
    
    EventSender<ItemCreationEvent> item_creation_event;
};
