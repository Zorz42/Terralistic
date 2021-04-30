//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include "map.hpp"
#include "dev.hpp"
#include <random>

std::vector<map::uniqueItem> map::unique_items;

void map::initItems() {
    // all currently unique items
    unique_items.reserve(4);
    unique_items = {
        {"nothing",     0,  blockType::AIR        },
        {"stone",       99, blockType::STONE      },
        {"dirt",        99, blockType::DIRT       },
        {"stone_block", 99, blockType::STONE_BLOCK},
    };
}

void map::spawnItem(itemType item_id, int x, int y, short id) {
    static short curr_id = 0;
    if(id == -1)
        id = curr_id++;
    items.emplace_back();
    items.back().create(item_id, x, y, id);
}

map::item* map::getItemById(unsigned short id) {
    for(item& i : map::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id");
    return nullptr;
}

void map::item::create(itemType item_id_, int x_, int y_, unsigned short id_) {
    static std::random_device device;
    static std::mt19937 engine(device());
    velocity_x = (int)engine() % 100;
    velocity_y = -int(engine() % 100) - 50;
    
    x = x_ * 100;
    y = y_ * 100;
    id = id_;
    item_id = item_id_;
}

map::uniqueItem::uniqueItem(const std::string& name, unsigned short stack_size, map::blockType places) : name(name), stack_size(stack_size), places(places) {
    texture.setTexture(name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + name + ".png"));
    texture.scale = 2;
    text_texture.setTexture(gfx::renderText(name, {255, 255, 255}));
    text_texture.scale = 2;
}

map::uniqueItem& map::item::getUniqueItem() const {
    ASSERT((int)item_id >= 0 && (int)item_id < unique_items.size(), "item_id is not valid");
    return unique_items[(int)item_id];
}

void map::renderItems() {
    for(map::item& i : items)
        gfx::render(i.getUniqueItem().texture, short(i.x / 100 - view_x + gfx::getWindowWidth() / 2), short(i.y / 100 - view_y + gfx::getWindowHeight() / 2));
}
