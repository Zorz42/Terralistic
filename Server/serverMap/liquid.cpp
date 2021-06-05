//
//  liquid.cpp
//  Server
//
//  Created by Jakob Zorz on 03/06/2021.
//

#include "serverMap.hpp"

std::vector<serverMap::uniqueLiquid> serverMap::unique_liquids;

void serverMap::initLiquids() {
    unique_liquids = {
        {0},
        {100},
    };
}

serverMap::uniqueLiquid& serverMap::blockData::getUniqueLiquid() const {
    return unique_liquids[(int)liquid_id];
}

void serverMap::block::setLiquidLevel(unsigned char level) {
    if(level != getLiquidLevel()) {
        block_data->liquid_level = level;
        scheduleLiquidUpdate();
        syncWithClient();
        updateNeighbors();
    }
}

void serverMap::block::liquidUpdate() {
    block_data->when_to_update_liquid = 0;
    if(getLiquidType() == liquidType::EMPTY || getLiquidLevel() == 0)
        return;
    
    if(!isGhost()) {
        setType(liquidType::EMPTY);
        setLiquidLevel(0);
        return;
    }
    
    block block_under = parent_map->getBlock(x, y + 1), block_left = parent_map->getBlock(x - 1, y), block_right = parent_map->getBlock(x + 1, y), under, left, right;
    if((block_under.isGhost() && block_under.getLiquidType() == liquidType::EMPTY) || (block_under.getLiquidType() == getLiquidType() && block_under.getLiquidLevel() != 127))
        under = block_under;
    if((block_left.isGhost() && block_left.getLiquidType() == liquidType::EMPTY) || (block_left.getLiquidType() == getLiquidType() && block_left.getLiquidLevel() < getLiquidLevel()))
        left = block_left;
    if((block_right.isGhost() && block_right.getLiquidType() == liquidType::EMPTY) || (block_right.getLiquidType() == getLiquidType() && block_right.getLiquidLevel() < getLiquidLevel()))
        right = block_right;
    
    
    if(under.refersToABlock()) {
        if(under.getLiquidType() == liquidType::EMPTY) {
            under.setType(getLiquidType());
        }
        
        unsigned char liquid_sum = under.getLiquidLevel() + getLiquidLevel();
        if(liquid_sum > 127) {
            under.setLiquidLevel(127);
            setLiquidLevel(liquid_sum - 127);
        } else {
            setType(liquidType::EMPTY);
            under.setLiquidLevel(liquid_sum);
        }
    } else {
        if(right.refersToABlock() && right.getLiquidType() == liquidType::EMPTY) {
            right.setType(getLiquidType());
        }
        if(left.refersToABlock() && left.getLiquidType() == liquidType::EMPTY) {
            left.setType(getLiquidType());
        }
        
        if(left.refersToABlock() && right.refersToABlock()) {
            int avg = (getLiquidLevel() + right.getLiquidLevel() + left.getLiquidLevel()) / 3;
            left.setLiquidLevel(avg);
            right.setLiquidLevel(avg);
            setLiquidLevel(avg);
        } else if(right.refersToABlock()) {
            int avg = (getLiquidLevel() + right.getLiquidLevel()) / 2;
            int mod = (getLiquidLevel() + right.getLiquidLevel()) % 2;
            right.setLiquidLevel(avg + mod);
            setLiquidLevel(avg);
        } else if(left.refersToABlock()) {
            int avg = (getLiquidLevel() + left.getLiquidLevel()) / 2;
            int mod = (getLiquidLevel() + left.getLiquidLevel()) % 2;
            left.setLiquidLevel(avg + mod);
            setLiquidLevel(avg);
        }
    }
}
