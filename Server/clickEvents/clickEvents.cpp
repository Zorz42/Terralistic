//
//  clickEvents.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 07/03/2021.
//

#include "clickEvents.hpp"
#include "packets.hpp"

void grass_block_leftClickEvent(serverMap::block* block, serverMap::player* player) {
    block->setType(serverMap::blockType::DIRT);
}

void air_rightClickEvent(serverMap::block* block, serverMap::player* player) {
    serverMap::blockType type = player->inventory.inventory[player->inventory.selected_slot].getUniqueItem().places;
    if(type != serverMap::blockType::AIR && player->inventory.inventory[player->inventory.selected_slot].decreaseStack(1)) {
        packets::packet item_loss_packet(packets::INVENTORY_CHANGE);
        item_loss_packet << (unsigned char)player->inventory.inventory[player->inventory.selected_slot].item_id << (unsigned short)player->inventory.inventory[player->inventory.selected_slot].getStack() << (char)player->inventory.selected_slot;
        player->conn->sendPacket(item_loss_packet);
        block->setType(type);
        block->update();
    }
}

void air_leftClickEvent(serverMap::block* block, serverMap::player* player) {}

void clickEvents::init() {
    click_events = std::vector<clickEvents::clickEvent>(serverMap::unique_blocks.size());

    click_events[(int)serverMap::blockType::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    click_events[(int)serverMap::blockType::AIR].rightClickEvent = &air_rightClickEvent;
    click_events[(int)serverMap::blockType::AIR].leftClickEvent = &air_leftClickEvent;
}
