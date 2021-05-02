//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include "map.hpp"
#include "dev.hpp"
#include <random>
#include "networkingModule.hpp"

std::vector<map::uniqueItem> map::unique_items;

void map::initItems() {
    // all currently unique items
    map::unique_items = {
        {"nothing",     0,  map::blockType::AIR        },
        {"stone",       99, map::blockType::STONE      },
        {"dirt",        99, map::blockType::DIRT       },
        {"stone_block", 99, map::blockType::STONE_BLOCK},
    };
}

void map::updateItems(float frame_length) {
    for(auto & item : items)
        item.update(frame_length, *this);
}

void map::spawnItem(itemType item_id, int x, int y, short id) {
    static short curr_id = 0;
    if(id == -1)
        id = curr_id++;
    items.emplace_back();
    items.back().create(item_id, x, y, id, *this);
}

map::item* map::getItemById(unsigned short id) {
    for(item& i : map::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id");
    return nullptr;
}

void map::item::create(itemType item_id_, int x_, int y_, unsigned short id_, map& world_map) {
    static std::random_device device;
    static std::mt19937 engine(device());
    velocity_x = (int)engine() % 100;
    velocity_y = -int(engine() % 100) - 50;
    
    x = x_ * 100;
    y = y_ * 100;
    id = id_;
    item_id = item_id_;
    
    packets::packet packet(packets::ITEM_CREATION);
    packet << x << y << getId() << (char)getItemId();
    networking::sendToEveryone(packet);
}

void map::item::destroy(map& world_map) {
    packets::packet packet(packets::ITEM_DELETION);
    packet << getId();
    networking::sendToEveryone(packet);
}

map::uniqueItem::uniqueItem(std::string  name, unsigned short stack_size, map::blockType places) : name(std::move(name)), stack_size(stack_size), places(places) {}

map::uniqueItem& map::item::getUniqueItem() const {
    ASSERT((int)item_id >= 0 && (int)item_id < unique_items.size(), "item_id is not valid");
    return unique_items[(int)item_id];
}

void map::item::update(float frame_length, map& world_map) {
    int prev_x = x, prev_y = y;
    
    // move and go back if colliding
    velocity_y += (int)frame_length / 16 * 5;
    for(int i = 0; i < frame_length / 16 * velocity_x; i++) {
        x++;
        if(colliding(world_map)) {
            x--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_x; i--) {
        x--;
        if(colliding(world_map)) {
            x++;
            break;
        }
    }
    for(int i = 0; i < frame_length / 16 * velocity_y; i++) {
        y++;
        if(colliding(world_map)) {
            y--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_y; i--) {
        y--;
        if(colliding(world_map)) {
            y++;
            break;
        }
    }
    
    y++;
    if(colliding(world_map))
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
        packets::packet packet(packets::ITEM_MOVEMENT);
        packet << x << y << getId();
        networking::sendToEveryone(packet);
    }
}

bool map::item::colliding(map& world_map) const {
    int height_x = 1, height_y = 1;
    if(x / 100 % BLOCK_WIDTH)
        height_x++;
    if(y / 100 % BLOCK_WIDTH)
        height_y++;
    int block_x = x / 100 / BLOCK_WIDTH, block_y = y / 100 / BLOCK_WIDTH;
    for(int x_ = 0; x_ < height_x; x_++)
        for(int y_ = 0; y_ < height_y; y_++)
            if(!world_map.getBlock((unsigned short)(block_x + x_), (unsigned short)(block_y + y_)).isTransparent())
                return true;
    return false;
}
