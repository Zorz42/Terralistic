//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/04/2021.
//

#include "serverMap.hpp"
#include "assert.hpp"
#include <random>
#include "serverNetworking.hpp"

std::vector<serverMap::uniqueItem> serverMap::unique_items;

void serverMap::initItems() {
    // all currently unique items
    serverMap::unique_items = {
        {"nothing",     0,  serverMap::blockType::AIR        },
        {"stone",       99, serverMap::blockType::STONE      },
        {"dirt",        99, serverMap::blockType::DIRT       },
        {"stone_block", 99, serverMap::blockType::STONE_BLOCK},
        {"wood_planks", 99, serverMap::blockType::SAND       },
    };
}

void serverMap::updateItems(float frame_length) {
    for(auto & item : items)
        item.update(frame_length, *this);
}

void serverMap::spawnItem(itemType item_id, int x, int y, short id) {
    static short curr_id = 0;
    if(id == -1)
        id = curr_id++;
    items.emplace_back();
    items.back().create(item_id, x, y, id, *this);
}

serverMap::item* serverMap::getItemById(unsigned short id) {
    for(item& i : serverMap::items)
        if(i.getId() == id)
            return &i;
    ASSERT(false, "item not found by id");
    return nullptr;
}

void serverMap::item::create(itemType item_id_, int x_, int y_, unsigned short id_, serverMap& world_serverMap) {
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
    world_serverMap.manager->sendToEveryone(packet);
}

void serverMap::item::destroy(serverMap& world_serverMap) {
    packets::packet packet(packets::ITEM_DELETION);
    packet << getId();
    world_serverMap.manager->sendToEveryone(packet);
}

serverMap::uniqueItem::uniqueItem(std::string  name, unsigned short stack_size, serverMap::blockType places) : name(std::move(name)), stack_size(stack_size), places(places) {}

serverMap::uniqueItem& serverMap::item::getUniqueItem() const {
    ASSERT((int)item_id >= 0 && (int)item_id < unique_items.size(), "item_id is not valid");
    return unique_items[(int)item_id];
}

void serverMap::item::update(float frame_length, serverMap& world_serverMap) {
    int prev_x = x, prev_y = y;
    
    // move and go back if colliding
    velocity_y += (int)frame_length / 16 * 5;
    for(int i = 0; i < frame_length / 16 * velocity_x; i++) {
        x++;
        if(colliding(world_serverMap)) {
            x--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_x; i--) {
        x--;
        if(colliding(world_serverMap)) {
            x++;
            break;
        }
    }
    for(int i = 0; i < frame_length / 16 * velocity_y; i++) {
        y++;
        if(colliding(world_serverMap)) {
            y--;
            break;
        }
    }
    for(int i = 0; i > frame_length / 16 * velocity_y; i--) {
        y--;
        if(colliding(world_serverMap)) {
            y++;
            break;
        }
    }
    
    y++;
    if(colliding(world_serverMap))
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
        world_serverMap.manager->sendToEveryone(packet);
    }
}

bool serverMap::item::colliding(serverMap& world_serverMap) const {
    int height_x = 1, height_y = 1;
    if(x / 100 % BLOCK_WIDTH)
        height_x++;
    if(y / 100 % BLOCK_WIDTH)
        height_y++;
    int block_x = x / 100 / BLOCK_WIDTH, block_y = y / 100 / BLOCK_WIDTH;
    for(int x_ = 0; x_ < height_x; x_++)
        for(int y_ = 0; y_ < height_y; y_++)
            if(!world_serverMap.getBlock((unsigned short)(block_x + x_), (unsigned short)(block_y + y_)).isTransparent())
                return true;
    return false;
}
