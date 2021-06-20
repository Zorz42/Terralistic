//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

// inventory is a class which you can easily manage with function calls

#include "inventory.hpp"

char inventory::addItem(map::itemType id, int quantity) {
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

void inventory::swapWithMouseItem(inventoryItem* item) {
    inventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

inventoryItem* inventory::getMouseItem() {
    return &mouse_item;
}

void inventory::clearMouseItem() {
    mouse_item.item_id = map::itemType::NOTHING;
    mouse_item.setStack(0);
}
