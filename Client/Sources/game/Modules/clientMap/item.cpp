//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include "clientMap.hpp"
#include "assert.hpp"
#include <random>

std::vector<map::uniqueItem> map::unique_items;

void map::initItems() {
    // all currently unique items
    unique_items = {
        {"nothing"    },
        {"stone"      },
        {"dirt"       },
        {"stone_block"},
        {"wood_planks"},
    };
}

map::item* map::getItemById(unsigned short id) {
    for(item& i : map::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id")
    return nullptr;
}

map::uniqueItem::uniqueItem(const std::string& name) : name(name) {
    texture.setTexture(name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + name + ".png"));
    texture.scale = 2;
    texture.free_texture = false;
}

map::uniqueItem& map::item::getUniqueItem() const {
    ASSERT((int)item_type >= 0 && (int)item_type < unique_items.size(), "item_id is not valid")
    return unique_items[(int)item_type];
}

void map::item::draw(short x_, short y_, unsigned char scale) {
    getUniqueItem().draw(x_, y_, scale);
}

void map::uniqueItem::draw(short x, short y, unsigned char scale) {
    if(texture.getTexture()) {
        texture.scale = scale;
        gfx::render(texture, x, y);
    }
}

void map::renderItems() {
    for(map::item& item : items)
        item.draw(item.x / 100 - view_x + gfx::getWindowWidth() / 2, item.y / 100 - view_y + gfx::getWindowHeight() / 2, 2);
}
