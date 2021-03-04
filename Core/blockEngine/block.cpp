//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/12/2020.
//

#define FILENAME block
#define NAMESPACE blockEngine
#include "core.hpp"

void updateNeighbours(unsigned short x, unsigned short y) {
    // update upper, lower, right and left block
    blockEngine::block* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    if(x != 0)
        neighbors[0] = &blockEngine::getBlock(x - 1, y);
    if(x != blockEngine::world_width - 1)
        neighbors[1] = &blockEngine::getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = &blockEngine::getBlock(x, y - 1);
    if(y != blockEngine::world_height - 1)
        neighbors[3] = &blockEngine::getBlock(x, y + 1);
    for(blockEngine::block* neighbor : neighbors)
        if(neighbor != nullptr)
            neighbor->update();
}

blockEngine::uniqueBlock::uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop, unsigned short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(name), drop(drop), break_time(break_time) {}

void blockEngine::block::update() {
    if(getUniqueBlock().only_on_floor && getBlock(getX(), (unsigned short)(getY() + 1)).getUniqueBlock().transparent)
        break_block();
}

blockEngine::uniqueBlock& blockEngine::block::getUniqueBlock() const {
     return unique_blocks[block_id];
}

void blockEngine::block::setBlockType(blockType id) {
    if(id != block_id) {
        removeNaturalLight(getX());
        block_id = id;
        setNaturalLight(getX());
        update();
        updateNeighbours(getX(), getY());
        light_update();
        
        block_change_data data;
        data.x = getX();
        data.y = getY();
        data.type = id;
        events::callEvent(block_change, (void*)&data);
    }
}

unsigned short blockEngine::block::getX() const {
    return (unsigned int)(this - blocks) % world_width;
}

unsigned short blockEngine::block::getY() const {
    return (unsigned int)(this - blocks) / world_width;
}

void blockEngine::block::setBreakProgress(unsigned short ms) {
    break_progress_ms = ms;
    unsigned char progress = (unsigned char)((float)break_progress_ms / (float)getUniqueBlock().break_time * 9.0f);
    if(progress != break_progress) {
        break_progress = progress;
        break_progress_change_data data;
        data.x = getX();
        data.y = getY();
        events::callEvent(break_progress_change, (void*)&data);
    }
}

void blockEngine::block::break_block() {
    if(getUniqueBlock().drop != itemEngine::NOTHING)
        itemEngine::spawnItem(getUniqueBlock().drop, getX() * BLOCK_WIDTH, getY() * BLOCK_WIDTH);
    setBlockType(blockEngine::AIR);
    setBreakProgress(0);
}
