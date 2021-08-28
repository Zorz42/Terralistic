#ifndef serverItems_hpp
#define serverItems_hpp

#include <vector>
#include "serverBlocks.hpp"

class ServerItem {
    inline static unsigned short curr_id = 0;
    
    int velocity_x = 0, velocity_y = 0;
    int x, y;
    unsigned short id;
    ItemType type;
    ServerBlocks* parent_blocks;
    
    bool collidingWithBlocks() const;
    bool grounded();
public:
    ServerItem(ServerBlocks* parent_blocks, ItemType type, int x, int y) : parent_blocks(parent_blocks), type(type), x(x * 100), y(y * 100), id(curr_id++) {}
    void update(float frame_length);
    const ItemInfo& getUniqueItem() const;
    unsigned short getId() const;
    ItemType getType() const;
    int getX() const;
    int getY() const;
    void addVelocityX(int vel_x);
    void addVelocityY(int vel_y);
};

class ServerItems : EventListener<ServerBlockBreakEvent> {
    ServerBlocks* parent_blocks;
    std::vector<ServerItem> item_arr;
    void onEvent(ServerBlockBreakEvent& event) override;
public:
    explicit ServerItems(ServerBlocks* parent_blocks) : parent_blocks(parent_blocks) {}
    
    void spawnItem(ItemType item_id, int x, int y);
    void updateItems(float frame_length);
    void removeItem(const ServerItem& item_to_destroy);
    
    const std::vector<ServerItem>& getItems() { return item_arr; }
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
