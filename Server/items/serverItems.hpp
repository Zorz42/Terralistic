#ifndef serverItems_hpp
#define serverItems_hpp

#include <vector>
#include "serverBlocks.hpp"
#include "serverEntities.hpp"

#define ITEM_WIDTH 8

class ServerItem : public ServerEntity {
    ItemType type;
public:
    ServerItem(ServerEntities* entities, ItemType type, int x, int y);
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

class ServerItems : EventListener<ServerBlockBreakEvent> {
    void onEvent(ServerBlockBreakEvent& event) override;
    ServerEntities* entities;
    ServerBlocks* blocks;
    std::vector<ServerItem*> items;
public:
    explicit ServerItems(ServerEntities* entities, ServerBlocks* blocks) : entities(entities), blocks(blocks) {}
    void init();
    const std::vector<ServerItem*>& getItems() { return items; }
    ServerItem* spawnItem(ItemType type, int x, int y);
    void removeItem(ServerItem* item);
    EventSender<ServerItemCreationEvent> item_creation_event;
    EventSender<ServerItemDeletionEvent> item_deletion_event;
};

#endif
