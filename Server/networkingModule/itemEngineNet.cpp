//
//  item.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 26/01/2021.
//

#define FILENAME itemEngineNet
#define NAMESPACE networkingModule
#include "core.hpp"

#include "networkingModule.hpp"
#include "print.hpp"

unsigned short curr_id_item = 0;

EVENT_LISTENER(itemEngine::item_creation)
    packets::packet packet(packets::ITEM_CREATION);
    packet << data.item->x << data.item->y << data.item->getId() << (char)data.item->getItemId();
    networking::sendToEveryone(packet);
EVENT_LISTENER_END

EVENT_LISTENER(itemEngine::item_deletion)
    packets::packet packet(packets::ITEM_DELETION);
    packet << data.item->getId();
    networking::sendToEveryone(packet);
EVENT_LISTENER_END

EVENT_LISTENER(itemEngine::item_movement)
    packets::packet packet(packets::ITEM_MOVEMENT);
    packet << data.item->x << data.item->y << data.item->getId();
    networking::sendToEveryone(packet);
EVENT_LISTENER_END

