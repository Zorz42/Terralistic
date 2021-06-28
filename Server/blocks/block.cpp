//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2021.
//

#include "blocks.hpp"

#include "assert.hpp"
#include "serverNetworking.hpp"
#include "properties.hpp"

block blocks::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < height && x >= 0 && x < width, "requested block is out of bounds")
    return block(x, y, &block_arr[y * width + x], this, manager);
}

void block::setType(blockType block_id, bool process) {
    setType(block_id, block_data->liquid_id, process);
}

void block::setType(liquidType liquid_id, bool process) {
    setType(block_data->block_id, liquid_id, process);
}

void block::setType(blockType block_id, liquidType liquid_id, bool process) {
    if(!process) {
        block_data->block_id = block_id;
        block_data->liquid_id = liquid_id;
    }
    else if(block_id != block_data->block_id || liquid_id != block_data->liquid_id) {
        bool was_transparent = getUniqueBlock().transparent;
        block_data->block_id = block_id;
        block_data->liquid_id = liquid_id;
        if(liquid_id == liquidType::EMPTY)
            setLiquidLevel(0);

        if(getUniqueBlock().transparent != was_transparent) {
            if(getUniqueBlock().transparent)
                for(int curr_y = y; parent_map->getBlock(x, curr_y).getUniqueBlock().transparent; curr_y++)
                    parent_map->getBlock(x, curr_y).setLightSource(MAX_LIGHT);
            else
                for(int curr_y = y; parent_map->getBlock(x, curr_y).isLightSource(); curr_y++)
                    parent_map->getBlock(x, curr_y).removeLightSource();
        }

        update();
        updateNeighbors();
        syncWithClient();
    }
}

void block::updateNeighbors() {
    // update upper, lower, right and left block (neighbours)
    if(x != 0)
        parent_map->getBlock(x - 1, y).update();
    if(x != parent_map->getWidth() - 1)
        parent_map->getBlock(x + 1, y).update();
    if(y != 0)
        parent_map->getBlock(x, y - 1).update();
    if(y != parent_map->getHeight() - 1)
        parent_map->getBlock(x, y + 1).update();
}

void block::syncWithClient() {
    packets::packet packet(packets::BLOCK_CHANGE, sizeof(getX()) + sizeof(getY()) + sizeof(unsigned char) + sizeof(unsigned char) + sizeof(unsigned char) + sizeof(unsigned char));
    packet << getX() << getY() << (unsigned char)getLiquidType() << (unsigned char)getLiquidLevel() << (unsigned char)getLightLevel() << (unsigned char)getType();
    manager->sendToEveryone(packet);
}

void block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getUniqueBlock().break_time * 9.0f);
    if(stage != getBreakStage()) {
        block_data->break_stage = stage;
        packets::packet packet(packets::BLOCK_PROGRESS_CHANGE, sizeof(getY()) + sizeof(getX()) + sizeof(getBreakStage()));
        packet << getY() << getX() << getBreakStage();
        manager->sendToEveryone(packet);
    }
}

void block::update() {
    //if(isOnlyOnFloor() && parent_map->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        //breakBlock();
    scheduleLightUpdate();
}
