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

itemEngine::uniqueItem::uniqueItem(std::string name, unsigned short stack_size) : name(name), stack_size(stack_size), texture(name == "nothing" ? nullptr : swl::loadTextureFromFile("texturePack/items/" + name + ".png")) {}

itemEngine::inventoryItem::inventoryItem() : item_id(NOTHING), stack(0) {}

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

itemEngine::uniqueItem& itemEngine::inventoryItem::getUniqueItem() {
    return unique_items.at(item_id);
}

void itemEngine::inventoryItem::render(int x, int y) {
    if(getUniqueItem().texture != nullptr)
        swl::render(getUniqueItem().texture, {x, y, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
    if(stack > 1) {
        stack_texture.setX(x + BLOCK_WIDTH * 2 - stack_texture.getWidth());
        stack_texture.setY(y + BLOCK_WIDTH * 2 - stack_texture.getHeight());
        stack_texture.render();
    }
}

void itemEngine::inventoryItem::setStack(unsigned short stack_) {
    stack = stack_;
    if(stack > 1)
        stack_texture.loadFromText(std::to_string(stack_), {255, 255, 255});
}

unsigned short itemEngine::inventoryItem::getStack() {
    return stack;
}

unsigned short itemEngine::inventoryItem::increaseStack(int stack_) {
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack(stack_to_be);
    return result;
}
