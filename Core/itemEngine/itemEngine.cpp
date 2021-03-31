//
//  itemEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "core.hpp"

// item engine updates and renders items

INIT_SCRIPT
    // all currently unique items
    itemEngine::unique_items = {
        {"nothing",     0,  blockEngine::AIR        },
        {"stone",       99, blockEngine::STONE      },
        {"dirt",        99, blockEngine::DIRT       },
        {"stone_block", 99, blockEngine::STONE_BLOCK},
    };
INIT_SCRIPT_END

void itemEngine::close() {
    items.clear();
}

void itemEngine::updateItems(float frame_length) {
    for(auto & item : items)
        item.update(frame_length);
}

void itemEngine::spawnItem(itemType item_id, int x, int y, unsigned short id) {
    items.emplace_back();
    items.back().create(item_id, x, y, id);
}

itemEngine::item* itemEngine::getItemById(unsigned short id) {
    for(item& i : itemEngine::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id");
    return nullptr;
}
