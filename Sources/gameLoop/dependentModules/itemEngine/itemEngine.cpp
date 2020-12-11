//
//  itemEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "itemEngine.hpp"

void itemEngine::init() {
    unique_items = {
        uniqueItem("nothing"),
        uniqueItem("stone"),
    };
}

void itemEngine::prepare() {
    
}

void itemEngine::close() {
    items.clear();
}

void itemEngine::updateItems() {
    
}

void itemEngine::renderItems() {
    for(item& i : items)
        i.draw();
}

void itemEngine::spawnItem(itemType item_id, int x, int y) {
    items.emplace_back(item_id, x, y);
}
