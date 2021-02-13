//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#include "inventory.hpp"
#include "blockEngine.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"

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
