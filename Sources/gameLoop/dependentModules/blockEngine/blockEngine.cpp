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

void grass_block_leftClickEvent(blockEngine::block* block) {
    block->block_id = blockEngine::DIRT;
}

void air_rightClickEvent(blockEngine::block* block) {
    blockEngine::blockType type = itemEngine::selected_item->getUniqueItem().places;
    if(type != blockEngine::AIR && itemEngine::selected_item->decreaseStack(1)) {
        lightingEngine::removeNaturalLight(block->getX());
        block->block_id = type;
        lightingEngine::setNaturalLight(block->getX());
        blockEngine::updateNearestBlocks(block->getX(), block->getY());
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
    world_width = 4200;
    world = new block[world_width * world_height];
    
    position_x = world_width / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    position_y = world_height / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    view_x = position_x;
    view_y = position_y;
}

void blockEngine::close() {
    delete[] world;
}

void blockEngine::render_blocks() {
#define VIEW_PADDING 2

    if(view_x < swl::window_width / 2)
        view_x = swl::window_width / 2;
    if(view_y < swl::window_height / 2)
        view_y = swl::window_height / 2;
    if(view_x >= blockEngine::world_width * BLOCK_WIDTH - swl::window_width / 2)
        view_x = blockEngine::world_width * BLOCK_WIDTH - swl::window_width / 2;
    if(view_y >= blockEngine::world_height * BLOCK_WIDTH - swl::window_height / 2)
        view_y = blockEngine::world_height * BLOCK_WIDTH - swl::window_height / 2;
    
    int begin_x = (int)view_x / BLOCK_WIDTH - swl::window_width / 2 / BLOCK_WIDTH - VIEW_PADDING;
    int end_x = (int)view_x / BLOCK_WIDTH + swl::window_width / 2 / BLOCK_WIDTH + VIEW_PADDING;
    
    int begin_y = (int)view_y / BLOCK_WIDTH - swl::window_height / 2 / BLOCK_WIDTH - VIEW_PADDING;
    int end_y = (int)view_y / BLOCK_WIDTH + swl::window_height / 2 / BLOCK_WIDTH + VIEW_PADDING;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > world_width)
        end_x = world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > world_height)
        end_y = world_height;
    
    for(int x = begin_x; x < end_x; x++)
        for(int y = begin_y; y < end_y; y++)
            getBlock(x, y).draw();
}

blockEngine::block& blockEngine::getBlock(unsigned int x, unsigned int y) {
    return world[y * world_width + x];
}

void blockEngine::updateNearestBlocks(int x, int y) {
    char x_[] = {0, 0, 0, -1, 1};
    char y_[] = {0, -1, 1, 0, 0};
    for(char i = 0; i < 5; i++)
        blockEngine::getBlock(x + x_[i], y + y_[i]).update();
}

void blockEngine::rightClickEvent(int x, int y) {
    block* block = &getBlock(x, y);
    if(block->getUniqueBlock().rightClickEvent)
        block->getUniqueBlock().rightClickEvent(block);
}

void blockEngine::leftClickEvent(int x, int y) {
    block* block = &getBlock(x, y);
    if(block->getUniqueBlock().leftClickEvent)
        block->getUniqueBlock().leftClickEvent(block);
    else {
        if(block->getUniqueBlock().drop != itemEngine::NOTHING)
            itemEngine::spawnItem(block->getUniqueBlock().drop, x * BLOCK_WIDTH, y * BLOCK_WIDTH);
        getBlock(x, y) = blockEngine::AIR;
        updateNearestBlocks(x, y);
    }
}
