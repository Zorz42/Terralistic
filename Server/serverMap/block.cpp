//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "serverMap.hpp"
#include "assert.hpp"
#include "serverNetworking.hpp"

std::vector<serverMap::uniqueBlock> serverMap::unique_blocks;

void serverMap::initBlocks() {
    unique_blocks = {
        uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemType::NOTHING,     /*break_time*/1000       ),
        uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemType::DIRT,        /*break_time*/1000       ),
        uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemType::STONE_BLOCK, /*break_time*/1000       ),
        uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemType::NOTHING,     /*break_time*/1000       ),
        uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/itemType::STONE,       /*break_time*/1000       ),
        uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemType::NOTHING,     /*break_time*/1000       ),
        uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemType::NOTHING,     /*break_time*/UNBREAKABLE),
    };
}

serverMap::block serverMap::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth(), "requested block is out of bounds");
    return block(x, y, &blocks[y * getWorldWidth() + x], this);
}

void serverMap::block::setType(serverMap::blockType id, bool process) {
    if(!process)
        block_data->block_id = id;
    else if(id != block_data->block_id) {
        parent_serverMap->removeNaturalLight(x);
        block_data->block_id = id;
        parent_serverMap->setNaturalLight(x);
        update();
        
        // update upper, lower, right and left block (neighbours)
        if(x != 0)
            parent_serverMap->getBlock(x - 1, y).update();
        if(x != parent_serverMap->getWorldWidth() - 1)
            parent_serverMap->getBlock(x + 1, y).update();
        if(y != 0)
            parent_serverMap->getBlock(x, y - 1).update();
        if(y != parent_serverMap->getWorldHeight() - 1)
            parent_serverMap->getBlock(x, y + 1).update();
        
        scheduleLightUpdate();
        
        packets::packet packet(packets::BLOCK_CHANGE);
        packet << getX() << getY() << (unsigned char)getType();
        parent_serverMap->manager->sendToEveryone(packet);
    }
}

void serverMap::block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getBreakTime() * 9.0f);
    if(stage != getBreakStage()) {
        block_data->break_stage = stage;
        packets::packet packet(packets::BLOCK_PROGRESS_CHANGE);
        packet << getY() << getX() << getBreakStage();
        parent_serverMap->manager->sendToEveryone(packet);
    }
}

void serverMap::block::update() {
    if(isOnlyOnFloor() && parent_serverMap->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        breakBlock();
    scheduleLightUpdate();
}

void serverMap::block::breakBlock() {
    if(getDrop() != itemType::NOTHING)
        parent_serverMap->spawnItem(getDrop(), x * BLOCK_WIDTH, y * BLOCK_WIDTH);
    setType(blockType::AIR);
    setBreakProgress(0);
    if(block_data->getUniqueBlock().onBreak)
        block_data->getUniqueBlock().onBreak(this);
}

serverMap::uniqueBlock& serverMap::blockData::getUniqueBlock() const {
    return unique_blocks[(int)block_id];
}
