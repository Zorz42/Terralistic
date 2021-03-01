//
//  clickEvents.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#define FILENAME clickEvents
#define NAMESPACE blockEngine
#include "core.hpp"

#include "playerHandler.hpp"
#include "gameLoop.hpp"
#include "blockRenderer.hpp"

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
    blockEngine::unique_blocks[blockEngine::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    blockEngine::unique_blocks[blockEngine::AIR].rightClickEvent = &air_rightClickEvent;
    blockEngine::unique_blocks[blockEngine::AIR].leftClickEvent = &air_leftClickEvent;
INIT_SCRIPT_END
