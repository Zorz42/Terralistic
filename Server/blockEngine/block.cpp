//
//  block.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#define FILENAME block
#define NAMESPACE blockEngine
#include "essential.hpp"

#include "blockEngine.hpp"

blockEngine::uniqueBlock::uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop, unsigned short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(name), drop(drop), break_time(break_time) {}

void blockEngine::block::update(unsigned short x, unsigned short y) {
    if(getUniqueBlock().only_on_floor)
        if(getBlock(x, (unsigned short)(y + 1)).getUniqueBlock().transparent)
            blockChange(x, y, AIR);
}

blockEngine::uniqueBlock& blockEngine::block::getUniqueBlock() const {
    return unique_blocks[block_id];
}
