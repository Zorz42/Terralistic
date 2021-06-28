//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include <random>
#include "clientMap.hpp"
#include "assert.hpp"
#include "properties.hpp"
#include "textures.hpp"

map::item* map::getItemById(unsigned short id) {
    for(item& i : map::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id")
    return nullptr;
}

const uniqueItem& map::item::getUniqueItem() const {
    return ::getUniqueItem(item_type);
}

void map::item::draw(short x_, short y_, unsigned char scale) {
    gfx::image& texture = getItemTexture(item_type);
    if(texture.getTexture()) {
        texture.scale = scale;
        gfx::render(texture, x, y);
    }
}

void map::renderItems() {
    for(map::item& item : items)
        item.draw(item.x / 100 - view_x + gfx::getWindowWidth() / 2, item.y / 100 - view_y + gfx::getWindowHeight() / 2, 2);
}
