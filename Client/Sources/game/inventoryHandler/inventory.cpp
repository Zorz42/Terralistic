//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

// inventory is a class which you can easily manage with function calls

#include "inventoryHandler.hpp"

char ClientInventory::addItem(ItemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == ItemType::NOTHING) {
            inventory[i].item_id = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

void ClientInventory::swapWithMouseItem(ClientInventoryItem* item) {
    ClientInventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

ClientInventoryItem* ClientInventory::getMouseItem() {
    return &mouse_item;
}

void ClientInventory::clearMouseItem() {
    mouse_item.item_id = ItemType::NOTHING;
    mouse_item.setStack(0);
}
