//
//  inventoryItem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/01/2021.
//

#include "inventory.hpp"
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"

inventory::inventoryItem::inventoryItem() : item_id(itemEngine::NOTHING), stack(0) {}

itemEngine::uniqueItem& inventory::inventoryItem::getUniqueItem() const {
    return itemEngine::unique_items.at(item_id);
}

void inventory::inventoryItem::render(int x, int y) {
    if(getUniqueItem().texture != nullptr)
        swl::render(getUniqueItem().texture, {x, y, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
    if(stack > 1) {
        stack_texture.setX(short(x + BLOCK_WIDTH * 2 - stack_texture.getWidth()));
        stack_texture.setY(short(y + BLOCK_WIDTH * 2 - stack_texture.getHeight()));
        stack_texture.render();
    }
}

void inventory::inventoryItem::setStack(unsigned short stack_) {
    stack = stack_;
    if(stack > 1)
        stack_texture.loadFromText(std::to_string(stack_), {255, 255, 255});
    else if(!stack)
        item_id = itemEngine::NOTHING;
}

unsigned short inventory::inventoryItem::getStack() const {
    return stack;
}

unsigned short inventory::inventoryItem::increaseStack(unsigned short stack_) {
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

bool inventory::inventoryItem::decreaseStack(unsigned short stack_) {
    if(stack_ > stack)
        return false;
    else {
        setStack(stack - stack_);
        return true;
    }
}

