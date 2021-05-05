//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include "map.hpp"
#include "assert.hpp"
#include <random>

map::uniqueItem* map::unique_items = nullptr;

void map::initItems() {
    // all currently unique items
    unique_items = new uniqueItem[(int)itemType::TOTAL_ITEMS];
    
    unique_items[0] = {"nothing",     0, };
    unique_items[1] = {"stone",       99,};
    unique_items[2] = {"dirt",        99,};
    unique_items[3] = {"stone_block", 99,};
}

map::item* map::getItemById(unsigned short id) {
    for(item& i : map::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id");
    return nullptr;
}

map::uniqueItem::uniqueItem(const std::string& name, unsigned short stack_size) : name(name), stack_size(stack_size) {
    texture.setTexture(name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + name + ".png"));
    texture.scale = 2;
    texture.free_texture = false;
}

map::uniqueItem& map::item::getUniqueItem() const {
    ASSERT((int)item_type >= 0 && item_type < itemType::TOTAL_ITEMS, "item_id is not valid");
    return unique_items[(int)item_type];
}

void map::item::draw(short x, short y, unsigned char scale) {
    getUniqueItem().draw(x, y, scale);
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
