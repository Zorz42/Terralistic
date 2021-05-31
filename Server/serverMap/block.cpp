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
        uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemType::WOOD_PLANKS, /*break_time*/1000       ),
        uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemType::NOTHING,     /*break_time*/UNBREAKABLE),
    };
    
    unique_blocks[(int)blockType::WOOD].onBreak = [](serverMap* world_map, block* this_block) {
        block blocks[] = {world_map->getBlock(this_block->getX(), this_block->getY() - 1), world_map->getBlock(this_block->getX() + 1, this_block->getY()), world_map->getBlock(this_block->getX() - 1, this_block->getY())};
        for(block& i : blocks)
            if(i.getType() == blockType::WOOD || i.getType() == blockType::LEAVES)
                i.breakBlock();
    };
    
    unique_blocks[(int)blockType::LEAVES].onBreak = unique_blocks[(int)blockType::WOOD].onBreak;
    
    unique_blocks[(int)blockType::GRASS_BLOCK].onLeftClick = [](block* this_block, player* peer) {
        this_block->setType(serverMap::blockType::DIRT);
    };
    
    unique_blocks[(int)blockType::AIR].onLeftClick = [](block* this_block, player* peer) {};
    
    unique_blocks[(int)blockType::AIR].onRightClick = [](block* this_block, player* peer) {
        serverMap::blockType type = peer->inventory.getSelectedSlot()->getUniqueItem().places;
        if(type != serverMap::blockType::AIR && peer->inventory.inventory_arr[peer->inventory.selected_slot].decreaseStack(1)) {
            this_block->setType(type);
            this_block->update();
        }
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
        parent_map->removeNaturalLight(x);
        block_data->block_id = id;
        parent_map->setNaturalLight(x);
        update();
        
        // update upper, lower, right and left block (neighbours)
        if(x != 0)
            parent_map->getBlock(x - 1, y).update();
        if(x != parent_map->getWorldWidth() - 1)
            parent_map->getBlock(x + 1, y).update();
        if(y != 0)
            parent_map->getBlock(x, y - 1).update();
        if(y != parent_map->getWorldHeight() - 1)
            parent_map->getBlock(x, y + 1).update();
        
        scheduleLightUpdate();
        
        packets::packet packet(packets::BLOCK_CHANGE);
        packet << getX() << getY() << (unsigned char)getType();
        parent_map->manager->sendToEveryone(packet);
    }
}

void serverMap::block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getBreakTime() * 9.0f);
    if(stage != getBreakStage()) {
        block_data->break_stage = stage;
        packets::packet packet(packets::BLOCK_PROGRESS_CHANGE);
        packet << getY() << getX() << getBreakStage();
        parent_map->manager->sendToEveryone(packet);
    }
}

void serverMap::block::update() {
    if(isOnlyOnFloor() && parent_map->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        breakBlock();
    scheduleLightUpdate();
}

void serverMap::block::breakBlock() {
    if(getDrop() != itemType::NOTHING)
        parent_map->spawnItem(getDrop(), x * BLOCK_WIDTH, y * BLOCK_WIDTH);
    uniqueBlock *unique_block = &block_data->getUniqueBlock();
    setType(blockType::AIR);
    setBreakProgress(0);
    if(unique_block->onBreak)
        unique_block->onBreak(parent_map, this);
}

serverMap::uniqueBlock& serverMap::blockData::getUniqueBlock() const {
    return unique_blocks[(int)block_id];
}

void serverMap::block::leftClickEvent(connection& connection, unsigned short tick_length) {
    if(block_data->getUniqueBlock().onLeftClick)
        block_data->getUniqueBlock().onLeftClick(this, parent_map->getPlayerByConnection(&connection));
    else {
        setBreakProgress(getBreakProgress() + tick_length);
        if(getBreakProgress() >= getBreakTime())
            breakBlock();
    }
}

void serverMap::block::rightClickEvent(player* peer) {
    if(block_data->getUniqueBlock().onRightClick)
        block_data->getUniqueBlock().onRightClick(this, peer);
}