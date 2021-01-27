//
//  itemEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "blockEngine.hpp"
#include "playerHandler.hpp"
#include "inventory.hpp"

void itemEngine::init() {
    unique_items = {
        {"nothing",     0,  blockEngine::AIR        },
        {"stone",       99, blockEngine::STONE      },
        {"dirt",        99, blockEngine::DIRT       },
        {"stone_block", 99, blockEngine::STONE_BLOCK},
    };
}

void itemEngine::close() {
    items.clear();
}

void itemEngine::updateItems() {
    for(unsigned long i = 0; i < items.size(); i++) {
        items[i].update();
        if(abs(items[i].x / 100 + BLOCK_WIDTH / 2  - blockEngine::position_x - playerHandler::player.getWidth() / 2) < 50 && abs(items[i].y / 100 + BLOCK_WIDTH / 2 - blockEngine::position_y - playerHandler::player.getHeight() / 2) < 50 && inventory::addItemToInventory(items[i].item_id, 1))
            items.erase(items.begin() + i);
    }
}

void itemEngine::renderItems() {
    for(item& i : items)
        i.draw();
}

void itemEngine::spawnItem(itemType item_id, int x, int y) {
    items.emplace_back(item_id, x, y);
}
