//
//  inventoryItem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/01/2021.
//

#include "inventory.hpp"
#include "blockEngine.hpp"
#include "dev.hpp"

// inventoryItem is a class which exists in inventory

inventory::inventoryItem::inventoryItem() : item_id(itemEngine::NOTHING), stack(0) {}

itemEngine::uniqueItem& inventory::inventoryItem::getUniqueItem() const {
    // unique item holds properties which all items of the same type share
    ASSERT(item_id >= 0 && item_id < itemEngine::unique_items.size(), "item_id is not valid");
    return itemEngine::unique_items[item_id];
}

void inventory::inventoryItem::setStack(unsigned short stack_) {
    // just upadate to nothing if stack reaches 0
    if(stack != stack_) {
        stack = stack_;
        if(!stack)
            item_id = itemEngine::NOTHING;
        stack_changed = true;
    }
}

unsigned short inventory::inventoryItem::getStack() const {
    return stack;
}

unsigned short inventory::inventoryItem::increaseStack(unsigned short stack_) {
    // increase stack by stack_ and if it reaches limit, return what left. example: stack_limit is 99, current stack is 40, you increase stack by 100. current stack becomes 99 and increase stack returns 41
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

bool inventory::inventoryItem::decreaseStack(unsigned short stack_) {
    // returns true if stack can be decreased
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
