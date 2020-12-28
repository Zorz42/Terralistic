//
//  lightingEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/12/2020.
//

#include "lightingEngine.hpp"
#include "blockEngine.hpp"
#include "singleWindowLibrary.hpp"


#include <iostream>

void lightingEngine::init() {
    
}

void lightingEngine::prepare() {
    light_map = new lightBlock[blockEngine::world_width * blockEngine::world_height];
}

void lightingEngine::close() {
    delete[] light_map;
}

void lightingEngine::lightBlock::render(short x, short y) {
    SDL_Rect rect = {x, y, BLOCK_WIDTH, BLOCK_WIDTH};
    swl::setDrawColor(0, 0, 0, 255 - 255.0 / MAX_LIGHT * level);
    swl::render(rect);
}

void lightingEngine::lightBlock::update() {
    std::vector<lightBlock*> neighbors;
    if(getX() != 0)
        neighbors.push_back(&getLightBlock(getX() - 1, getY()));
    if(getX() != blockEngine::world_width - 1)
        neighbors.push_back(&getLightBlock(getX() + 1, getY()));
    if(getY() != 0)
        neighbors.push_back(&getLightBlock(getX(), getY() - 1));
    if(getY() != blockEngine::world_height - 1)
        neighbors.push_back(&getLightBlock(getX(), getY() + 1));
    bool update_neighbors = false;
    if(source)
        update_neighbors = true;
    else {
        unsigned char level_to_be = 0;
        for(lightBlock* neighbor : neighbors)
            if(neighbor->level > level_to_be)
                level_to_be = neighbor->level;
        if(!level_to_be)
            return;
        level_to_be--;
        if(level_to_be != level) {
            level = level_to_be;
            update_neighbors = true;
        }
    }
    if(update_neighbors)
        for(lightBlock* neighbor : neighbors)
            if(!neighbor->source)
                neighbor->update();
}

unsigned short lightingEngine::lightBlock::getX() {
    return (unsigned int)(this - light_map) % blockEngine::world_width;
}

unsigned short lightingEngine::lightBlock::getY() {
    return (unsigned int)(this - light_map) / blockEngine::world_width;
}

void lightingEngine::removeNaturalLight(unsigned short x) {
    for(unsigned short y = 0; blockEngine::getBlock(x, y).getUniqueBlock().transparent; y++) {
        getLightBlock(x, y).source = false;
        getLightBlock(x, y).level = 0;
        getLightBlock(x, y).update();
    }
}

void lightingEngine::setNaturalLight(unsigned short x) {
    unsigned short y;
    for(y = 0; blockEngine::getBlock(x, y).getUniqueBlock().transparent; y++) {
        getLightBlock(x, y).source = true;
        getLightBlock(x, y).level = 10;
    }
    getLightBlock(x, y).update();
}

lightingEngine::lightBlock& lightingEngine::getLightBlock(unsigned short x, unsigned short y) {
    return light_map[y * blockEngine::world_width + x];
}
