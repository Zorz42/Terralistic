//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"

void blockEngine::init() {
    world_height = 1200;
    world_width = 4200;
    world = new block[world_width * world_height];
    
    block_types.push_back(unique_block("air"));
    block_types.at(0).texture = nullptr;
    block_types.push_back(unique_block("dirt"));
    block_types.at(1).texture = swl::loadTextureFromFile("texturePack/dirt_block.png");
    
    position_x = world_width / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    position_y = world_height / 2 * BLOCK_WIDTH - 100 * BLOCK_WIDTH;
    view_x = position_x;
    view_y = position_y;
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

void blockEngine::block::draw() {
    unique_block* this_block_type = &block_types.at(block_id);
    if(this_block_type->texture) {
        SDL_Rect rect = getRect();
        swl::render(this_block_type->texture, rect);
    }
}

blockEngine::block& blockEngine::getBlock(unsigned int x, unsigned int y) {
    return world[y * world_width + x];
}

SDL_Rect blockEngine::block::getRect() {
    SDL_Rect rect = {0, 0, BLOCK_WIDTH, BLOCK_WIDTH};
    rect.x = int(getX() * BLOCK_WIDTH - view_x + swl::window_width / 2);
    rect.y = int(getY() * BLOCK_WIDTH - view_y + swl::window_height / 2);
    return rect;
}

unsigned int blockEngine::block::getX() {
    return (unsigned int)(this - world) % world_width;
}

unsigned int blockEngine::block::getY() {
    return (unsigned int)(this - world) / world_width;
}
