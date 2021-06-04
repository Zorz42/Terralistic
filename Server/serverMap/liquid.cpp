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
    
    block block_under = parent_map->getBlock(x, y + 1), block_left = parent_map->getBlock(x - 1, y), block_right = parent_map->getBlock(x + 1, y);
    if(block_under.isGhost() && block_under.getLiquidType() == liquidType::EMPTY) {
        block_under.setType(getLiquidType());
        block_under.setLiquidLevel(0);
    }
    if(block_under.getLiquidType() == getLiquidType() && block_under.getLiquidLevel() != 127) {
        unsigned char liquid_sum = block_under.getLiquidLevel() + getLiquidLevel();
        if(liquid_sum > 127) {
            block_under.setLiquidLevel(127);
            setLiquidLevel(liquid_sum - 127);
        } else {
            setType(liquidType::EMPTY);
            setLiquidLevel(0);
            block_under.setLiquidLevel(liquid_sum);
        }
    } else {
        if(block_left.isGhost() && block_left.getLiquidType() == liquidType::EMPTY) {
            block_left.setType(getLiquidType());
            block_left.setLiquidLevel(0);
        }
        
        if(block_right.isGhost() && block_right.getLiquidType() == liquidType::EMPTY) {
            block_right.setType(getLiquidType());
            block_right.setLiquidLevel(0);
        }
        
        if(block_left.getLiquidType() == getLiquidType() && block_right.getLiquidType() == getLiquidType()) {
            unsigned char avg = ((int)block_left.getLiquidLevel() + (int)block_right.getLiquidLevel() + (int)getLiquidLevel()) / 3;
            block_left.setLiquidLevel(avg);
            block_right.setLiquidLevel(avg);
            setLiquidLevel(avg);
        } else if(block_left.getLiquidType() == getLiquidType()) {
            unsigned char avg = ((int)block_left.getLiquidLevel() + (int)getLiquidLevel()) / 2;
            block_left.setLiquidLevel(avg);
            setLiquidLevel(avg);
        } else if(block_right.getLiquidType() == getLiquidType()) {
            unsigned char avg = ((int)block_right.getLiquidLevel() + (int)getLiquidLevel()) / 2;
            block_right.setLiquidLevel(avg);
            setLiquidLevel(avg);
        }
    }
}
