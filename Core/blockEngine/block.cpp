//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/12/2020.
//

#define FILENAME block
#define NAMESPACE blockEngine
#include "core.hpp"

blockEngine::uniqueBlock::uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop, unsigned short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(name), drop(drop), break_time(break_time) {}

void blockEngine::block::update() {
    if(getUniqueBlock().only_on_floor && getBlock(getX(), (unsigned short)(getY() + 1)).getUniqueBlock().transparent) {
        itemEngine::spawnItem(getUniqueBlock().drop, getX() * BLOCK_WIDTH, getY() * BLOCK_WIDTH);
        getBlock(getX(), getY()).setBlockType(AIR);
        updateNeighbours(getX(), getY());
    }
}

blockEngine::uniqueBlock& blockEngine::block::getUniqueBlock() const {
     return unique_blocks[block_id];
}

void blockEngine::block::setBlockType(blockType id) {
    if(id != block_id) {
        removeNaturalLight(getX());
        block_id = id;
        setNaturalLight(getX());
        light_update();
        
        block_change_data data;
        data.x = getX();
        data.y = getY();
        data.type = id;
        events::callEvent(block_change, (void*)&data);
    }
}

void blockEngine::block::light_update(bool update) {
    unsigned short x = getX(), y = getY();
    if(update)
        to_update_light = false;
    block* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    if(x != 0 && getChunk((x - 1) >> 4, y >> 4).loaded)
        neighbors[0] = &getBlock(x - 1, y);
    if(x != blockEngine::world_width - 1 && getChunk((x + 1) >> 4, y >> 4).loaded)
        neighbors[1] = &getBlock(x + 1, y);
    if(y != 0 && getChunk(x >> 4, (y - 1) >> 4).loaded)
        neighbors[2] = &getBlock(x, y - 1);
    if(y != blockEngine::world_height - 1 && getChunk(x >> 4, (y + 1) >> 4).loaded)
        neighbors[3] = &getBlock(x, y + 1);
    bool update_neighbors = false;
    if(!light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors) {
            if(neighbor != nullptr) {
                auto light_step = (unsigned char)(neighbor->getUniqueBlock().transparent ? 3 : 15);
                auto light = (unsigned char)(light_step > neighbor->light_level ? 0 : neighbor->light_level - light_step);
                if(light > level_to_be)
                    level_to_be = light;
            }
        }
        if(!level_to_be)
            return;
        if(level_to_be != light_level) {
            light_level = level_to_be;
            light_change_data data;
            data.x = x;
            data.y = y;
            events::callEvent(light_change, (void*)&data);
            update_neighbors = true;
        }
    }
    if((update_neighbors || light_source) && update)
        for(int i = 0; i < 4; i++)
            if(neighbors[i] != nullptr && !neighbors[i]->light_source)
                neighbors[i]->light_update();
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
