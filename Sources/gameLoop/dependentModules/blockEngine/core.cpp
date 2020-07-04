//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"

#define BLOCK_WIDTH 16

void block_engine::init() {
    unsigned int world_height = 1200;
    world_width = 4200;
    world.reserve(world_width * world_height);
    for(int i = 0; i < world_width * world_height; i++)
        world.push_back(BLOCK_AIR);
    
    block_types.push_back(unique_block("air"));
    block_types.at(0).texture = nullptr;
    block_types.push_back(unique_block("dirt"));
    block_types.at(1).texture = swl::loadTextureFromFile("texturePack/dirt_block.png");
    
    position_x = world_width / 2 * BLOCK_WIDTH;
    position_y = world_height / 2 * BLOCK_WIDTH;
}

void block_engine::render_blocks() {
#define WIEW_PADDING 2

    int begin_x = (int)position_x / BLOCK_WIDTH - swl::window_width / 2 / BLOCK_WIDTH - (WIEW_PADDING);
    int end_x = (int)position_x / BLOCK_WIDTH + swl::window_width / 2 / BLOCK_WIDTH + (WIEW_PADDING);
    
    int begin_y = (int)position_y / BLOCK_WIDTH - swl::window_height / 2 / BLOCK_WIDTH - (WIEW_PADDING);
    int end_y = (int)position_y / BLOCK_WIDTH + swl::window_height / 2 / BLOCK_WIDTH + (WIEW_PADDING);
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > world_width)
        end_x = world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > getWorldHeight())
        end_y = getWorldHeight();
    
    for(int x = begin_x; x < end_x; x++)
        for(int y = begin_y; y < end_y; y++)
            getBlock(x, y).draw(x, y);
}

void block_engine::block::draw(unsigned int x, unsigned int y) {
    unique_block* this_block_type = &block_types.at(block_id);
    if(this_block_type->texture) {
        SDL_Rect rect = {0, 0, BLOCK_WIDTH, BLOCK_WIDTH};
        rect.x = int(x * BLOCK_WIDTH - position_x + swl::window_width / 2);
        rect.y = int(y * BLOCK_WIDTH - position_y + swl::window_height / 2);
        swl::render(this_block_type->texture, rect);
    }
}

block_engine::block& block_engine::getBlock(unsigned int x, unsigned int y) {
    return world.at(y * world_width + x);
}

unsigned int block_engine::getWorldHeight() {
    return (unsigned int)(world.size() / world_width);
}
