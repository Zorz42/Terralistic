//
//  terrainGenerator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "blockEngine.hpp"
#include "terrainGenerator.hpp"
#include "singleWindowLibrary.hpp"
#include "simplex-noise.hpp"
#include <cmath>
#include <thread>

// terrain generation parameters

// TERRAIN
#define TERRAIN_VERTICAL_MULTIPLIER 140
#define TERRAIN_HORIZONTAL_DIVIDER 4
#define TERRAIN_HORIZONT (blockEngine::world_height / 2 - 50)

// CAVES

#define CAVE_START 0.15
#define CAVE_LENGTH 0.3

#define STONE_START 0.69
#define STONE_LENGTH 1

//xPeriod and yPeriod together define the angle of the lines
//xPeriod and yPeriod both 0 ==> it becomes a normal clouds or turbulence pattern
#define X_PERIOD 0.5 //defines repetition of marble lines in x direction
#define Y_PERIOD 1.0 //defines repetition of marble lines in y direction
//turbPower = 0 ==> it becomes a normal sine pattern
#define TURB_POWER 5.0 //makes twists
#define TURB_SIZE 64.0 //initial size of the turbulence

void stackDirt(unsigned int x, unsigned int height);

void generateSurface(unsigned int seed);
void generateCaves(unsigned int seed);
void generateStone(unsigned int seed);

unsigned int highest_height = 0;

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
    terrainGenerator::loading_total = 5;
    terrainGenerator::loading_current = 0;
    std::thread thread(generateTerrainDaemon, seed);
    
    generatingScreen();

    thread.join();
    
    if(loading_current != loading_total)
        swl::popupError("Loading total is " + std::to_string(loading_total) + ", but loading current got to " + std::to_string(loading_current));
}

void stackDirt(unsigned int x, unsigned int height) {
    for(unsigned int y = static_cast<unsigned int>(blockEngine::world_height - 1); y > blockEngine::world_height - height - 1; y--)
        blockEngine::getBlock(static_cast<unsigned short>(x), static_cast<unsigned short>(y)).block_id = blockEngine::DIRT;
    blockEngine::getBlock(static_cast<unsigned short>(x),
                          static_cast<unsigned short>(blockEngine::world_height - height - 1)).block_id = blockEngine::GRASS_BLOCK;
}

double turbulence(double x, double y, double size, PerlinNoise& noise) {
  double value = 0.0, initialSize = size;

  while(size >= 2) {
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
        unsigned int height = static_cast<unsigned int>(TERRAIN_HORIZONT + TERRAIN_VERTICAL_MULTIPLIER * turbulence(
                (double) x / TERRAIN_HORIZONTAL_DIVIDER, 0.8, 64, noise));
        
        heights[x] = height;
        if(height > highest_height)
            highest_height = height;
    }
    LOADING_NEXT
    
    srand(seed);
    
    // apply terrain to world
    for(unsigned int x = 0; x < blockEngine::world_width; x++) {
        stackDirt(x, heights[x]);
        if(rand() % 7 == 0) // generate stones
            blockEngine::getBlock(static_cast<unsigned short>(x),
                                  static_cast<unsigned short>(blockEngine::world_height - heights[x] - 2)).block_id = blockEngine::STONE;
    }
    LOADING_NEXT
    
    blockEngine::getBlock(0, 0).block_id = blockEngine::DIRT;
}

double terrainGenerator::turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, PerlinNoise noise) {
    double turb = ::turbulence(x, y, size, noise);
    
    double xy_value = x * x_period / blockEngine::world_width + y * y_period / highest_height + turb_power * turb / 2.0;
    return fabs(sin(xy_value * M_PI));
}

void generateCaves(unsigned int seed) {
    osn_context* ctx;
    open_simplex_noise(seed, &ctx);
    
    for(unsigned int y = 0; y < highest_height; y++)
        for(unsigned int x = 0; x < blockEngine::world_width; x++)
            if(open_simplex_noise2(ctx, (double)x / 10, (double)y / 10) > CAVE_START + CAVE_LENGTH - (double)y / highest_height * CAVE_LENGTH)
                blockEngine::getBlock(static_cast<unsigned short>(x),
                                      static_cast<unsigned short>(y + (blockEngine::world_height - highest_height))).block_id = blockEngine::AIR;
    open_simplex_noise_free(ctx);
    LOADING_NEXT
}

void generateStone(unsigned int seed) {
    PerlinNoise noise(seed);
    for(unsigned int y = blockEngine::world_height - highest_height; y < blockEngine::world_height; y++)
        for(unsigned int x = 0; x < blockEngine::world_width; x++)
            if(blockEngine::getBlock(static_cast<unsigned short>(x), static_cast<unsigned short>(y)).block_id && terrainGenerator::turbulence(x, y, TURB_SIZE, X_PERIOD, Y_PERIOD, TURB_POWER, noise) > STONE_START + STONE_LENGTH - (double)y / highest_height * STONE_LENGTH)
                blockEngine::getBlock(static_cast<unsigned short>(x), static_cast<unsigned short>(y)).block_id = blockEngine::STONE_BLOCK;
    LOADING_NEXT
}
