//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "core.hpp"

map::block map::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth(), "requested block is out of bounds");
    return block(x, y, &blocks[y * getWorldWidth() + x], this);
}

void map::block::setType(map::blockType id, bool process) {
    if(!process)
        block_data->block_id = id;
    else if(id != block_data->block_id) {
        parent_map->removeNaturalLight(x);
        block_data->block_id = id;
        parent_map->setNaturalLight(x);
        update();
        
        // update upper, lower, right and left block (neighbours)
        std::pair<short, short> neighbors[4] = {{-1, -1}, {-1, -1}, {-1, -1}, {-1, -1}};
        if(x != 0)
            neighbors[0] = {x - 1, y};
        if(x != parent_map->getWorldWidth() - 1)
            neighbors[1] = {x + 1, y};
        if(y != 0)
            neighbors[2] = {x, y - 1};
        if(y != parent_map->getWorldHeight() - 1)
            neighbors[3] = {x, y + 1};
        for(auto neighbor : neighbors)
            if(neighbor.first != -1)
                parent_map->getBlock(neighbor.first, neighbor.second).update();
        
        lightUpdate();
        
        /*block_change_data data{};
        data.x = getX();
        data.y = getY();
        data.type = id;
        events::callEvent(block_change, (void*)&data);*/
    }
}

void map::block::setBreakProgress(unsigned short ms) {
    block_data->break_progress = ms;
    auto stage = (unsigned char)((float)getBreakProgress() / (float)getBreakTime() * 9.0f);
    if(stage != getBreakStage()) {
        block_data->break_stage = stage;
        /*getBreakStage()_change_data data{};
        data.x = getX();
        data.y = getY();
        events::callEvent(getBreakStage()_change, (void*)&data);*/
    }
}

void map::block::update() {
    if(isOnlyOnFloor() && parent_map->getBlock(x, (unsigned short)(y + 1)).isTransparent())
        breakBlock();
}

void map::block::breakBlock() {
    if(getDrop() != itemType::NOTHING)
        parent_map->spawnItem(getDrop(), x * BLOCK_WIDTH, y * BLOCK_WIDTH);
    setType(blockType::AIR);
    setBreakProgress(0);
}

map::uniqueBlock& map::blockData::getUniqueBlock() const {
    return unique_blocks[(int)block_id];
}
