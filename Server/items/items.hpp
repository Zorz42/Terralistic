//
//  items.hpp
//  Server
//
//  Created by Jakob Zorz on 22/06/2021.
//

#ifndef items_hpp
#define items_hpp

#include <vector>

enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS};

struct uniqueItem {
    uniqueItem(std::string name, unsigned short stack_size, blockType places);
    std::string name;
    unsigned short stack_size;
    blockType places;
};

inline std::vector<uniqueItem> unique_items;
inline std::vector<itemType> block_drops;

class item {
    int velocity_x, velocity_y;
    unsigned short id;
    itemType item_id;
public:
    void create(itemType item_id_, int x_, int y_, unsigned short id_, serverMap& world_serverMap);
    void destroy(serverMap& world_serverMap);
    void update(float frame_length, serverMap& world_serverMap);
    bool colliding(serverMap& world_serverMap) const;
    int x, y;

    [[maybe_unused]] [[nodiscard]] uniqueItem& getUniqueItem() const;
    [[nodiscard]] unsigned short getId() const { return id; }
    itemType getItemId() { return item_id; }
};

class items {
    void updateItems(float frame_length);
    void spawnItem(itemType item_id, int x, int y, short id=-1);
    std::vector<item> items;
    
    static void initItems();
};

#endif /* items_hpp */
