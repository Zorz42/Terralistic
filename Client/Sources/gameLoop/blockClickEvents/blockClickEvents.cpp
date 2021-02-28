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
void grass_block_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    block->setBlockType(blockEngine::DIRT);
}

void air_rightClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    blockEngine::blockType type = playerHandler::player_inventory.getSelectedSlot()->getUniqueItem().places;
    if(type != blockEngine::AIR && playerHandler::player_inventory.getSelectedSlot()->decreaseStack(1)) {
        blockEngine::removeNaturalLight(x);
        block->setBlockType(type);
        blockEngine::setNaturalLight(x);
        if(!gameLoop::online) {
            blockEngine::getBlock(x, y).update();
            blockEngine::updateNeighbours(x, y);
        }
        blockEngine::getBlock(x, y).light_update();
    }
}

void air_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {}

INIT_SCRIPT
    INIT_ASSERT(blockEngine::unique_blocks.size());
    blockEngine::unique_blocks[blockEngine::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    blockEngine::unique_blocks[blockEngine::AIR].rightClickEvent = &air_rightClickEvent;
    blockEngine::unique_blocks[blockEngine::AIR].leftClickEvent = &air_leftClickEvent;
INIT_SCRIPT_END
