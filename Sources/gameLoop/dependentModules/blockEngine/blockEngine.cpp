//
//  blockEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"
#include "itemEngine.hpp"
#include "inventory.hpp"
#include "gameLoop.hpp"
#include "blockSelector.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"

ogl::texture prepare_text;

void grass_block_leftClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    block->setBlockType(blockEngine::DIRT, x, y);
}

void air_rightClickEvent(blockEngine::block* block, unsigned short x, unsigned short y) {
    blockEngine::blockType type = playerHandler::selected_item->getUniqueItem().places;
    if(type != blockEngine::AIR && playerHandler::selected_item->decreaseStack(1)) {
        blockEngine::removeNaturalLight(x);
        block->setBlockType(type, x, y);
        blockEngine::setNaturalLight(x);
        blockEngine::getBlock(x, y).update(x, y);
        blockEngine::updateNearestBlocks(x, y);
        blockEngine::getBlock(x, y).light_update(x, y);
    }
}

void blockEngine::init() {
    unique_blocks = {
        uniqueBlock("air",         /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING),
        uniqueBlock("dirt",        /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::DIRT),
        uniqueBlock("stone_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::STONE_BLOCK),
        uniqueBlock("grass_block", /*ghost*/false, /*only_on_floor*/false,  /*transparent*/false, /*drop*/itemEngine::NOTHING),
        uniqueBlock("stone",       /*ghost*/true,  /*only_on_floor*/true,   /*transparent*/true,  /*drop*/itemEngine::STONE),
        uniqueBlock("wood",        /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING),
        uniqueBlock("leaves",      /*ghost*/true,  /*only_on_floor*/false,  /*transparent*/true,  /*drop*/itemEngine::NOTHING),
    };
    
    unique_blocks[GRASS_BLOCK].connects_to.push_back(DIRT);
    unique_blocks[DIRT].connects_to.push_back(GRASS_BLOCK);
    unique_blocks[WOOD].connects_to.push_back(GRASS_BLOCK);
    unique_blocks[WOOD].connects_to.push_back(LEAVES);
    
    unique_blocks[GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    unique_blocks[AIR].rightClickEvent = &air_rightClickEvent;
    
    prepare_text.loadFromText("Preparing world", {255, 255, 255});
    prepare_text.scale = 3;
}

void blockEngine::prepare() {
    world_height = 1200;
    world_width = 4400;
    world = new chunk[(world_width >> 4) * (world_height >> 4)];
    
    for(unsigned short x = 0; x < (world_width >> 4); x++)
        for(unsigned short y = 0; y < (world_height >> 4); y++)
            for(auto & block : getChunk(x, y).blocks)
                for(unsigned short y_ = 0; y_ < 16; y_++)
                    block[y_].to_update = true;
}

void blockEngine::close() {
    delete[] world;
}

void blockEngine::render_blocks() {
    unsigned short begin_x = playerHandler::view_x / BLOCK_WIDTH - swl::window_width / 2 / BLOCK_WIDTH;
    unsigned short end_x = playerHandler::view_x / BLOCK_WIDTH + swl::window_width / 2 / BLOCK_WIDTH;

    unsigned short begin_y = playerHandler::view_y / BLOCK_WIDTH - swl::window_height / 2 / BLOCK_WIDTH;
    unsigned short end_y = playerHandler::view_y / BLOCK_WIDTH + swl::window_height / 2 / BLOCK_WIDTH;
    
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
            if(blockEngine::getBlock(x, y).to_update_light)
                blockEngine::getBlock(x, y).light_update(x, y);
}

blockEngine::block& blockEngine::getBlock(unsigned short x, unsigned short y) {
    return getChunk(x >> 4, y >> 4).blocks[x & 15][y & 15];
}

blockEngine::chunk& blockEngine::getChunk(unsigned short x, unsigned short y) {
    return world[y * (world_width >> 4) + x];
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
    if(gameLoop::online) {
        packets::packet packet(packets::RIGHT_CLICK);
        packet << x << y;
        networking::sendPacket(packet);
    } else {
        block* block = &getBlock(x, y);
        if(block->getUniqueBlock().rightClickEvent)
            block->getUniqueBlock().rightClickEvent(block, x, y);
    }
}

void blockEngine::leftClickEvent(unsigned short x, unsigned short y) {
    if(gameLoop::online) {
        packets::packet packet(packets::LEFT_CLICK);
        packet << x << y;
        networking::sendPacket(packet);
    } else {
        block* block = &getBlock(x, y);
        if(block->getUniqueBlock().leftClickEvent)
            block->getUniqueBlock().leftClickEvent(block, x, y);
        else {
            if(block->getUniqueBlock().drop != itemEngine::NOTHING && !gameLoop::online)
                itemEngine::spawnItem(block->getUniqueBlock().drop, x * BLOCK_WIDTH, y * BLOCK_WIDTH);
            blockEngine::removeNaturalLight(x);
            getBlock(x, y).setBlockType(blockEngine::AIR, x, y);
            updateNearestBlocks(x, y);
            blockEngine::setNaturalLight(x);
            blockEngine::getBlock(x, y).light_update(x, y);
        }
    }
}

void blockEngine::prepareWorld() {
    swl::setDrawColor(0, 0, 0);
    swl::clear();
    prepare_text.render();
    swl::update();
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            blockEngine::getChunk(x, y).createTexture();
    
    for(unsigned short x = 0; x < world_width; x++)
        setNaturalLight(x);
}

void blockEngine::handleEvents(SDL_Event& event) {
    if(event.type == SDL_MOUSEBUTTONDOWN) {
        if(event.button.button == SDL_BUTTON_LEFT && !playerHandler::hovered)
            blockEngine::leftClickEvent(blockSelector::selectedBlockX, blockSelector::selectedBlockY);
        else if(event.button.button == SDL_BUTTON_RIGHT && !blockSelector::collidingWithPlayer() && !playerHandler::hovered)
            blockEngine::rightClickEvent(blockSelector::selectedBlockX, blockSelector::selectedBlockY);
    }
}
