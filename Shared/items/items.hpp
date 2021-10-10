#pragma once

#include <vector>
#include "blocks.hpp"
#include "entities.hpp"

#define ITEM_WIDTH 8

class Item : public Entity {
    ItemType type;
public:
    Item(ItemType type, int x, int y, unsigned short id=0);
    ItemType getType() const;
    
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
public:
    explicit Items(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks) {}
    Item* spawnItem(ItemType type, int x, int y);
    
    EventSender<ItemCreationEvent> item_creation_event;
};
