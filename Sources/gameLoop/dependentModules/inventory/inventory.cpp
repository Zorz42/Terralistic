//
//  inventory.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#include "inventory.hpp"
#include "blockEngine.hpp"
#include "playerHandler.hpp"

bool inventory::inventory::addItem(itemEngine::itemType id, int quantity) {
    for(auto & i : inventory)
        if(i.item_id == id) {
            quantity -= i.increaseStack((unsigned short)quantity);
            if(!quantity)
                return true;
        }
    for(auto & i : inventory)
        if(i.item_id == itemEngine::NOTHING) {
            i.item_id = id;
            quantity -= i.increaseStack((unsigned short)quantity);
            if(!quantity)
                return true;
        }
    return false;
}
