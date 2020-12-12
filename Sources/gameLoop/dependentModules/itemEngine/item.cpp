//
//  item.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#include "itemEngine.hpp"
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "framerateRegulator.hpp"

itemEngine::item::item(itemType item_id, int x, int y) : item_id(item_id), x(x * 100), y(y * 100), velocity_x(rand() % 100 - 50), velocity_y(-rand() % 100 - 20) {}

void itemEngine::item::draw() {
    swl::render(getUniqueItem().texture, getRect());
}

SDL_Rect itemEngine::item::getRect() {
    return {int(x / 100 - blockEngine::view_x + swl::window_width / 2), int(y / 100 - blockEngine::view_y + swl::window_height / 2), BLOCK_WIDTH, BLOCK_WIDTH};
}

itemEngine::uniqueItem& itemEngine::item::getUniqueItem() {
    return unique_items.at(item_id);
}

void itemEngine::item::update() {
    velocity_y += (float)framerateRegulator::frame_length / 16 * 5;
    for(int i = 0; i < (float)framerateRegulator::frame_length / 16 * velocity_x; i++) {
        x++;
        if(colliding()) {
            x--;
            break;
        }
    }
    for(int i = 0; i > (float)framerateRegulator::frame_length / 16 * velocity_x; i--) {
        x--;
        if(colliding()) {
            x++;
            break;
        }
    }
    for(int i = 0; i < (float)framerateRegulator::frame_length / 16 * velocity_y; i++) {
        y++;
        if(colliding()) {
            y--;
            break;
        }
    }
    for(int i = 0; i > (float)framerateRegulator::frame_length / 16 * velocity_y; i--) {
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
        velocity_x -= (float)framerateRegulator::frame_length / 8;
        if(velocity_x < 0)
            velocity_x = 0;
    }
    else if(velocity_x < 0) {
        velocity_x += (float)framerateRegulator::frame_length / 8;
        if(velocity_x > 0)
            velocity_x = 0;
    }
}

bool itemEngine::item::colliding() {
    int height_x = 1, height_y = 1;
    if(x / 100 % BLOCK_WIDTH)
        height_x++;
    if(y / 100 % BLOCK_WIDTH)
        height_y++;
    int block_x = x / 100 / BLOCK_WIDTH, block_y = y / 100 / BLOCK_WIDTH;
    for(int x_ = 0; x_ < height_x; x_++)
        for(int y_ = 0; y_ < height_y; y_++)
            if(!blockEngine::getBlock(block_x + x_, block_y + y_).getUniqueBlock().transparent)
                return true;
    return false;
}
