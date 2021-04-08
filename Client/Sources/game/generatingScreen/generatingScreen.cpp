//
//  generatingScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/07/2020.
//

#include "generatingScreen.hpp"
#include "terrainGenerator.hpp"
#include "dev.hpp"

#define TEXT_SCALE 3

void generatingScreen::init() {
    loading_text.scale = TEXT_SCALE;
    loading_text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    loading_text.setTexture(gfx::renderText("Generating world", {255, 255, 255}));
    loading_text.orientation = gfx::center;

    
    terrainGenerator::loading_total = 6;
    terrainGenerator::loading_current = 0;
    thread = std::thread(terrainGenerator::generateTerrainDaemon, seed, &world_map);
}

void generatingScreen::update() {
    loading_bar.w += (terrainGenerator::loading_current * LOADING_RECT_WIDTH / terrainGenerator::loading_total - loading_bar.w) / 3;
    loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
    
    if(terrainGenerator::loading_current == terrainGenerator::loading_total)
        gfx::returnFromScene();
}

void generatingScreen::render() {
    gfx::render(loading_text);
    gfx::render(loading_bar_back);
    gfx::render(loading_bar);
}

void generatingScreen::stop() {
    thread.join();
    
    ASSERT(terrainGenerator::loading_current == terrainGenerator::loading_total, "Loading total is " + std::to_string(terrainGenerator::loading_total) + ", but loading current got to " + std::to_string(terrainGenerator::loading_current));
}
