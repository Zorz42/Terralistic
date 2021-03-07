//
//  clickEvents.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#include "core.hpp"

#include "playerHandler.hpp"
#include "gameLoop.hpp"
#include "blockRenderer.hpp"
#include "blockSelector.hpp"

// you can register special click events to blocks for custom behaviour
void grass_block_leftClickEvent(blockEngine::block* block) {
    block->setBlockType(blockEngine::DIRT);
}

void air_rightClickEvent(blockEngine::block* block) {
    blockEngine::blockType type = playerHandler::player_inventory.getSelectedSlot()->getUniqueItem().places;
    if(type != blockEngine::AIR && playerHandler::player_inventory.getSelectedSlot()->decreaseStack(1)) {
        block->setBlockType(type);
        block->light_update();
    }
}

void air_leftClickEvent(blockEngine::block* block) {}

INIT_SCRIPT
    INIT_ASSERT(blockEngine::unique_blocks.size());
    blockSelector::click_events = std::vector<blockSelector::clickEvents>(blockEngine::unique_blocks.size());

    blockSelector::click_events[blockEngine::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    blockSelector::click_events[blockEngine::AIR].rightClickEvent = &air_rightClickEvent;
    blockSelector::click_events[blockEngine::AIR].leftClickEvent = &air_leftClickEvent;
INIT_SCRIPT_END
