//
//  items.hpp
//  Server
//
//  Created by Jakob Zorz on 22/06/2021.
//

#ifndef items_hpp
#define items_hpp

#include <vector>
#include "blocks.hpp"
#include "events.hpp"


class item {
    int velocity_x, velocity_y;
    short id;
    ItemType item_id;
    Blocks* parent_blocks;
public:
    void create(ItemType item_id_, int x_, int y_, unsigned short id_, Blocks* parent_blocks_);
    void update(float frame_length);
    bool colliding() const;
    int x, y;

    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    [[nodiscard]] unsigned short getId() const { return id; }
    inline ItemType getItemId() const { return item_id; }
};

class ServerItemCreationEvent : public Event<ServerItemCreationEvent> {
public:
    ServerItemCreationEvent(ItemType item_id, int x, int y, short id) : item_id(item_id), x(x), y(y), id(id) {}
    ItemType item_id;
    int x, y;
    short id;
};

class ServerItemDeletionEvent : public Event<ServerItemDeletionEvent> {
public:
    ServerItemDeletionEvent(item& item_to_delete) : item_to_delete(item_to_delete) {}
    item& item_to_delete;
};

class ServerItemMovementEvent : public Event<ServerItemMovementEvent> {
public:
    ServerItemMovementEvent(item& moved_item) : moved_item(moved_item) {}
    item& moved_item;
};

class items {
    Blocks* parent_blocks;
    
    std::vector<item> item_arr;
public:
    items(Blocks* parent_blocks) : parent_blocks(parent_blocks) {}
    
    void updateItems(float frame_length);
    void spawnItem(ItemType item_id, int x, int y, short id=-1);
    void destroyItem(const item& item_to_destroy);
    inline const std::vector<item>& getItems() { return item_arr; }
};

#endif /* items_hpp */
