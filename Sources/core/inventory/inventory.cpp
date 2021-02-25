//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#define FILENAME inventory
#define NAMESPACE inventory
#include "essential.hpp"

#include "inventory.hpp"
#include "blockEngine.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"

// inventory is a class which you can easily manage with function calls

char inventory::inventory::addItem(itemEngine::itemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == itemEngine::NOTHING) {
            inventory[i].item_id = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return i;
        }
    return -1;
}

inventory::inventoryItem* inventory::inventory::getSelectedSlot() {
    return &inventory[(int)selected_slot];
}

void inventory::inventory::swapWithMouseItem(inventoryItem* item) {
    inventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

inventory::inventoryItem* inventory::inventory::getMouseItem() {
    return &mouse_item;
}

void inventory::inventory::clearMouseItem() {
    mouse_item.item_id = itemEngine::NOTHING;
    mouse_item.setStack(0);
}
