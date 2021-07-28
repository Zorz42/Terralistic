#include <cassert>
#include "serverBlocks.hpp"
#include "packetType.hpp"

ServerBlock ServerBlocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < height && x >= 0 && x < width);
    return ServerBlock(x, y, &blocks[y * width + x], this);
}

const BlockInfo& ServerBlock::getUniqueBlock() {
    return ::getBlockInfo(block_data->block_type);
}

void ServerBlock::setTypeWithoutProcessing(BlockType block_type) {
    assert((int)block_type >= 0 && block_type < BlockType::NUM_BLOCKS);
    block_data->block_type = block_type;
}

void ServerBlock::setType(BlockType block_type) {
    if(block_type != block_data->block_type) {
        ServerBlockChangeEvent event(*this, block_type);
        event.call();
        
        if(event.cancelled)
            return;
        
        parent_map->removeNaturalLight(x);
        setTypeWithoutProcessing(block_type);
        parent_map->setNaturalLight(x);
        
        update();
        updateNeighbors();
    }
}

void ServerBlock::updateNeighbors() {
    if(x != 0)
        parent_map->getBlock(x - 1, y).update();
    if(x != parent_map->getWidth() - 1)
        parent_map->getBlock(x + 1, y).update();
    if(y != 0)
        parent_map->getBlock(x, y - 1).update();
    if(y != parent_map->getHeight() - 1)
        parent_map->getBlock(x, y + 1).update();
}

void ServerBlock::setBreakProgress(unsigned short ms) {
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

void ServerBlock::update() {
    ServerBlockUpdateEvent event(*this);
    event.call();
    
    if(event.cancelled)
        return;
    
    if(getUniqueBlock().only_on_floor && parent_map->getBlock(x, (unsigned short)(y + 1)).getUniqueBlock().transparent)
        breakBlock();
    scheduleLightUpdate();
    scheduleLiquidUpdate();
}

void ServerBlock::breakBlock() {
    ServerBlockBreakEvent event(*this);
    event.call();
    
    if(event.cancelled)
        return;
    
    setType(BlockType::AIR);
    setBreakProgress(0);
}
