//
//  clickEvents.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 07/03/2021.
//

#include "clickEvents.hpp"
#include "packets.hpp"

void grass_block_leftClickEvent(map::block* block, player* player) {
    block->setType(map::blockType::DIRT);
}

void air_rightClickEvent(map::block* block, player* player) {
    map::blockType type = player->inventory.inventory[player->inventory.selected_slot].getUniqueItem().places;
    if(type != map::blockType::AIR && player->inventory.inventory[player->inventory.selected_slot].decreaseStack(1)) {
        packets::packet item_loss_packet(packets::INVENTORY_CHANGE);
        item_loss_packet << (unsigned char)player->inventory.inventory[player->inventory.selected_slot].item_id << (unsigned short)player->inventory.inventory[player->inventory.selected_slot].getStack() << (char)player->inventory.selected_slot;
        player->conn->sendPacket(item_loss_packet);
        block->setType(type);
        block->update();
    }
}

void air_leftClickEvent(map::block* block, player* player) {}

void clickEvents::init() {
    click_events = std::vector<clickEvents::clickEvent>(map::unique_blocks.size());

    click_events[(int)map::blockType::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    click_events[(int)map::blockType::AIR].rightClickEvent = &air_rightClickEvent;
    click_events[(int)map::blockType::AIR].leftClickEvent = &air_leftClickEvent;
}
