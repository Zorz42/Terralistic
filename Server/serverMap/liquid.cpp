//
//  liquid.cpp
//  Server
//
//  Created by Jakob Zorz on 03/06/2021.
//

#include "serverMap.hpp"

void serverMap::block::liquidUpdate() {
    if(getLiquidType() == liquidType::EMPTY)
        return;
    block block_under = parent_map->getBlock(x, y + 1);
    if(block_under.getLiquidType() == liquidType::EMPTY && block_under.isGhost() == true) {
        block_under.setType(getLiquidType());
        setType(liquidType::EMPTY);
    }
}
