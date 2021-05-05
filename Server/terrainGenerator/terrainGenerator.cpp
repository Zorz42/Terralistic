//
//  terrainGenerator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "simplex-noise.hpp"
#include <random>
#include "serverMap.hpp"
#include "terrainGenerator.hpp"

// terrain generation parameters

// TERRAIN
#define TERRAIN_VERTICAL_MULTIPLIER 140
#define TERRAIN_HORIZONTAL_DIVIDER 4
#define TERRAIN_HORIZONT ((float)world_map.getWorldHeight() / 2 - 50)

// CAVES

#define CAVE_START 0.15
#define CAVE_LENGTH 0.3

#define STONE_START 0.1
#define STONE_LENGTH 1

//xPeriod and yPeriod together define the angle of the lines
//xPeriod and yPeriod both 0 ==> it becomes a normal clouds or turbulence pattern
//#define X_PERIOD 0.5 //defines repetition of marble lines in x direction
//#define Y_PERIOD 1.0 //defines repetition of marble lines in y direction
//turbPower = 0 ==> it becomes a normal sine pattern
//#define TURB_POWER 5.0 //makes twists
#define TURB_SIZE 64.0 //initial size of the turbulence

void stackDirt(unsigned int x, unsigned int height);

void generateSurface(std::mt19937& engine, map& world_map);
void generateCaves(std::mt19937& engine, map& world_map);
void generateStone(std::mt19937& engine, map& world_map);
void generateTrees(std::mt19937& engine, map& world_map);

static unsigned int highest_height = 0;
static unsigned short* heights;

#define LOADING_NEXT terrainGenerator::loading_current++;

int terrainGenerator::generateTerrainDaemon(unsigned int seed, map* world_map) {
    std::mt19937 engine(seed);
    
    generateSurface(engine, *world_map);
    generateCaves(engine, *world_map);
    generateStone(engine, *world_map);
    generateTrees(engine, *world_map);
    world_map->setNaturalLight();
    LOADING_NEXT
    return 0;
}

void stackDirt(unsigned int x, unsigned int height, map& world_map) {
    for(auto y = (unsigned int)(world_map.getWorldHeight() - 1); y > world_map.getWorldHeight() - height - 1; y--)
        world_map.getBlock((unsigned short)x, (unsigned short)y).setType(map::blockType::DIRT, false);
    world_map.getBlock((unsigned short)x, (unsigned short)(world_map.getWorldHeight() - height - 1)).setType(map::blockType::GRASS_BLOCK, false);
}

double turbulence(double x, double y, double size, osn_context* ctx) {

//double turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, PerlinNoise noise) {
    double value = 0.0, initialSize = size;

    while(size >= 2) {
        value += open_simplex_noise2(ctx, x / size, y / size) * size;
        size /= 2.0;
    }
    
    return value / initialSize;
    
    //double xy_value = x * x_period / world_map.getWorldWidth() + y * y_period / highest_height + turb_power * value / initialSize / 2.0;
    //return fabs(sin(xy_value * M_PI));
}

void generateSurface(std::mt19937& engine, map& world_map) {
    osn_context* ctx;
    open_simplex_noise(engine(), &ctx);
    
    heights = new unsigned short[world_map.getWorldWidth()];
    
    // generate terrain
    for(unsigned int x = 0; x < world_map.getWorldWidth(); x++) {
        // apply multiple layers of perlin noise
        auto height = (unsigned int)(TERRAIN_HORIZONT + TERRAIN_VERTICAL_MULTIPLIER * turbulence((double) x / TERRAIN_HORIZONTAL_DIVIDER, 0.8, TURB_SIZE, ctx));
        
        heights[x] = height;
        if(height > highest_height)
            highest_height = height;
    }
    LOADING_NEXT

    // apply terrain to world
    for(unsigned int x = 0; x < world_map.getWorldWidth(); x++) {
        stackDirt(x, heights[x], world_map);
        if(!(engine() % 7)) // generate stones
            world_map.getBlock((unsigned short)x, (unsigned short)(world_map.getWorldHeight() - heights[x] - 2)).setType(map::blockType::STONE, false);
    }
    LOADING_NEXT
    
    world_map.getBlock(0, 0).setType(map::blockType::DIRT, false);
}

