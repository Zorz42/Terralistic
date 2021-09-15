#ifndef serverItems_hpp
#define serverItems_hpp

#include <vector>
#include "serverBlocks.hpp"
#include "serverEntities.hpp"

#define ITEM_WIDTH 8

class ServerItem : public ServerEntity {
    ItemType type;
public:
    ServerItem(ItemType type, int x, int y);
    ItemType getType() const;
    
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ServerItems : EventListener<ServerBlockBreakEvent> {
    void onEvent(ServerBlockBreakEvent& event) override;
    ServerEntities* entities;
    std::vector<ServerItem*> items;
public:
    explicit ServerItems(ServerEntities* entities) : entities(entities) {}
    const std::vector<ServerItem*>& getItems() { return items; }
    ServerItem* spawnItem(ItemType type, int x, int y);
    void removeItem(ServerItem* item);
};



class ServerItemCreationEvent : public Event<ServerItemCreationEvent> {
public:
    ServerItemCreationEvent(ItemType item_id, int x, int y, short id) : item_id(item_id), x(x), y(y), id(id) {}
    ItemType item_id;
    int x, y;
    short id;
};

#endif
