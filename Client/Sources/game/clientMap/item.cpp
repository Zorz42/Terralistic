//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include <random>
#include "clientMap.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"
#include <cassert>

map::item* map::getItemById(unsigned short id) {
    for(item& i : map::items)
        if(i.getId() == id)
            return &i;
    assert(false);
    return nullptr;
}

const ItemInfo& map::item::getUniqueItem() const {
    return ::getItemInfo(item_type);
}

void map::renderItems() {
    for(map::item& item : items) {
        const gfx::Image& texture = resource_pack->getItemTexture(item.getType());
        texture.render(2, item.x / 100 - view_x + gfx::getWindowWidth() / 2, item.y / 100 - view_y + gfx::getWindowHeight() / 2);
    }
}
