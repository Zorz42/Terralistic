//
//  inventoryItem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/01/2021.
//

#include "inventory.hpp"
#include "blockEngine.hpp"

inventory::inventoryItem::inventoryItem() : item_id(itemEngine::NOTHING), stack(0) {}

itemEngine::uniqueItem& inventory::inventoryItem::getUniqueItem() const {
    return itemEngine::unique_items[item_id];
}

void inventory::inventoryItem::setStack(unsigned short stack_) {
    stack = stack_;
    if(!stack)
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

inventory::inventoryItem& inventory::inventoryItem::operator=(const inventoryItem& item) {
    item_id = item.item_id;
    setStack(item.stack);
    return *this;
}
