#include "players.hpp"
#include "properties.hpp"

void InventoryItem::setTypeWithoutProcessing(ItemType type_) {
    type = type_;
}

void InventoryItem::setType(ItemType type_) {
    if(type != type_) {
        ServerInventoryItemTypeChangeEvent event(*this, type_);
        event.call();
        
        if(event.cancelled)
            return;
        
        setTypeWithoutProcessing(type_);
    }
}

const ItemInfo& InventoryItem::getUniqueItem() const {
    return ::getItemInfo(type);
}

void InventoryItem::setStackWithoutProcessing(unsigned short stack_) {
    stack = stack_;
}

void InventoryItem::setStack(unsigned short stack_) {
    if(stack != stack_) {
        ServerInventoryItemStackChangeEvent event(*this, stack_);
        event.call();
        
        if(event.cancelled)
            return;
        
        setStackWithoutProcessing(stack_);
        if(!stack)
            setType(ItemType::NOTHING);
    }
}

unsigned short InventoryItem::getStack() const {
    return stack;
}

unsigned short InventoryItem::increaseStack(unsigned short stack_) {
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

bool InventoryItem::decreaseStack(unsigned short stack_) {
    if(stack_ > stack)
        return false;
    else {
        setStack(stack - stack_);
        return true;
    }
}

unsigned char InventoryItem::getPosInInventory() {
    return this - &inventory->inventory_arr[0];
}

Inventory::Inventory(Player* owner) : player(owner) {
    for(InventoryItem& i : inventory_arr)
        i = InventoryItem(this);
}

char Inventory::addItem(ItemType id, int quantity) {
    // adds item to inventory
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getType() == id) {
            quantity -= inventory_arr[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getType() == ItemType::NOTHING) {
            inventory_arr[i].setType(id);
            quantity -= inventory_arr[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

InventoryItem* Inventory::getSelectedSlot() {
    return &inventory_arr[(int)(unsigned char)selected_slot];
}

void Inventory::swapWithMouseItem(InventoryItem* item) {
    InventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}
