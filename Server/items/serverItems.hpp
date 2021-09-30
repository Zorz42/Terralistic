#ifndef serverItems_hpp
#define serverItems_hpp

#include <vector>
#include "blocks.hpp"
#include "entities.hpp"

#define ITEM_WIDTH 8

class ServerItem : public Entity {
    ItemType type;
public:
    ServerItem(Entities* entities, ItemType type, int x, int y);
    ItemType getType() const;
    
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ServerItemCreationEvent {
public:
    ServerItemCreationEvent(const ServerItem& item) : item(item) {}
    const ServerItem& item;
};

class ServerItemDeletionEvent {
public:
    explicit ServerItemDeletionEvent(const ServerItem& item) : item(item) {}
    const ServerItem& item;
};

class ServerItems : EventListener<BlockBreakEvent> {
    void onEvent(BlockBreakEvent& event) override;
    Entities* entities;
    Blocks* blocks;
    std::vector<ServerItem*> items;
public:
    explicit ServerItems(Entities* entities, Blocks* blocks) : entities(entities), blocks(blocks) {}
    void init();
    const std::vector<ServerItem*>& getItems() { return items; }
    ServerItem* spawnItem(ItemType type, int x, int y);
    void removeItem(ServerItem* item);
    EventSender<ServerItemCreationEvent> item_creation_event;
    EventSender<ServerItemDeletionEvent> item_deletion_event;
};

#endif
