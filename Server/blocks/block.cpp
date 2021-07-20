#include "blocks.hpp"

#include "packetType.hpp"
#include <cassert>


Block Blocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < height && x >= 0 && x < width);
    return Block(x, y, &blocks[y * width + x], this);
}

const BlockInfo& Block::getUniqueBlock() {
    return ::getBlockInfo(block_data->block_type);
}

void Block::setTypeWithoutProcessing(BlockType block_id) {
    assert((int)block_id >= 0 && block_id < BlockType::NUM_BLOCKS);
    block_data->block_type = block_id;
}

void Block::setType(BlockType block_id) {
    if(block_id != block_data->block_type) {
        ServerBlockChangeEvent event(*this, block_id);
        event.call();
        
        if(event.cancelled)
            return;
        
        parent_map->removeNaturalLight(x);
        setTypeWithoutProcessing(block_id);
        parent_map->setNaturalLight(x);
        
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
    if(getUniqueBlock().only_on_floor && parent_map->getBlock(x, (unsigned short)(y + 1)).getUniqueBlock().transparent)
        breakBlock();
    scheduleLightUpdate();
}

void Block::breakBlock() {
    ServerBlockBreakEvent event(*this);
    event.call();
    
    if(event.cancelled)
        return;
    
    setType(BlockType::AIR);
    setBreakProgress(0);
}
