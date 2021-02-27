//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/12/2020.
//

#define FILENAME block
#define NAMESPACE blockEngine
#include "core.hpp"

#include "singleWindowLibrary.hpp"
#include "gameLoop.hpp"

blockEngine::uniqueBlock::uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemEngine::itemType drop, unsigned short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(name), drop(drop), break_time(break_time) {
    unsigned short h = 0;
    texture = name == "air" ? nullptr : swl::loadTextureFromFile("texturePack/blocks/" + name + ".png", nullptr, &h);
    single_texture = h == 8;
}

void blockEngine::block::update(unsigned short x, unsigned short y) {
    block_orientation = 0;
    
    if(!gameLoop::online && getUniqueBlock().only_on_floor && getBlock(x, (unsigned short)(y + 1)).getUniqueBlock().transparent) {
        itemEngine::spawnItem(getUniqueBlock().drop, x * BLOCK_WIDTH, y * BLOCK_WIDTH);
        getBlock(x, y).setBlockType(AIR, x, y);
        updateNeighbours(x, y);
    }
    
    if(!getUniqueBlock().single_texture) {
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        Uint8 c = 1;
        for(int i = 0; i < 4; i++) {
            if(x + x_[i] >= world_width || x + x_[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(y + y_[i] >= world_height || y + y_[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(getBlock(x + x_[i], y + y_[i]).block_id == block_id || std::count(getUniqueBlock().connects_to.begin(), getUniqueBlock().connects_to.end(), getBlock(x + x_[i], y + y_[i]).block_id))
                block_orientation += c;
            c += c;
        }
    }
    
    getBlock(x, y).to_update = true;
    getChunk(x >> 4, y >> 4).update = true;
}

blockEngine::uniqueBlock& blockEngine::block::getUniqueBlock() const {
     return unique_blocks[block_id];
}

void blockEngine::block::setBlockType(blockType id, unsigned short x, unsigned short y) {
    if(id != block_id) {
        block_id = id;
        getBlock(x, y).to_update = true;
        getChunk(x >> 4, y >> 4).update = true;
        block_change_data data;
        data.x = x;
        data.y = y;
        data.type = id;
        events::callEvent(block_change, (void*)&data);
    }
}

void blockEngine::block::light_update(unsigned short x, unsigned short y, bool update) {
    if(update)
        to_update_light = false;
    block* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    unsigned short x_[] = {(unsigned short)(x - 1), (unsigned short)(x + 1), x, x}, y_[] = {y, y, (unsigned short)(y - 1), (unsigned short)(y + 1)};
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
            update_neighbors = true;
        }
    }
    if((update_neighbors || light_source) && update) {
        getBlock(x, y).to_update = true;
        blockEngine::getChunk(x >> 4, y >> 4).update = true;
        for(int i = 0; i < 4; i++)
            if(neighbors[i] != nullptr && !neighbors[i]->light_source)
                neighbors[i]->light_update(x_[i], y_[i]);
    }
}
