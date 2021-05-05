//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

// inventory is a class which you can easily manage with function calls

#include "map.hpp"

map::inventoryItem::inventoryItem() : item_id(map::itemType::NOTHING), stack(0) {}

map::uniqueItem& map::inventoryItem::getUniqueItem() const {
    // unique item holds properties which all items of the same type share
    return map::unique_items[(int)item_id];
}

void map::inventoryItem::setStack(unsigned short stack_) {
    // just upadate to nothing if stack reaches 0
    if(stack != stack_) {
        stack = stack_;
        if(!stack)
            item_id = map::itemType::NOTHING;
        stack_changed = true;
    }
}

unsigned short map::inventoryItem::getStack() const {
    return stack;
}

unsigned short map::inventoryItem::increaseStack(unsigned short stack_) {
    // increase stack by stack_ and if it reaches limit, return what left. example: stack_limit is 99, current stack is 40, you increase stack by 100. current stack becomes 99 and increase stack returns 41
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

bool map::inventoryItem::decreaseStack(unsigned short stack_) {
    // returns true if stack can be decreased
    if(stack_ > stack)
        return false;
    else {
        setStack(stack - stack_);
        return true;
    }
}


char map::inventory::addItem(map::itemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == map::itemType::NOTHING) {
            inventory[i].item_id = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

map::inventoryItem* map::inventory::getSelectedSlot() {
    return &inventory[(int)(unsigned char)selected_slot];
}

void map::inventory::swapWithMouseItem(map::inventoryItem* item) {
    map::inventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

map::inventoryItem* map::inventory::getMouseItem() {
    return &mouse_item;
}

void map::inventory::clearMouseItem() {
    mouse_item.item_id = map::itemType::NOTHING;
    mouse_item.setStack(0);
}

void map::inventory::clear() {
    for(inventoryItem& i : inventory)
        i.setStack(0);
}
