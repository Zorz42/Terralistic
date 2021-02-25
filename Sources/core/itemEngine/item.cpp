//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#define FILENAME item
#define NAMESPACE itemEngine
#include "essential.hpp"

#include "itemEngine.hpp"
#include "blockEngine.hpp"
#include "dev.hpp"

#include <random>

std::random_device device;

itemEngine::item::item(itemType item_id, int x, int y, unsigned short id) : x(x * 100), y(y * 100), id(id), item_id(item_id) {
    std::mt19937 engine(device());
    velocity_x = (int)engine() % 200 - 100;
    velocity_y = -int(engine() % 100) - 50;
}

itemEngine::uniqueItem::uniqueItem(const std::string& name, unsigned short stack_size, blockEngine::blockType places) : name(name), stack_size(stack_size), places(places) {}

itemEngine::uniqueItem& itemEngine::item::getUniqueItem() const {
    ASSERT(item_id >= 0 && item_id < unique_items.size(), "item_id is not valid");
    return unique_items[item_id];
}

void itemEngine::item::update(float frame_length) {
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
    
    
}

bool itemEngine::item::colliding() const {
    int height_x = 1, height_y = 1;
    if(x / 100 % BLOCK_WIDTH)
        height_x++;
    if(y / 100 % BLOCK_WIDTH)
        height_y++;
    int block_x = x / 100 / BLOCK_WIDTH, block_y = y / 100 / BLOCK_WIDTH;
    for(int x_ = 0; x_ < height_x; x_++)
        for(int y_ = 0; y_ < height_y; y_++)
            if(!blockEngine::getBlock((unsigned short)(block_x + x_), (unsigned short)(block_y + y_)).getUniqueBlock().transparent)
                return true;
    return false;
}
