//
//  network.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/02/2021.
//

#include "itemEngine.hpp"
#include "networkingModule.hpp"

void itemCreationListener(packets::packet& packet) {
    auto type = (itemEngine::itemType)packet.getChar();
    unsigned short id = packet.getUShort();
    int y = packet.getInt(), x = packet.getInt();
    itemEngine::spawnItem(type, x, y);
    itemEngine::items.back().id = id;
}

void itemDeletionListener(packets::packet& packet) {
    unsigned short id = packet.getUShort();
    for(auto i = itemEngine::items.begin(); i != itemEngine::items.end(); i++)
        if(i->id == id) {
            itemEngine::items.erase(i);
            break;
        }
}

void itemMovementListener(packets::packet& packet) {
    unsigned short id = packet.getUShort();
    int y = packet.getInt(), x = packet.getInt();
    for(auto & item : itemEngine::items)
        if(item.id == id) {
            item.x = x;
            item.y = y;
            break;
        }
}

networking::registerListener item_creation_listener(itemCreationListener, packets::ITEM_CREATION), item_deletion_listener(itemDeletionListener, packets::ITEM_DELETION), item_movement_listener(itemMovementListener, packets::ITEM_MOVEMENT);
