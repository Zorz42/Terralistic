//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

// inventory is a class which you can easily manage with function calls

#include "inventory.hpp"

char clientInventory::addItem(itemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == itemType::NOTHING) {
            inventory[i].item_id = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

void clientInventory::swapWithMouseItem(clientInventoryItem* item) {
    clientInventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

clientInventoryItem* clientInventory::getMouseItem() {
    return &mouse_item;
}

void clientInventory::clearMouseItem() {
    mouse_item.item_id = itemType::NOTHING;
    mouse_item.setStack(0);
}
