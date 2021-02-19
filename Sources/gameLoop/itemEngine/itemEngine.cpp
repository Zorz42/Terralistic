//
//  itemEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "blockEngine.hpp"
#include "playerHandler.hpp"
#include "dev.hpp"
#include "init.hpp"

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

void itemEngine::updateItems() {
    for(unsigned long i = 0; i < items.size(); i++) {
        items[i].update();
        if(abs(items[i].x / 100 + BLOCK_WIDTH / 2  - playerHandler::position_x - playerHandler::player.getWidth() / 2) < 50 && abs(items[i].y / 100 + BLOCK_WIDTH / 2 - playerHandler::position_y - playerHandler::player.getHeight() / 2) < 50) {
            char result = playerHandler::player_inventory.addItem(items[i].getItemId(), 1);
            if(result != -1) {
                items.erase(items.begin() + i);
            }
        }
    }
}

void itemEngine::renderItems() {
    for(item& i : items)
        i.draw();
}

void itemEngine::spawnItem(itemType item_id, int x, int y, unsigned short id) {
    items.emplace_back(item_id, x, y, id);
}

itemEngine::item* itemEngine::getItemById(unsigned short id) {
    for(item& i : itemEngine::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id");
    return nullptr;
}