void generateCaves(std::mt19937& engine, map& world_map) {
    osn_context* ctx;
    open_simplex_noise(engine(), &ctx);
    
    for(unsigned int y = 0; y < highest_height; y++)
        for(unsigned int x = 0; x < world_map.getWorldWidth(); x++)
            if(open_simplex_noise2(ctx, (double)x / 10, (double)y / 10) > CAVE_START + CAVE_LENGTH - (double)y / highest_height * CAVE_LENGTH)
                world_map.getBlock((unsigned short)x, (unsigned short)(y + (world_map.getWorldHeight() - highest_height))).setType(map::blockType::AIR, false);
    open_simplex_noise_free(ctx);
    LOADING_NEXT
}

void generateStone(std::mt19937& engine, map& world_map) {
    osn_context* ctx;
    open_simplex_noise(engine(), &ctx);
    
    for(unsigned int y = world_map.getWorldHeight() - highest_height; y < world_map.getWorldHeight(); y++)
        for(unsigned int x = 0; x < world_map.getWorldWidth(); x++)
            if(world_map.getBlock((unsigned short)x, (unsigned short)y).getType() != map::blockType::AIR && open_simplex_noise2(ctx, (double)x / 10, (double)y / 10) > STONE_START + STONE_LENGTH - (double)y / highest_height * STONE_LENGTH)
                world_map.getBlock((unsigned short)x, (unsigned short)y).setType(map::blockType::STONE_BLOCK, false);
    LOADING_NEXT
}

void generateTrees(std::mt19937& engine, map& world_map) {
    unsigned short x = 0;
    while(true) {
        x += engine() % 4 + 6;
        if(x >= world_map.getWorldWidth() - 5)
            break;
        if(world_map.getBlock(x, world_map.getWorldHeight() - heights[x] - 1).getType() != map::blockType::GRASS_BLOCK)
            continue;
        unsigned short height = engine() % 5 + 10;
        unsigned short y;
        for(y = world_map.getWorldHeight() - heights[x] - 2; y > world_map.getWorldHeight() - heights[x] - height; y--)
            world_map.getBlock(x, y).setType(map::blockType::WOOD, false);
        
        for(unsigned short y_leave = y; y_leave < y + 5; y_leave++)
            for(unsigned short x_leave = x - 2; x_leave <= x + 2; x_leave++)
                if((x_leave != x - 2 || y_leave != y) && (x_leave != x + 2 || y_leave != y) && (x_leave != x - 2 || y_leave != y + 4) && (x_leave != x + 2 || y_leave != y + 4))
                    world_map.getBlock(x_leave, y_leave).setType(map::blockType::LEAVES, false);
        
        for(unsigned short y2 = world_map.getWorldHeight() - heights[x] - engine() % 4 - 4; y2 > world_map.getWorldHeight() - heights[x] - height + 5; y2 -= engine() % 4 + 2) {
            world_map.getBlock(x - 1, y2).setType(map::blockType::WOOD, false);
            world_map.getBlock(x - 2, y2).setType(map::blockType::LEAVES, false);
        }
        
        for(unsigned short y2 = world_map.getWorldHeight() - heights[x] - engine() % 4 - 4; y2 > world_map.getWorldHeight() - heights[x] - height + 5; y2 -= engine() % 4 + 2) {
            world_map.getBlock(x + 1, y2).setType(map::blockType::WOOD, false);
            world_map.getBlock(x + 2, y2).setType(map::blockType::LEAVES, false);
        }
        
        if(world_map.getBlock(x - 1, world_map.getWorldHeight() - heights[x] - 1).getType() == map::blockType::GRASS_BLOCK && world_map.getBlock(x - 2, world_map.getWorldHeight() - heights[x] - 2).getType() == map::blockType::AIR)
            world_map.getBlock(x - 1, world_map.getWorldHeight() - heights[x] - 2).setType(map::blockType::WOOD, false);
        
        if(world_map.getBlock(x + 1, world_map.getWorldHeight() - heights[x] - 1).getType() == map::blockType::GRASS_BLOCK && world_map.getBlock(x + 2, world_map.getWorldHeight() - heights[x] - 2).getType() == map::blockType::AIR)
            world_map.getBlock(x + 1, world_map.getWorldHeight() - heights[x] - 2).setType(map::blockType::WOOD, false);
    }
    LOADING_NEXT
}
