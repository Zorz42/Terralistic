#ifndef items_hpp
#define items_hpp

#include <vector>
#include "blocks.hpp"
#include "events.hpp"


class Item {
    int velocity_x, velocity_y;
    short id;
    ItemType item_id;
    Blocks* parent_blocks;
public:
    void create(ItemType item_id_, int x_, int y_, unsigned short id_, Blocks* parent_blocks_);
    void update(float frame_length);
    bool collidingWithBlocks() const;
    int x, y;

    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    [[nodiscard]] unsigned short getId() const { return id; }
    inline ItemType getItemId() const { return item_id; }
};

class Items {
    Blocks* parent_blocks;
    
    std::vector<Item> item_arr;
public:
    Items(Blocks* parent_blocks) : parent_blocks(parent_blocks) {}
    
    void updateItems(float frame_length);
    void spawnItem(ItemType item_id, int x, int y, short id=-1);
    void destroyItem(const Item& item_to_destroy);
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
