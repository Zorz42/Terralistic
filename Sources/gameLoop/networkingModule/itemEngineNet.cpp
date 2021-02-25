//
//  network.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/02/2021.
//

#define FILENAME itemEngineNet
#define NAMESPACE networkingModule
#include "essential.hpp"

#include "itemEngine.hpp"
#include "networkingModule.hpp"

// listen to item creation, destruction and movement

PACKET_LISTENER(packets::ITEM_CREATION)
    auto type = (itemEngine::itemType)packet.getChar();
    unsigned short id = packet.getUShort();
    int y = packet.getInt(), x = packet.getInt();
    itemEngine::spawnItem(type, x, y, id);
PACKET_LISTENER_END

PACKET_LISTENER(packets::ITEM_DELETION)
    unsigned short id = packet.getUShort();
    for(auto i = itemEngine::items.begin(); i != itemEngine::items.end(); i++)
        if(i->getId() == id) {
            itemEngine::items.erase(i);
            break;
        }
PACKET_LISTENER_END

PACKET_LISTENER(packets::ITEM_MOVEMENT)
    itemEngine::item* item = itemEngine::getItemById(packet.getUShort());
    item->y = packet.getInt();
    item->x = packet.getInt();
PACKET_LISTENER_END
