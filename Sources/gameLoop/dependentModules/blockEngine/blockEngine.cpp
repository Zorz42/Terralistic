//
//  core.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include <algorithm>
#include "singleWindowLibrary.hpp"
#include "blockEngine.hpp"

void grass_block_leftClickEvent(blockEngine::block* block) {
    block->block_id = blockEngine::DIRT;
}

void air_rightClickEvent(blockEngine::block* block) {
    block->block_id = blockEngine::STONE;
    blockEngine::updateNearestBlocks(block->getX(), block->getY());
}

void blockEngine::init() {
    std::vector<std::string> block_types_arr = {
        "air",
        "dirt",
        "stone_block",
        "grass_block",
        "stone",
    };
    
    for(auto& i : block_types_arr)
        block_types.push_back(unique_block(i));
    
    std::vector<std::pair<blockType, blockType>> connections = {
        {GRASS_BLOCK, DIRT},
    };
    
    for(std::pair<blockType, blockType> i : connections) {
        block_types.at(i.first).connects_to.push_back(i.second);
        block_types.at(i.second).connects_to.push_back(i.first);
    }
    
    block_types.at(GRASS_BLOCK).leftClickEvent = &grass_block_leftClickEvent;
    block_types.at(AIR).rightClickEvent = &air_rightClickEvent;
    block_types.at(AIR).ghost = true;
    block_types.at(STONE).ghost = true;
    block_types.at(STONE).only_on_floor = true;
    block_types.at(AIR).transparent = true;
    block_types.at(STONE).transparent = true;
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

void blockEngine::block::draw() {
#define THIS_BLOCK_TYPE block_types.at(block_id)
    if(THIS_BLOCK_TYPE.texture) {
        SDL_Rect rect = getRect(), cutout_rect = {0, 8 * block_orientation, 8, 8};
        swl::render(THIS_BLOCK_TYPE.texture, rect, cutout_rect);
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

void blockEngine::block::update() {
    block_orientation = 0;
    
    if(block_types.at(block_id).only_on_floor) {
        if(block_types.at(getBlock(getX(), getY() + 1).block_id).transparent)
            getBlock(getX(), getY()).block_id = AIR;
    }
    
    if(!block_types.at(block_id).single_texture) {
        char x[] = {0, 1, 0, -1};
        char y[] = {-1, 0, 1, 0};
        Uint8 c = 1;
        for(int i = 0; i < 4; i++) {
            if(getX() + x[i] >= world_width || getX() + x[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(getY() + y[i] >= world_height || getY() + y[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(getBlock(getX() + x[i], getY() + y[i]).block_id == block_id || std::count(block_types.at(block_id).connects_to.begin(), block_types.at(block_id).connects_to.end(), getBlock(getX() + x[i], getY() + y[i]).block_id))
                block_orientation += c;
            c += c;
        }
    }
}

blockEngine::unique_block::unique_block(std::string name) : name(name) {
    int h = 0;
    texture = name == "air" ? nullptr : swl::loadTextureFromFile("texturePack/blocks/" + name + ".png", nullptr, &h);
    single_texture = h == 8;
}

void blockEngine::updateNearestBlocks(int x, int y) {
    char x_[] = {0, 0, 0, -1, 1};
    char y_[] = {0, -1, 1, 0, 0};
    for(char i = 0; i < 5; i++)
        blockEngine::getBlock(x + x_[i], y + y_[i]).update();
}

void blockEngine::rightClickEvent(int x, int y) {
    block* block = &getBlock(x, y);
    if(block_types.at(block->block_id).rightClickEvent)
        block_types.at(block->block_id).rightClickEvent(block);
}

void blockEngine::leftClickEvent(int x, int y) {
    block* block = &getBlock(x, y);
    if(block_types.at(block->block_id).leftClickEvent)
        block_types.at(block->block_id).leftClickEvent(block);
    else {
        getBlock(x, y) = blockEngine::AIR;
        updateNearestBlocks(x, y);
    }
}
