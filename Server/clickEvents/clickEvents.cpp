//
//  clickEvents.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 07/03/2021.
//

#include "core.hpp"

#include "clickEvents.hpp"

void grass_block_leftClickEvent(blockEngine::block* block, playerHandler::player* player) {
    block->setBlockType(blockEngine::DIRT);
}

void air_rightClickEvent(blockEngine::block* block, playerHandler::player* player) {
    blockEngine::blockType type = player->inventory.inventory[player->inventory.selected_slot].getUniqueItem().places;
    if(type != blockEngine::AIR && player->inventory.inventory[player->inventory.selected_slot].decreaseStack(1)) {
        packets::packet item_loss_packet(packets::INVENTORY_CHANGE);
        item_loss_packet << (unsigned char)player->inventory.inventory[player->inventory.selected_slot].item_id << (unsigned short)player->inventory.inventory[player->inventory.selected_slot].getStack() << (char)player->inventory.selected_slot;
        player->conn->sendPacket(item_loss_packet);
        block->setBlockType(type);
        block->update();
    }
}

void air_leftClickEvent(blockEngine::block* block, playerHandler::player* player) {}

INIT_SCRIPT
    INIT_ASSERT(blockEngine::unique_blocks.size());
    clickEvents::click_events = std::vector<clickEvents::clickEvents>(blockEngine::unique_blocks.size());

    clickEvents::click_events[blockEngine::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    clickEvents::click_events[blockEngine::AIR].rightClickEvent = &air_rightClickEvent;
    clickEvents::click_events[blockEngine::AIR].leftClickEvent = &air_leftClickEvent;
INIT_SCRIPT_END
