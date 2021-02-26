//
//  itemEngine.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 26/01/2021.
//

#ifndef itemEngine_hpp
#define itemEngine_hpp

#include "itemType.hpp"
#include "blockType.hpp"
#include <string>
#include <vector>

namespace itemEngine {

struct uniqueItem {
    uniqueItem(const std::string& name, unsigned short stack_size, blockEngine::blockType places);
    std::string name;
    unsigned short stack_size;
    blockEngine::blockType places;
};

struct item {
    item(itemType item_id, int x, int y);
    void destroy();
    itemType item_id;
    int x, y, velocity_x, velocity_y;
    
    void update(unsigned short frame_length);
    [[nodiscard]] bool colliding() const;
    [[nodiscard]] uniqueItem& getUniqueItem() const;
    unsigned short id;
};

void close();
void updateItems();

inline std::vector<uniqueItem> unique_items;
inline std::vector<item> items;

void spawnItem(itemType item_id, int x, int y);

}

#endif /* itemEngine_hpp */
