//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2021.
//

#include "blocks.hpp"

#include "properties.hpp"
#include "packetType.hpp"
#include <cassert>

Block Blocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < height && x >= 0 && x < width);
    return Block(x, y, &block_arr[y * width + x], this);
}

void Block::setTypeWithoutProcessing(BlockType block_id) {
    setTypeWithoutProcessing(block_id, block_data->liquid_id);
}

void Block::setTypeWithoutProcessing(LiquidType liquid_id) {
    setTypeWithoutProcessing(block_data->block_id, liquid_id);
}

void Block::setTypeWithoutProcessing(BlockType block_id, LiquidType liquid_id) {
    block_data->block_id = block_id;
    block_data->liquid_id = liquid_id;
}

void Block::setType(BlockType block_id) {
    setType(block_id, block_data->liquid_id);
}

void Block::setType(LiquidType liquid_id) {
    setType(block_data->block_id, liquid_id);
}

void Block::setType(BlockType block_id, LiquidType liquid_id) {
    if(block_id != block_data->block_id || liquid_id != block_data->liquid_id) {
        bool was_transparent = getUniqueBlock().transparent;
        
        setTypeWithoutProcessing(block_id, liquid_id);
        
        if(liquid_id == LiquidType::EMPTY)
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

void Block::updateNeighbors() {
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

void Block::syncWithClient() {
    /*sf::Packet packet;
    packet << PacketType::BLOCK_CHANGE << getX() << getY() << (unsigned char)getLiquidType() << (unsigned char)getLiquidLevel() << (unsigned char)getLightLevel() << (unsigned char)getType();
    manager->sendToEveryone(packet);*/
}

void Block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getUniqueBlock().break_time * 9.0f);
    if(stage != getBreakStage()) {
        block_data->break_stage = stage;
        
        /*sf::Packet packet;
        packet << PacketType::BLOCK_PROGRESS_CHANGE << getX() << getY() << getBreakStage();
        manager->sendToEveryone(packet);*/
    }
}

void Block::update() {
    //if(isOnlyOnFloor() && parent_map->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        //breakBlock();
    scheduleLightUpdate();
}
