#pragma once

#include <vector>
#include "blocks.hpp"
#include "entities.hpp"

#define ITEM_WIDTH 8

class Item : public Entity {
    ItemType type;
public:
    Item(Entities* entities, ItemType type, int x, int y);
    ItemType getType() const;
    
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ItemCreationEvent {
public:
    ItemCreationEvent(const Item* item) : item(item) {}
    const Item* item;
};

class Items : EventListener<BlockBreakEvent> {
    void onEvent(BlockBreakEvent& event) override;
    Entities* entities;
    Blocks* blocks;
public:
    explicit Items(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks) {}
    void init();
    Item* spawnItem(ItemType type, int x, int y);
    
    EventSender<ItemCreationEvent> item_creation_event;
};
