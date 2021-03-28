//
//  generatingScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/07/2020.
//

#include "core.hpp"

#include "generatingScreen.hpp"
#include "game.hpp"
#include "main.hpp"

#define TEXT_SCALE 3
    
#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

gfx::sprite loading_text;
gfx::rect loading_bar_back, loading_bar;

INIT_SCRIPT
    loading_text.scale = TEXT_SCALE;
    loading_text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    loading_text.setTexture(gfx::renderText("Generating world", {255, 255, 255}));
    loading_text.orientation = gfx::center;

    
    loading_bar_back.h = LOADING_RECT_HEIGHT;
    loading_bar_back.w = LOADING_RECT_WIDTH;
    loading_bar_back.c = {100, 100, 100};
    loading_bar_back.y = -LOADING_RECT_ELEVATION;
    loading_bar_back.orientation = gfx::bottom;

    loading_bar.h = LOADING_RECT_HEIGHT;
    loading_bar.c = {255, 255, 255};
    loading_bar.y = -LOADING_RECT_ELEVATION;
    loading_bar.orientation = gfx::bottom;
INIT_SCRIPT_END

void terrainGenerator::scene::init() {
    terrainGenerator::loading_total = 6;
    terrainGenerator::loading_current = 0;
    thread = std::thread(terrainGenerator::generateTerrainDaemon, seed);
}

void terrainGenerator::scene::update() {
    loading_bar.w = terrainGenerator::loading_current * LOADING_RECT_WIDTH / terrainGenerator::loading_total;
    loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
    
    if(terrainGenerator::loading_current == terrainGenerator::loading_total)
        gfx::returnFromScene();
}

void terrainGenerator::scene::render() {
    gfx::render(loading_text);
    gfx::render(loading_bar_back);
    gfx::render(loading_bar);
}

void terrainGenerator::scene::stop() {
    thread.join();
    
    ASSERT(terrainGenerator::loading_current == terrainGenerator::loading_total, "Loading total is " + std::to_string(terrainGenerator::loading_total) + ", but loading current got to " + std::to_string(terrainGenerator::loading_current));
}
