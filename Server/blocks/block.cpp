#include "blocks.hpp"

#include "packetType.hpp"
#include <cassert>


Block Blocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < height && x >= 0 && x < width);
    return Block(x, y, &blocks[y * width + x], this);
}

const BlockInfo& Block::getUniqueBlock() {
    return ::getBlockInfo(block_data->block_id);
}

void Block::setTypeWithoutProcessing(BlockType block_id) {
    block_data->block_id = block_id;
}

void Block::setType(BlockType block_id) {
    if(block_id != block_data->block_id) {
        ServerBlockChangeEvent event(*this, block_id);
        event.call();
        
        if(event.cancelled)
            return;
        
        bool was_transparent = getUniqueBlock().transparent;
        
        setTypeWithoutProcessing(block_id);
        
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
    }
}

void Block::updateNeighbors() {
    if(x != 0)
        parent_map->getBlock(x - 1, y).update();
    if(x != parent_map->getWidth() - 1)
        parent_map->getBlock(x + 1, y).update();
    if(y != 0)
        parent_map->getBlock(x, y - 1).update();
    if(y != parent_map->getHeight() - 1)
        parent_map->getBlock(x, y + 1).update();
}

void Block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getUniqueBlock().break_time * 9.0f);
    if(stage != getBreakStage()) {
        ServerBlockBreakStageChangeEvent event(*this, stage);
        event.call();
        
        if(event.cancelled)
            return;
        
        block_data->break_stage = stage;
    }
}

void Block::update() {
    //if(isOnlyOnFloor() && parent_map->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        //breakBlock();
    scheduleLightUpdate();
}
