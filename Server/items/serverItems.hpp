#ifndef serverItems_hpp
#define serverItems_hpp

#include <vector>
#include "serverBlocks.hpp"
#include "serverEntities.hpp"

#define ITEM_WIDTH 8

class ServerItem : public ServerEntity {
    ItemType type;
    
    void onSpawn() override;
    void onDestroy() override;
public:
    ServerItem(ItemType type, int x, int y) : type(type), ServerEntity(EntityType::ITEM, x, y) {}
    void update() override;
    const ItemInfo& getItemInfo() const;
    unsigned short getId() const;
    ItemType getType() const;
    
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ServerItems : EventListener<ServerBlockBreakEvent> {
    void onEvent(ServerBlockBreakEvent& event) override;
    ServerEntities* entities;
public:
    explicit ServerItems(ServerEntities* entities) : entities(entities) {}
};



class ServerItemCreationEvent : public Event<ServerItemCreationEvent> {
public:
    ServerItemCreationEvent(ItemType item_id, int x, int y, short id) : item_id(item_id), x(x), y(y), id(id) {}
    ItemType item_id;
    int x, y;
    short id;
};

class ServerItemMovementEvent : public Event<ServerItemMovementEvent> {
public:
    explicit ServerItemMovementEvent(ServerItem& moved_item) : moved_item(moved_item) {}
    ServerItem& moved_item;
};

class ServerItemDeletionEvent : public Event<ServerItemDeletionEvent> {
public:
    explicit ServerItemDeletionEvent(const ServerItem& item_to_delete) : item_to_delete(item_to_delete) {}
    const ServerItem& item_to_delete;
};

#endif
