//
//  itemEngine.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#ifndef itemEngine_hpp
#define itemEngine_hpp

#include "itemType.hpp"
#include "blockType.hpp"

namespace itemEngine {

struct uniqueItem {
    uniqueItem(const std::string& name, unsigned short stack_size, blockEngine::blockType places);
    std::string name;
    unsigned short stack_size;
    blockEngine::blockType places;
};

struct item {
public:
    void create(itemType item_id_, int x_, int y_, unsigned short id_);
    void destroy();
    int x, y;
    void update(float frame_length);
    [[nodiscard]] bool colliding() const;
    [[nodiscard]] uniqueItem& getUniqueItem() const;
    unsigned short getId() { return id; }
    itemType getItemId() { return item_id; }
private:
    int velocity_x, velocity_y;
    unsigned short id;
    itemType item_id;
};

void close();
void updateItems(float frame_length);

inline std::vector<uniqueItem> unique_items;
inline std::vector<item> items;

inline unsigned short curr_id = 0;

void spawnItem(itemType item_id, int x, int y, unsigned short id=curr_id++);

item* getItemById(unsigned short id);

REGISTER_EVENT(item_movement) {
    itemEngine::item* item;
};

REGISTER_EVENT(item_creation) {
    itemEngine::item* item;
};

REGISTER_EVENT(item_deletion) {
    itemEngine::item* item;
};

}

#endif /* itemEngine_hpp */
