//
//  blockEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "itemEngine.hpp"
#include "lightingEngine.hpp"

void grass_block_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    block->block_id = blockEngine::DIRT;
}

void air_rightClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    blockEngine::blockType type = itemEngine::selected_item->getUniqueItem().places;
    if(type != blockEngine::AIR && itemEngine::selected_item->decreaseStack(1)) {
        lightingEngine::removeNaturalLight(x);
        block->block_id = type;
        lightingEngine::setNaturalLight(x);
        blockEngine::updateNearestBlocks(x, y);
        lightingEngine::getLightBlock(x, y).update(x, y);
    }
}

void blockEngine::init() {
    unique_blocks = {
        uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING),
        uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::DIRT),
        uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::STONE_BLOCK),
        uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::NOTHING),
        uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/itemEngine::STONE),
    };
    
    std::vector<std::pair<blockType, blockType>> connections = {
        {GRASS_BLOCK, DIRT},
    };
    
    for(std::pair<blockType, blockType> i : connections) {
        unique_blocks.at(i.first).connects_to.push_back(i.second);
        unique_blocks.at(i.second).connects_to.push_back(i.first);
    }
    
    unique_blocks.at(GRASS_BLOCK).leftClickEvent = &grass_block_leftClickEvent;
    unique_blocks.at(AIR).rightClickEvent = &air_rightClickEvent;
}

void blockEngine::prepare() {
    world_height = 1200;
    world_width = 4400;
    world_ = new block[world_width * world_height];
    world = new chunk[(world_width >> 4) * (world_height >> 4)];
    
    position_x = world_width / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    position_y = world_height / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    view_x = position_x;
    view_y = position_y;
}

void blockEngine::close() {
    delete[] world_;
    delete[] world;
}

void blockEngine::render_blocks() {
    if(view_x < swl::window_width / 2)
        view_x = swl::window_width / 2;
    if(view_y < swl::window_height / 2)
        view_y = swl::window_height / 2;
    if(view_x >= blockEngine::world_width * BLOCK_WIDTH - swl::window_width / 2)
        view_x = blockEngine::world_width * BLOCK_WIDTH - swl::window_width / 2;
    if(view_y >= blockEngine::world_height * BLOCK_WIDTH - swl::window_height / 2)
        view_y = blockEngine::world_height * BLOCK_WIDTH - swl::window_height / 2;
    
    int begin_x = view_x / BLOCK_WIDTH - swl::window_width / 2 / BLOCK_WIDTH;
    int end_x = view_x / BLOCK_WIDTH + swl::window_width / 2 / BLOCK_WIDTH;
    
    int begin_y = view_y / BLOCK_WIDTH - swl::window_height / 2 / BLOCK_WIDTH;
    int end_y = view_y / BLOCK_WIDTH + swl::window_height / 2 / BLOCK_WIDTH;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > world_width)
        end_x = (int)world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > world_height)
        end_y = (int)world_height;
    
    for(unsigned short x = (begin_x >> 4) - 1; x < (end_x >> 4) + 1; x++)
        for(unsigned short y = (begin_y >> 4) - 1; y < (end_y >> 4) + 1; y++)
            getChunk(x, y).render(x, y);
}

blockEngine::block& blockEngine::getBlock(unsigned short x, unsigned short y) {
    return getChunk(x >> 4, y >> 4).blocks[x & 15][y & 15];
}

blockEngine::chunk& blockEngine::getChunk(unsigned short x, unsigned short y) {
    return world[y * (world_width >> 4) + x];
}

void blockEngine::updateNearestBlocks(unsigned short x, unsigned short y) {
    char x_[] = {0, 0, 0, -1, 1};
    char y_[] = {0, -1, 1, 0, 0};
    for(int i = 0; i < 5; i++)
        blockEngine::getBlock(x + x_[i], y + y_[i]).update(x + x_[i], y + y_[i]);
}

void blockEngine::rightClickEvent(unsigned short x, unsigned short y) {
    block* block = &getBlock(x, y);
    if(block->getUniqueBlock().rightClickEvent)
        block->getUniqueBlock().rightClickEvent(block, x, y);
}

void blockEngine::leftClickEvent(unsigned short x, unsigned short y) {
    block* block = &getBlock(x, y);
    if(block->getUniqueBlock().leftClickEvent)
        block->getUniqueBlock().leftClickEvent(block, x, y);
    else {
        if(block->getUniqueBlock().drop != itemEngine::NOTHING)
            itemEngine::spawnItem(block->getUniqueBlock().drop, x * BLOCK_WIDTH, y * BLOCK_WIDTH);
        lightingEngine::removeNaturalLight(x);
        getBlock(x, y).block_id = blockEngine::AIR;
        updateNearestBlocks(x, y);
        lightingEngine::setNaturalLight(x);
        lightingEngine::getLightBlock(x, y).update(x, y);
    }
}
