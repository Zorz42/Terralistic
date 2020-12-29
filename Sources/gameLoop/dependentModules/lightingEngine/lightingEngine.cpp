//
//  lightingEngine.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/12/2020.
//

#include "lightingEngine.hpp"
#include "blockEngine.hpp"

void lightingEngine::init() {
    
}

void lightingEngine::prepare() {
    light_map = new lightBlock[blockEngine::world_width * blockEngine::world_height];
}

void lightingEngine::close() {
    delete[] light_map;
}

void lightingEngine::removeNaturalLight(unsigned short x) {
    for(unsigned short y = 0; blockEngine::getBlock(x, y).getUniqueBlock().transparent; y++)
        removeLightSource(x, y);
}

void lightingEngine::setNaturalLight(unsigned short x) {
    for(unsigned short y = 0; blockEngine::getBlock(x, y).getUniqueBlock().transparent; y++)
        addLightSource(x, y, MAX_LIGHT);
}

lightingEngine::lightBlock& lightingEngine::getLightBlock(unsigned short x, unsigned short y) {
    return light_map[y * blockEngine::world_width + x];
}

void lightingEngine::addLightSource(unsigned short x, unsigned short y, unsigned char power) {
    lightBlock& block = getLightBlock(x, y);
    block.source = true;
    block.level = power;
    block.update();
}

void lightingEngine::removeLightSource(unsigned short x, unsigned short y) {
    lightBlock& block = getLightBlock(x, y);
    block.source = false;
    block.level = 0;
    block.update();
}
