//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "map.hpp"
#include "dev.hpp"
#include "networkingModule.hpp"

std::vector<map::uniqueBlock> map::unique_blocks;

void map::initBlocks() {
    map::unique_blocks = {
        map::uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
        map::uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/map::itemType::DIRT,        /*break_time*/1000),
        map::uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/map::itemType::STONE_BLOCK, /*break_time*/1000),
        map::uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
        map::uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/map::itemType::STONE,       /*break_time*/1000),
        map::uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
        map::uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/map::itemType::NOTHING,     /*break_time*/1000),
    };
}

map::block map::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth(), "requested block is out of bounds");
    return block(x, y, &blocks[y * getWorldWidth() + x], this);
}

void map::block::setType(map::blockType id, bool process) {
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
        networking::sendToEveryone(packet);
    }
}

void map::block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getBreakTime() * 9.0f);
    if(stage != getBreakStage()) {
        block_data->break_stage = stage;
        packets::packet packet(packets::BLOCK_PROGRESS_CHANGE);
        packet << getY() << getX() << getBreakStage();
        networking::sendToEveryone(packet);
    }
}

void map::block::update() {
    if(isOnlyOnFloor() && parent_map->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        breakBlock();
    scheduleLightUpdate();
}

void map::block::breakBlock() {
    if(getDrop() != itemType::NOTHING)
        parent_map->spawnItem(getDrop(), x * BLOCK_WIDTH, y * BLOCK_WIDTH);
    setType(blockType::AIR);
    setBreakProgress(0);
}

map::uniqueBlock& map::blockData::getUniqueBlock() const {
    return unique_blocks[(int)block_id];
}
