#include <cassert>
#include "serverBlocks-old.hpp"

ServerBlock ServerBlocks::getBlock(unsigned short x, unsigned short y) {
    return {x, y, getMapBlock(x, y), this};
}

const BlockInfo& ServerBlock::getBlockInfo() {
    return ::getBlockInfo(block_data->block_type);
}

void ServerBlock::setType(BlockType block_type) {
    if(block_type != block_data->block_type) {
        ServerBlockChangeEvent event(*this, block_type);
        blocks->block_change_event.call(event);
        
        assert((int)block_type >= 0 && block_type < BlockType::NUM_BLOCKS);
        block_data->block_type = block_type;
        
        update();
        updateNeighbors();
    }
}

void ServerBlock::updateNeighbors() {
    if(x != 0)
        blocks->getBlock(x - 1, y).update();
    if(x != blocks->getWidth() - 1)
        blocks->getBlock(x + 1, y).update();
    if(y != 0)
        blocks->getBlock(x, y - 1).update();
    if(y != blocks->getHeight() - 1)
        blocks->getBlock(x, y + 1).update();
}

void ServerBlock::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    unsigned char stage = (unsigned char)((float)getBreakProgress() / (float)getBlockInfo().break_time * 9.f);
    if(stage != getBreakStage()) {
        ServerBlockBreakStageChangeEvent event(*this, stage);
        blocks->block_break_stage_change_event.call(event);

        block_data->break_stage = stage;
    }
}

void ServerBlock::update() {
    ServerBlockUpdateEvent event(*this);
    blocks->block_update_event.call(event);
    
    if(getBlockInfo().only_on_floor && blocks->getBlock(x, (unsigned short)(y + 1)).getBlockInfo().transparent)
        breakBlock();
    scheduleLightUpdate();
    scheduleLiquidUpdate();
}

void ServerBlock::breakBlock() {
    ServerBlockBreakEvent event(*this);
    blocks->block_break_event.call(event);
    
    setType(BlockType::AIR);
    setBreakProgress(0);
}

ServerMapBlock* ServerBlocks::getMapBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < height && x >= 0 && x < width);
    return &blocks[y * width + x];
}

BlockType ServerBlocks::getTypeDirectly(unsigned short x, unsigned short y) {
    return getMapBlock(x, y)->block_type;
}

unsigned char ServerBlocks::getLiquidLevelDirectly(unsigned short x, unsigned short y) {
    return getMapBlock(x, y)->liquid_level;
}

LiquidType ServerBlocks::getLiquidTypeDirectly(unsigned short x, unsigned short y) {
    return getMapBlock(x, y)->liquid_type;
}

void ServerBlocks::setTypeDirectly(unsigned short x, unsigned short y, BlockType type) {
    getMapBlock(x, y)->block_type = type;
}

void ServerBlocks::setTypeDirectly(unsigned short x, unsigned short y, LiquidType type) {
    getMapBlock(x, y)->liquid_type = type;
}

void ServerBlocks::setLiquidLevelDirectly(unsigned short x, unsigned short y, unsigned char level) {
    getMapBlock(x, y)->liquid_level = level;
}
