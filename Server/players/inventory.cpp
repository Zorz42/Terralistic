//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

// inventory is a class which you can easily manage with function calls

#include "players.hpp"
#include "properties.hpp"

void inventoryItem::setId(ItemType id) {
    if(item_id != id) {
        ServerInventoryItemTypeChangeEvent event(*this, id);
        event.call();
        
        if(event.cancelled)
            return;
        
        item_id = id;
    }
}

const ItemInfo& inventoryItem::getUniqueItem() const {
    // unique item holds properties which all items of the same type share
    return ::getItemInfo(item_id);
}

void inventoryItem::setStack(unsigned short stack_) {
    // just update to nothing if stack reaches 0
    if(stack != stack_) {
        ServerInventoryItemStackChangeEvent event(*this, stack_);
        event.call();
        
        if(event.cancelled)
            return;
        
        stack = stack_;
        if(!stack)
            setId(ItemType::NOTHING);
    }
}

unsigned short inventoryItem::getStack() const {
    return stack;
}

unsigned short inventoryItem::increaseStack(unsigned short stack_) {
    // increase stack by stack_ and if it reaches limit, return what left. example: stack_limit is 99, current stack is 40, you increase stack by 100. current stack becomes 99 and increase stack returns 41
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

bool inventoryItem::decreaseStack(unsigned short stack_) {
    // returns true if stack can be decreased
    if(stack_ > stack)
        return false;
    else {
        setStack(stack - stack_);
        return true;
    }
}

unsigned char inventoryItem::getPosInInventory() {
    return this - &holder->inventory_arr[0];
}

inventory::inventory(player* owner) : owner(owner) {
    for(inventoryItem& i : inventory_arr)
        i = inventoryItem(this);
}

char inventory::addItem(ItemType id, int quantity) {
    // adds item to inventory
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getId() == id) {
            quantity -= inventory_arr[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getId() == ItemType::NOTHING) {
            inventory_arr[i].setId(id);
            quantity -= inventory_arr[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

inventoryItem* inventory::getSelectedSlot() {
    return &inventory_arr[(int)(unsigned char)selected_slot];
}

void inventory::swapWithMouseItem(inventoryItem* item) {
    inventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}
