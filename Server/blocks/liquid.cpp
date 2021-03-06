#include "blocks.hpp"
#include "properties.hpp"

const LiquidInfo& Block::getUniqueLiquid() {
    return ::getLiquidInfo(block_data->liquid_id);
}

void Block::setTypeWithoutProcessing(LiquidType liquid_id) {
    block_data->liquid_id = liquid_id;
}

void Block::setType(LiquidType liquid_id) {
    if(liquid_id != block_data->liquid_id) {
        ServerLiquidChangeEvent event(*this, liquid_id, getLiquidLevel());
        event.call();
        
        if(event.cancelled)
            return;
        
        setTypeWithoutProcessing(liquid_id);
        
        if(liquid_id == LiquidType::EMPTY)
            setLiquidLevel(0);
        
        update();
        updateNeighbors();
    }
}

void Block::setLiquidLevelWithoutProcessing(unsigned char level) {
    block_data->liquid_level = level;
}

void Block::setLiquidLevel(unsigned char level) {
    if(level != getLiquidLevel()) {
        ServerLiquidChangeEvent event(*this, getLiquidType(), level);
        event.call();
        
        if(event.cancelled)
            return;
        
        setLiquidLevelWithoutProcessing(level);
        update();
    }
}

void Block::liquidUpdate() {
    block_data->when_to_update_liquid = (unsigned int)std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count() + getUniqueLiquid().flow_time;
    if(!getUniqueBlock().ghost)
        setType(LiquidType::EMPTY);
    
    if(getLiquidLevel() == 0)
        return;
    
    Block under, left, right;
    
    if(y != parent_map->getHeight() - 1) {
        Block block_under = parent_map->getBlock(x, y + 1);
        if((getUniqueBlock().ghost && block_under.getLiquidType() == LiquidType::EMPTY) || (block_under.getLiquidType() == getLiquidType() && block_under.getLiquidLevel() != 127))
            under = block_under;
    }
    
    if(x != 0) {
        Block block_left = parent_map->getBlock(x - 1, y);
        if((getUniqueBlock().ghost && block_left.getLiquidType() == LiquidType::EMPTY) || (block_left.getLiquidType() == getLiquidType() && block_left.getLiquidLevel() < getLiquidLevel()))
            left = block_left;
    }
    
    if(x != parent_map->getWidth() - 1) {
        Block block_right = parent_map->getBlock(x + 1, y);
        if((getUniqueBlock().ghost && block_right.getLiquidType() == LiquidType::EMPTY) || (block_right.getLiquidType() == getLiquidType() && block_right.getLiquidLevel() < getLiquidLevel()))
            right = block_right;
    }
    
    
    if(under.refersToABlock()) {
        if(under.getLiquidType() == LiquidType::EMPTY) {
            under.setType(getLiquidType());
        }
        
        unsigned char liquid_sum = under.getLiquidLevel() + getLiquidLevel();
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
    
    
    if(right.refersToABlock() && right.getLiquidType() == LiquidType::EMPTY)
        right.setType(getLiquidType());
    if(left.refersToABlock() && left.getLiquidType() == LiquidType::EMPTY)
        left.setType(getLiquidType());
    
    if(left.refersToABlock() && right.refersToABlock()) {
        int avg = (getLiquidLevel() + right.getLiquidLevel() + left.getLiquidLevel()) / 3;
        int mod = (getLiquidLevel() + right.getLiquidLevel() + left.getLiquidLevel()) % 3;
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
        int avg = (getLiquidLevel() + right.getLiquidLevel()) / 2;
        int mod = (getLiquidLevel() + right.getLiquidLevel()) % 2;
        right.setLiquidLevel(avg + mod);
        right.setFlowDirection(FlowDirection::RIGHT);
        setLiquidLevel(avg);
    } else if(left.refersToABlock()) {
        int avg = (getLiquidLevel() + left.getLiquidLevel()) / 2;
        int mod = (getLiquidLevel() + left.getLiquidLevel()) % 2;
        left.setLiquidLevel(avg + mod);
        left.setFlowDirection(FlowDirection::LEFT);
        setLiquidLevel(avg);
    }
}
