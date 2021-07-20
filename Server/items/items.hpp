#ifndef items_hpp
#define items_hpp

#include <vector>
#include "blocks.hpp"

class Item {
    inline static unsigned short curr_id = 0;
    
    int velocity_x = 0, velocity_y = 0;
    int x, y;
    unsigned short id;
    ItemType type;
    Blocks* parent_blocks;
    
    bool collidingWithBlocks() const;
    bool grounded();
public:
    Item(Blocks* parent_blocks, ItemType type, int x, int y) : parent_blocks(parent_blocks), type(type), x(x * 100), y(y * 100), id(curr_id++) {}
    void update(float frame_length);
    const ItemInfo& getUniqueItem() const;
    unsigned short getId() const;
    ItemType getType() const;
    int getX() const;
    int getY() const;
    void addVelocityX(int vel_x);
    void addVelocityY(int vel_y);
};

class Items : EventListener<ServerBlockBreakEvent> {
    Blocks* parent_blocks;
    std::vector<Item> item_arr;
    void onEvent(ServerBlockBreakEvent& event) override;
public:
    Items(Blocks* parent_blocks) : parent_blocks(parent_blocks) {}
    
    void spawnItem(ItemType item_id, int x, int y);
    void updateItems(float frame_length);
    void removeItem(const Item& item_to_destroy);
    
    inline const std::vector<Item>& getItems() { return item_arr; }
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
    ServerItemMovementEvent(Item& moved_item) : moved_item(moved_item) {}
    Item& moved_item;
};

class ServerItemDeletionEvent : public Event<ServerItemDeletionEvent> {
public:
    ServerItemDeletionEvent(Item& item_to_delete) : item_to_delete(item_to_delete) {}
    Item& item_to_delete;
};

#endif /* items_hpp */
