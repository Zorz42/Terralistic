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
#include <cmath>
#include <thread>

// terrain generation perameters

// TERRAIN
#define TERRAIN_VERTICAL_MULTIPLIER 140
#define TERRAIN_HORIZONTAL_DIVIDER 4
#define TERRAIN_HORIZONT (blockEngine::world_height / 2 - 50)

// CAVES

#define CAVE_START 0.71
#define CAVE_LENGTH 0.3
#define CAVE_CAP 0

#define STONE_START 0.65
#define STONE_LENGTH 1

//xPeriod and yPeriod together define the angle of the lines
//xPeriod and yPeriod both 0 ==> it becomes a normal clouds or turbulence pattern
#define X_PERIOD 0.5 //defines repetition of marble lines in x direction
#define Y_PERIOD 1.0 //defines repetition of marble lines in y direction
//turbPower = 0 ==> it becomes a normal sine pattern
#define TURB_POWER 5.0 //makes twists
#define TURB_SIZE 50.0 //initial size of the turbulence

#define CAVE_CONSERVATIVE 4
#define CAVE_SMOOTH 2

void stackDirt(unsigned int x, unsigned int height);

void generateSurface(unsigned int seed);
void generateCaves(unsigned int seed);
void generateStone(unsigned int seed);

void generateNoise();

unsigned int highest_height = CAVE_CAP;

#define LOADING_NEXT terrainGenerator::loading_current++;

int generateTerrainDaemon(unsigned int seed) {
    generateSurface(seed);
    generateCaves(seed);
    generateStone(seed);
    LOADING_NEXT
    for(unsigned long i = 0; i < blockEngine::world_width * blockEngine::world_height; i++)
        blockEngine::world[i].update();
    return 0;
}

void terrainGenerator::generateTerrain(unsigned int seed) {
    terrainGenerator::loading_total = 7 + CAVE_CONSERVATIVE + CAVE_SMOOTH;
    terrainGenerator::loading_current = 0;
    std::thread thread(generateTerrainDaemon, seed);
    
    generatingScreen();

    thread.join();
    
    if(loading_current != loading_total)
        swl::popupError("Loading total is " + std::to_string(loading_total) + ", but loading current got to " + std::to_string(loading_current));
}

void stackDirt(unsigned int x, unsigned int height) {
    for(unsigned int y = blockEngine::world_height - 1; y > blockEngine::world_height - height - 1; y--)
        blockEngine::getBlock(x, y).block_id = blockEngine::DIRT;
    blockEngine::getBlock(x, blockEngine::world_height - height - 1).block_id = blockEngine::GRASS_BLOCK;
}

double turbulence(double x, double y, double size, PerlinNoise& noise) {
  double value = 0.0, initialSize = size;

  while(size >= 1) {
    value += noise.noise(x / size, y / size) * size;
    size /= 2.0;
  }

  return value / initialSize;
}

void generateSurface(unsigned int seed) {
    PerlinNoise noise(seed);
    
    unsigned int heights[blockEngine::world_width];
    
    // generate terrain
    for(unsigned int x = 0; x < blockEngine::world_width; x++) {
        // apply multiple layers of perlin noise
        unsigned int height = TERRAIN_HORIZONT + TERRAIN_VERTICAL_MULTIPLIER * turbulence((double)x / TERRAIN_HORIZONTAL_DIVIDER, 0.8, 64, noise);
        
        heights[x] = height;
        if(height > highest_height)
            highest_height = height;
    }
    LOADING_NEXT
    
    srand(seed);
    
    // apply terrain to world
    for(unsigned int x = 0; x < blockEngine::world_width; x++) {
        stackDirt(x, heights[x]);
        if(rand() % 7 == 0) {
            blockEngine::getBlock(x, blockEngine::world_height - heights[x] - 2).block_id = blockEngine::STONE;
            //blockEngine::getBlock(x, blockEngine::world_height - heights[x] - 2).update();
        }
    }
    LOADING_NEXT
    
    blockEngine::getBlock(0, 0) = blockEngine::DIRT;
}

double terrainGenerator::turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, unsigned int highest_height, PerlinNoise noise) {
    double turb = ::turbulence(x, y, size, noise);
    
    double xy_value = x * x_period / blockEngine::world_width + y * y_period / highest_height + turb_power * turb / 2.0;
    return fabs(sin(xy_value * M_PI));
}

void generateCaves(unsigned int seed) {
    caveGenerator main_cm(blockEngine::world_width, highest_height - CAVE_CAP, seed);
    
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
    for(unsigned int x = 0; x < blockEngine::world_width; x++)
        for(unsigned int y = 0; main_cm.getElement(x, y) && y < 5; y++)
            main_cm.getElement(x, y) = 0;
    LOADING_NEXT
    
    // apply caves to the world
    for(unsigned int y = CAVE_CAP; y < highest_height; y++)
        for(unsigned int x = 0; x < blockEngine::world_width; x++)
            if(main_cm.getElement(x, y - CAVE_CAP))
                blockEngine::getBlock(x, y + (blockEngine::world_height - highest_height)).block_id = blockEngine::AIR;
    LOADING_NEXT
}


void generateStone(unsigned int seed) {
    PerlinNoise noise(seed);
    for(unsigned int y = blockEngine::world_height - highest_height; y < blockEngine::world_height; y++)
        for(unsigned int x = 0; x < blockEngine::world_width; x++)
            if(blockEngine::getBlock(x, y).block_id && terrainGenerator::turbulence(x, y, TURB_SIZE, X_PERIOD, Y_PERIOD, TURB_POWER, highest_height, noise) > STONE_START + STONE_LENGTH - (double)y / highest_height * STONE_LENGTH)
                blockEngine::getBlock(x, y).block_id = blockEngine::STONE_BLOCK;
    LOADING_NEXT
}
