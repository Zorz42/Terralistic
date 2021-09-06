#ifndef serverItems_hpp
#define serverItems_hpp

#include <vector>
#include "serverBlocks.hpp"
#include "serverEntity.hpp"

#define ITEM_WIDTH 8

class ServerItem : ServerEntity {
    ItemType type;
    ServerBlocks* parent_blocks;
    
    bool grounded();
public:
    ServerItem(ServerBlocks* parent_blocks, ItemType type, int x_, int y_) : parent_blocks(parent_blocks), type(type) { x = x_; y = y_; }
    void update(float frame_length);
    const ItemInfo& getItemInfo() const;
    unsigned short getId() const;
    ItemType getType() const;
    int getX() const;
    int getY() const;
    void addVelocityX(float vel_x);
    void addVelocityY(float vel_y);
    
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ServerItems : EventListener<ServerBlockBreakEvent> {
    ServerBlocks* parent_blocks;
    std::vector<ServerItem*> item_arr;
    void onEvent(ServerBlockBreakEvent& event) override;
public:
    explicit ServerItems(ServerBlocks* parent_blocks) : parent_blocks(parent_blocks) {}
    
    void spawnItem(ItemType item_id, int x, int y);
    void updateItems(float frame_length);
    void removeItem(const ServerItem& item_to_destroy);
    
    const std::vector<ServerItem*>& getItems() { return item_arr; }
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
    explicit ServerItemDeletionEvent(ServerItem& item_to_delete) : item_to_delete(item_to_delete) {}
    ServerItem& item_to_delete;
};

#endif
