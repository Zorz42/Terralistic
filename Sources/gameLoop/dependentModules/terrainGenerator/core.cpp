//
//  terrainGenerator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "caveGenerator.hpp"
#include "singleWindowLibrary.hpp"

#include <SDL2/SDL_thread.h>

// terrain generation perameters

// TERRAIN
#define TERRAIN_VERTICAL_MULTIPLIER 70
#define TERRAIN_HORIZONTAL_DIVIDER 3

// CAVES

#define CAVE_START 0.7
#define CAVE_LENGTH 0.3
#define CAVE_CAP 30

//xPeriod and yPeriod together define the angle of the lines
//xPeriod and yPeriod both 0 ==> it becomes a normal clouds or turbulence pattern
#define X_PERIOD 0.0 //defines repetition of marble lines in x direction
#define Y_PERIOD 1.0 //defines repetition of marble lines in y direction
//turbPower = 0 ==> it becomes a normal sine pattern
#define TURB_POWER 5.0 //makes twists
#define TURB_SIZE 32.0 //initial size of the turbulence

#define CAVE_CONSERVATIVE 4
#define CAVE_SMOOTH 2

void stackBlocks(unsigned int x, unsigned int height);

void generateSurface(unsigned int seed);
void generateCaves(unsigned int seed);

void generateNoise();

unsigned int highest_height = CAVE_CAP;

#define LOADING_NEXT terrainGenerator::loading_current++;

int generateTerrainDaemon(void* seed) {
    unsigned int seed_int = *((unsigned int*)seed);
    terrainGenerator::loading_total = 5 + CAVE_CONSERVATIVE + CAVE_SMOOTH;
    generateSurface(seed_int);
    generateCaves(seed_int);
    return 0;
}

void terrainGenerator::generateTerrain(unsigned int seed) {
    SDL_Thread *thread = SDL_CreateThread(generateTerrainDaemon, "terrain generator", (void*)&seed);
    
    generatingScreen();
    
    SDL_WaitThread(thread, nullptr);
    
    if(loading_current != loading_total)
        swl::popupError("Loading total is " + std::to_string(loading_total) + ", but loading current got to " + std::to_string(loading_current));
}

void stackBlocks(unsigned int x, unsigned int height) {
    for(unsigned int y = block_engine::getWorldHeight() - 1; y > block_engine::getWorldHeight() - height; y--)
        block_engine::getBlock(x, y).block_id = block_engine::BLOCK_DIRT;
}

void generateSurface(unsigned int seed) {
#define TERRAIN_HORIZONT (block_engine::getWorldHeight() / 2)
    PerlinNoise noise(seed);
    
    unsigned int heights[block_engine::world_width];
    
    // generate terrain
    for(unsigned int x = 0; x < block_engine::world_width; x++) {
        // apply multiple layers of perlin noise
        unsigned int height = TERRAIN_HORIZONT + TERRAIN_VERTICAL_MULTIPLIER * turbulence((double)x / TERRAIN_HORIZONTAL_DIVIDER, 0.8, 64, noise);
        
        heights[x] = height;
        if(height > highest_height)
            highest_height = height;
    }
    LOADING_NEXT
    /*// make terrain softer
    for(unsigned int x = 1; x < block_engine::world_width - 1; x++)
        heights[x] = (heights[x-1] + heights[x+1]) / 2;*/
    
    // apply terrain to world
    for(unsigned int x = 0; x < block_engine::world_width; x++)
        stackBlocks(x, heights[x]);
    LOADING_NEXT
}

void generateCaves(unsigned int seed) {
    caveGenerator main_cm(block_engine::world_width, highest_height - CAVE_CAP, seed);
    
    main_cm.generateMap(CAVE_START, CAVE_LENGTH, CAVE_CAP, X_PERIOD, Y_PERIOD, TURB_POWER, TURB_SIZE, highest_height);
    LOADING_NEXT
    // smoothen map
    for(int i = 0; i < CAVE_CONSERVATIVE; i++) {
        main_cm.evolveMap(caveGenerator::CM_CONSERVATIVE);
        LOADING_NEXT
    }
    for(int i = 0; i < CAVE_SMOOTH; i++) {
        main_cm.evolveMap(caveGenerator::CM_SMOOTH);
        LOADING_NEXT
    }
    
    // make a transition from cave cap
    for(unsigned int x = 0; x < block_engine::world_width; x++)
        for(unsigned int y = 0; main_cm.getElement(x, y) && y < 5; y++)
            main_cm.getElement(x, y) = 0;
    LOADING_NEXT
    
    // apply caves to the world
    for(unsigned int y = CAVE_CAP; y < highest_height; y++)
        for(unsigned int x = 0; x < block_engine::world_width; x++)
            if(main_cm.getElement(x, y - CAVE_CAP))
                block_engine::getBlock(x, y + (block_engine::getWorldHeight() - highest_height)).block_id = block_engine::BLOCK_AIR;
    LOADING_NEXT
}
