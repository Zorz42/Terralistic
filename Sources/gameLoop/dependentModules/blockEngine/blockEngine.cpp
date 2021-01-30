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
#include "objectedGraphicsLibrary.hpp"
#include "inventory.hpp"
#include "gameLoop.hpp"

ogl::texture chunk_text;

void grass_block_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    block->setBlockType(blockEngine::DIRT, x, y);
}

void air_rightClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    blockEngine::blockType type = inventory::selected_item->getUniqueItem().places;
    if(type != blockEngine::AIR && inventory::selected_item->decreaseStack(1)) {
        lightingEngine::removeNaturalLight(x);
        block->setBlockType(type, x, y);
        lightingEngine::setNaturalLight(x);
        blockEngine::getBlock(x, y).update(x, y);
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
        unique_blocks[i.first].connects_to.push_back(i.second);
        unique_blocks[i.second].connects_to.push_back(i.first);
    }
    
    unique_blocks[GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    unique_blocks[AIR].rightClickEvent = &air_rightClickEvent;
    
    chunk_text.loadFromText("Preparing chunks", {255, 255, 255});
    chunk_text.scale = 3;
}

void blockEngine::prepare() {
    world_height = 1200;
    world_width = 4400;
    world = new chunk[(world_width >> 4) * (world_height >> 4)];
    
    position_x = world_width / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    position_y = world_height / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    view_x = position_x;
    view_y = position_y;
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            for(unsigned short x_ = 0; x_ < 16; x_++)
                for(unsigned short y_ = 0; y_ < 16; y_++)
                    getChunk(x, y).updates[x_][y_] = true;
}

void blockEngine::close() {
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
        for(unsigned short y = (begin_y >> 4) - 1; y < (end_y >> 4) + 2; y++) {
            if(getChunk(x, y).update)
                getChunk(x, y).updateTexture();
            getChunk(x, y).render(x, y);
        }
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(lightingEngine::getLightBlock(x, y).to_update)
                lightingEngine::getLightBlock(x, y).update(x, y);
}

blockEngine::block& blockEngine::getBlock(unsigned short x, unsigned short y) {
    return getChunk(x >> 4, y >> 4).blocks[x & 15][y & 15];
}

blockEngine::chunk& blockEngine::getChunk(unsigned short x, unsigned short y) {
    return world[y * (world_width >> 4) + x];
}

void blockEngine::setUpdateBlock(unsigned short x, unsigned short y, bool value) {
    getChunk(x >> 4, y >> 4).updates[x & 15][y & 15] = value;
}

void blockEngine::updateNearestBlocks(unsigned short x, unsigned short y) {
    block* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    unsigned short x_[4] = {(unsigned short)(x - 1), (unsigned short)(x + 1), x, x}, y_[4] = {y, y, (unsigned short)(y - 1), (unsigned short)(y + 1)};
    if(x != 0)
        neighbors[0] = &getBlock(x - 1, y);
    if(x != blockEngine::world_width - 1)
        neighbors[1] = &getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = &getBlock(x, y - 1);
    if(y != blockEngine::world_height - 1)
        neighbors[3] = &getBlock(x, y + 1);
    for(int i = 0; i < 4; i++)
        if(neighbors[i] != nullptr)
            neighbors[i]->update(x_[i], y_[i]);
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
        if(block->getUniqueBlock().drop != itemEngine::NOTHING && !gameLoop::online)
            itemEngine::spawnItem(block->getUniqueBlock().drop, x * BLOCK_WIDTH, y * BLOCK_WIDTH);
        lightingEngine::removeNaturalLight(x);
        getBlock(x, y).setBlockType(blockEngine::AIR, x, y);
        updateNearestBlocks(x, y);
        lightingEngine::setNaturalLight(x);
        lightingEngine::getLightBlock(x, y).update(x, y);
    }
}

void blockEngine::prepareChunks() {
    swl::setDrawColor(0, 0, 0);
    swl::clear();
    chunk_text.render();
    swl::update();
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            blockEngine::getChunk(x, y).createTexture();
}
