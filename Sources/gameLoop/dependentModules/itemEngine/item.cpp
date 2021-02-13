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
#include "playerHandler.hpp"

#include <random>

std::random_device device;

itemEngine::item::item(itemType item_id, int x, int y) : item_id(item_id), x(x * 100), y(y * 100) {
    std::mt19937 engine(device());
    velocity_x = (int)engine() % 200 - 100;
    velocity_y = -int(engine() % 100) - 50;
}

itemEngine::uniqueItem::uniqueItem(const std::string& name, unsigned short stack_size, blockEngine::blockType places) : name(name), texture(name == "nothing" ? nullptr : swl::loadTextureFromFile("texturePack/items/" + name + ".png")), stack_size(stack_size), places(places) {
    text_texture.loadFromText(name, {255, 255, 255});
    text_texture.scale = 2;
    text_texture.free_texture = false;
}

void itemEngine::item::draw() const {
    swl::render(getUniqueItem().texture, getRect());
}

swl::rect itemEngine::item::getRect() const {
    return {short(x / 100 - playerHandler::view_x + swl::window_width / 2), short(y / 100 - playerHandler::view_y + swl::window_height / 2), BLOCK_WIDTH, BLOCK_WIDTH};
}

itemEngine::uniqueItem& itemEngine::item::getUniqueItem() const {
    return unique_items[item_id];
}

void itemEngine::item::update() {
    velocity_y += (int)framerateRegulator::frame_length / 16 * 5;
    for(int i = 0; i < framerateRegulator::frame_length / 16 * velocity_x; i++) {
        x++;
        if(colliding()) {
            x--;
            break;
        }
    }
    for(int i = 0; i > framerateRegulator::frame_length / 16 * velocity_x; i--) {
        x--;
        if(colliding()) {
            x++;
            break;
        }
    }
    for(int i = 0; i < framerateRegulator::frame_length / 16 * velocity_y; i++) {
        y++;
        if(colliding()) {
            y--;
            break;
        }
    }
    for(int i = 0; i > framerateRegulator::frame_length / 16 * velocity_y; i--) {
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
        velocity_x -= framerateRegulator::frame_length / 8;
        if(velocity_x < 0)
            velocity_x = 0;
    }
    else if(velocity_x < 0) {
        velocity_x += framerateRegulator::frame_length / 8;
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
