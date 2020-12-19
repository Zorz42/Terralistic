//
//  itemEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "itemEngine.hpp"
#include "blockEngine.hpp"
#include "playerHandler.hpp"

void itemEngine::init() {
    unique_items = {
        uniqueItem("nothing", 0),
        uniqueItem("stone", 99),
        uniqueItem("dirt", 99),
        uniqueItem("stone_block", 99),
    };
}

void itemEngine::prepare() {
    
}

void itemEngine::close() {
    items.clear();
}

void itemEngine::updateItems() {
    for(unsigned long i = 0; i < items.size(); i++) {
        items.at(i).update();
        if(abs(items.at(i).x / 100 + BLOCK_WIDTH / 2  - blockEngine::position_x - playerHandler::player.getWidth() / 2) < 50 && abs(items.at(i).y / 100 + BLOCK_WIDTH / 2 - blockEngine::position_y - playerHandler::player.getHeight() / 2) < 50)
            if(addItemToInventory(items.at(i).item_id, 1))
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

bool itemEngine::addItemToInventory(itemType id, int quantity) {
    for(int i = 0; i < 10; i++)
        if(inventory[i].item_id == id) {
            quantity -= inventory[i].increaseStack(quantity);
            if(!quantity)
                return true;
        }
    for(int i = 0; i < 10; i++)
        if(inventory[i].item_id == NOTHING) {
            inventory[i].item_id = id;
            quantity -= inventory[i].increaseStack(quantity);
            if(!quantity)
                return true;
        }
    return false;
}
