#include <cassert>
#include "graphics.hpp"
#include "serverBlocks.hpp"
#include "properties.hpp"

const LiquidInfo& ServerBlock::getUniqueLiquid() {
    return ::getLiquidInfo(block_data->liquid_type);
}

bool ServerBlock::canUpdateLiquid() {
    return block_data->when_to_update_liquid != 0 && gfx::getTicks() > block_data->when_to_update_liquid;
}

void ServerBlock::setType(LiquidType liquid_type) {
    if(liquid_type != block_data->liquid_type) {
        ServerLiquidChangeEvent event(*this, liquid_type, getLiquidLevel());
        event.call();
        
        if(event.cancelled)
            return;
        
        assert((int)liquid_type >= 0 && liquid_type < LiquidType::NUM_LIQUIDS);
        block_data->liquid_type = liquid_type;
        
        if(liquid_type == LiquidType::EMPTY)
            setLiquidLevel(0);
        
        update();
        updateNeighbors();
    }
}

void ServerBlock::setLiquidLevel(unsigned char level) {
    if(level != getLiquidLevel()) {
        ServerLiquidChangeEvent event(*this, getLiquidType(), level);
        event.call();
        
        if(event.cancelled)
            return;
        
        block_data->light_level = level;
        if(level == 0)
            setType(LiquidType::EMPTY);
        
        update();
        updateNeighbors();
    }
}

static bool isFlowable(ServerBlock &block) {
    return block.getBlockInfo().ghost && block.getLiquidType() == LiquidType::EMPTY;
}

void ServerBlock::scheduleLiquidUpdate() {
    block_data->when_to_update_liquid = gfx::getTicks() + getUniqueLiquid().flow_time;
}

void ServerBlock::liquidUpdate() {
    block_data->when_to_update_liquid = 0;
    
    if(!getBlockInfo().ghost)
        setType(LiquidType::EMPTY);
    
    if(getLiquidLevel() == 0)
        return;
    
    ServerBlock under, left, right;
    
    if(y != blocks->getHeight() - 1) {
        ServerBlock block_under = blocks->getBlock(x, y + 1);
        if(isFlowable(block_under) || (block_under.getLiquidType() == getLiquidType() && block_under.getLiquidLevel() != 127))
            under = block_under;
    }
    
    if(x != 0) {
        ServerBlock block_left = blocks->getBlock(x - 1, y);
        if(isFlowable(block_left) || (block_left.getLiquidType() == getLiquidType() && block_left.getLiquidLevel() < getLiquidLevel()))
            left = block_left;
    }
    
    if(x != blocks->getWidth() - 1) {
        ServerBlock block_right = blocks->getBlock(x + 1, y);
        if(isFlowable(block_right) || (block_right.getLiquidType() == getLiquidType() && block_right.getLiquidLevel() < getLiquidLevel()))
            right = block_right;
    }
    
    
    if(under.refersToABlock()) {
        under.setType(getLiquidType());
        
        short liquid_sum = under.getLiquidLevel() + getLiquidLevel();
        if(liquid_sum > 127) {
            under.setLiquidLevel(127);
            setLiquidLevel(liquid_sum - 127);
        } else {
            setType(LiquidType::EMPTY);
            under.setLiquidLevel(liquid_sum);
        }
        
        under.setFlowDirection(FlowDirection::NONE);
    }
    
    if(getLiquidLevel() == 0)
        return;
    
    if(right.refersToABlock())
        right.setType(getLiquidType());
    if(left.refersToABlock())
        left.setType(getLiquidType());
    
    if(left.refersToABlock() && right.refersToABlock()) {
        short avg = (getLiquidLevel() + right.getLiquidLevel() + left.getLiquidLevel()) / 3;
        short mod = (getLiquidLevel() + right.getLiquidLevel() + left.getLiquidLevel()) % 3;
        if(mod) {
            if(getFlowDirection() == FlowDirection::LEFT) {
                left.setLiquidLevel(avg + mod);
                left.setFlowDirection(FlowDirection::LEFT);
                right.setLiquidLevel(avg);
            } else {
                right.setLiquidLevel(avg + mod);
                right.setFlowDirection(FlowDirection::RIGHT);
                left.setLiquidLevel(avg);
            }
        } else {
            left.setFlowDirection(FlowDirection::NONE);
            left.setLiquidLevel(avg);
            right.setFlowDirection(FlowDirection::NONE);
            right.setLiquidLevel(avg);
        }
        
        setLiquidLevel(avg);
        setFlowDirection(FlowDirection::NONE);
        
    } else if(right.refersToABlock()) {
        short avg = (getLiquidLevel() + right.getLiquidLevel()) / 2;
        short mod = (getLiquidLevel() + right.getLiquidLevel()) % 2;
        right.setLiquidLevel(avg + mod);
        right.setFlowDirection(FlowDirection::RIGHT);
        setLiquidLevel(avg);
        
    } else if(left.refersToABlock()) {
        short avg = (getLiquidLevel() + left.getLiquidLevel()) / 2;
        short mod = (getLiquidLevel() + left.getLiquidLevel()) % 2;
        left.setLiquidLevel(avg + mod);
        left.setFlowDirection(FlowDirection::LEFT);
        setLiquidLevel(avg);
    }
}
