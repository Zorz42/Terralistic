//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include "core.hpp"

INIT_SCRIPT
    // all currently unique items
    itemEngine::unique_items = {
        {"nothing",     0,  scene->world_map.AIR        },
        {"stone",       99, scene->world_map.STONE      },
        {"dirt",        99, scene->world_map.DIRT       },
        {"stone_block", 99, scene->world_map.STONE_BLOCK},
    };
INIT_SCRIPT_END

void itemEngine::updateItems(float frame_length) {
    for(auto & item : items)
        item.update(frame_length);
}

void itemEngine::spawnItem(itemType item_id, int x, int y, unsigned short id) {
    static short curr_id = 0;
    if(id == -1)
        id = curr_id++;
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

void map::item::create(itemType item_id_, int x_, int y_, unsigned short id_) {
    static std::random_device device;
    static std::mt19937 engine(device());
    velocity_x = (int)engine() % 100;
    velocity_y = -int(engine() % 100) - 50;
    
    x = x_ * 100;
    y = y_ * 100;
    id = id_;
    item_id = item_id_;
    
    item_creation_data data{};
    data.item = this;
    events::callEvent(item_creation, (void*)&data);
}

void map::item::destroy() {
    item_deletion_data data{};
    data.item = this;
    events::callEvent(item_deletion, (void*)&data);
}

map::uniqueItem::uniqueItem(std::string  name, unsigned short stack_size, map::blockType places) : name(std::move(name)), stack_size(stack_size), places(places) {}

map::uniqueItem& itemEngine::item::getUniqueItem() const {
    ASSERT(item_id >= 0 && item_id < unique_items.size(), "item_id is not valid");
    return unique_items[item_id];
}

void map::item::update(float frame_length) {
    int prev_x = x, prev_y = y;
    
    // move and go back if colliding
    velocity_y += (int)frame_length / 16 * 5;
    for(int i = 0; i < frame_length / 16 * velocity_x; i++) {
        x++;
        if(colliding()) {
            x--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_x; i--) {
        x--;
        if(colliding()) {
            x++;
            break;
        }
    }
    for(int i = 0; i < frame_length / 16 * velocity_y; i++) {
        y++;
        if(colliding()) {
            y--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_y; i--) {
        y--;
        if(colliding()) {
            y++;
            break;
        }
    }
    
    y++;
    if(colliding())
        velocity_y = 0;
    y--;
    
    if(velocity_x > 0) {
        velocity_x -= frame_length / 8;
        if(velocity_x < 0)
            velocity_x = 0;
    }
    else if(velocity_x < 0) {
        velocity_x += frame_length / 8;
        if(velocity_x > 0)
            velocity_x = 0;
    }
    
    if(prev_x != x || prev_y != y) {
        item_movement_data data{};
        data.item = this;
        events::callEvent(item_movement, (void*)&data);
    }
}

bool map::item::colliding() const {
    int height_x = 1, height_y = 1;
    if(x / 100 % BLOCK_WIDTH)
        height_x++;
    if(y / 100 % BLOCK_WIDTH)
        height_y++;
    int block_x = x / 100 / BLOCK_WIDTH, block_y = y / 100 / BLOCK_WIDTH;
    for(int x_ = 0; x_ < height_x; x_++)
        for(int y_ = 0; y_ < height_y; y_++)
            if(!scene->world_map.getBlock((unsigned short)(block_x + x_), (unsigned short)(block_y + y_)).getUniqueBlock().transparent)
                return true;
    return false;
}
