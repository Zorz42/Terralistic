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

struct uniqueItem {
    uniqueItem(std::string name, unsigned short stack_size, blockType places);
    std::string name;
    unsigned short stack_size;
    blockType places;
};

inline std::vector<uniqueItem> unique_items;

void initItems();

class item {
    int velocity_x, velocity_y;
    unsigned short id;
    itemType item_id;
    blocks* parent_blocks;
    serverNetworkingManager* manager;
public:
    void create(itemType item_id_, int x_, int y_, unsigned short id_, blocks* parent_blocks_, serverNetworkingManager* manager_);
    void destroy();
    void update(float frame_length);
    bool colliding() const;
    int x, y;

    [[maybe_unused]] [[nodiscard]] uniqueItem& getUniqueItem() const;
    [[nodiscard]] unsigned short getId() const { return id; }
    inline itemType getItemId() const { return item_id; }
};

class items {
    blocks* parent_blocks;
    serverNetworkingManager* manager;
    
    std::vector<item> item_arr;
public:
    items(serverNetworkingManager* manager_, blocks* parent_blocks) : parent_blocks(parent_blocks), manager(manager_) {}
    
    void updateItems(float frame_length);
    void spawnItem(itemType item_id, int x, int y, short id=-1);
    void destroyItem(const item& item_to_destroy);
    inline const std::vector<item>& getItems() { return item_arr; }
};

#endif /* items_hpp */
