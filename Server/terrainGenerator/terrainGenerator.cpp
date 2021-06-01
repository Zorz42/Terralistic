//
//  terrainGenerator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "SimplexNoise.h"
#include <random>
#include "serverMap.hpp"
#include "terrainGenerator.hpp"

// terrain generation parameters

// TERRAIN
#define TERRAIN_VERTICAL_MULTIPLIER 140
#define TERRAIN_HORIZONTAL_DIVIDER 4
#define TERRAIN_HORIZONT ((float)world_serverMap.getWorldHeight() / 2 - 50)

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

void generateSurface(std::mt19937& engine, serverMap& world_serverMap);
void generateCaves(std::mt19937& engine, serverMap& world_serverMap);
void generateStone(std::mt19937& engine, serverMap& world_serverMap);
void generateTrees(std::mt19937& engine, serverMap& world_serverMap);

static unsigned int highest_height = 0;
static unsigned short* heights;

#define LOADING_NEXT terrainGenerator::generating_current++;

int terrainGenerator::generateTerrainDaemon(unsigned int seed, serverMap* world_serverMap) {
    std::mt19937 engine(seed);    
    generating_current = 0;
    

    generateSurface(engine, *world_serverMap);
    generateCaves(engine, *world_serverMap);
    generateStone(engine, *world_serverMap);
    generateTrees(engine, *world_serverMap);
    world_serverMap->setNaturalLight();
    LOADING_NEXT
    return 0;
}

void stackDirt(unsigned int x, unsigned int height, serverMap& world_serverMap) {
    for(auto y = (unsigned int)(world_serverMap.getWorldHeight() - 1); y > world_serverMap.getWorldHeight() - height - 1; y--)
        world_serverMap.getBlock((unsigned short)x, (unsigned short)y).setType(serverMap::blockType::DIRT, false);
    world_serverMap.getBlock((unsigned short)x, (unsigned short)(world_serverMap.getWorldHeight() - height - 1)).setType(serverMap::blockType::GRASS_BLOCK, false);
}

double turbulence(double x, double y, double size, SimplexNoise& noise) {

//double turbulence(double x, double y, double size, double x_period, double y_period, double turb_power, PerlinNoise noise) {
    double value = 0.0, initialSize = size;
    
    while(size >= 2) {
        value += noise.noise(x / size, y / size) * size;
        size /= 2.0;
    }
    
    return value / initialSize;
    
    //double xy_value = x * x_period / world_serverMap.getWorldWidth() + y * y_period / highest_height + turb_power * value / initialSize / 2.0;
    //return fabs(sin(xy_value * M_PI));
}

void generateSurface(std::mt19937& engine, serverMap& world_serverMap) {
    SimplexNoise noise(engine());



    
    heights = new unsigned short[world_serverMap.getWorldWidth()];
    
    // generate terrain
    for(unsigned int x = 0; x < world_serverMap.getWorldWidth(); x++) {
        // apply multiple layers of perlin noise
        auto height = (unsigned int)(TERRAIN_HORIZONT + TERRAIN_VERTICAL_MULTIPLIER * turbulence((double) x / TERRAIN_HORIZONTAL_DIVIDER, 0.8, TURB_SIZE, noise));
        
        /*unsigned short heat = (int)((noise.noise((float)x / 550 + 0.25) + 1) * 3);
        heat = heat / 2;
        if (heat == 3)
            heat = 2;

        unsigned short biomeheight = (int)((noise.noise((float)x / 300 + 0.001) + 1) * 4);
        biomeheight /= 2;
        if (biomeheight == 4)
            biomeheight = 3;

        switch (heat)
        {
        case 0:
            switch (biomeheight)
            {
            case 0://icy seas

                break;
            case 1://snowy tundra

                break;
            case 2://cold hills (with taiga trees?)

                break;
            case 3://snowy mountains

                break;
            }
            break;

        case 1:
            switch (biomeheight)
            {
            case 0://regular sea

                break;
            case 1://plains

                break;
            case 2://forest

                break;
            case 3://regular mountain

                break;
            }
            break;

        case 2:
            switch (biomeheight)
            {
            case 0://warm ocean

                break;
            case 1://desert

                break;
            case 2://savana

                break;
            case 3://savana mountains

            break;
            }
            break;
        }*/


        heights[x] = height;
        if(height > highest_height)
            highest_height = height;
    }
    LOADING_NEXT

    // apply terrain to world
    for(unsigned int x = 0; x < world_serverMap.getWorldWidth(); x++) {
        stackDirt(x, heights[x], world_serverMap);
        if(!(engine() % 7)) // generate stones
            world_serverMap.getBlock((unsigned short)x, (unsigned short)(world_serverMap.getWorldHeight() - heights[x] - 2)).setType(serverMap::blockType::STONE, false);
    }
    LOADING_NEXT
    
    world_serverMap.getBlock(0, 0).setType(serverMap::blockType::DIRT, false);
}

void generateCaves(std::mt19937& engine, serverMap& world_serverMap) {
    SimplexNoise noise(engine());
    
    for(unsigned int y = 0; y < highest_height; y++)
        for(unsigned int x = 0; x < world_serverMap.getWorldWidth(); x++)
            if(noise.noise((double)x / 10, (double)y / 10) > CAVE_START + CAVE_LENGTH - (double)y / highest_height * CAVE_LENGTH)
                world_serverMap.getBlock((unsigned short)x, (unsigned short)(y + (world_serverMap.getWorldHeight() - highest_height))).setType(serverMap::blockType::AIR, false);
    LOADING_NEXT
}

void generateStone(std::mt19937& engine, serverMap& world_serverMap) {
    SimplexNoise noise(engine());
    
    for(unsigned int y = world_serverMap.getWorldHeight() - highest_height; y < world_serverMap.getWorldHeight(); y++)
        for(unsigned int x = 0; x < world_serverMap.getWorldWidth(); x++)
            if(world_serverMap.getBlock((unsigned short)x, (unsigned short)y).getType() != serverMap::blockType::AIR && noise.noise((double)x / 10, (double)y / 10) > STONE_START + STONE_LENGTH - (double)y / highest_height * STONE_LENGTH)
                world_serverMap.getBlock((unsigned short)x, (unsigned short)y).setType(serverMap::blockType::STONE_BLOCK, false);
    LOADING_NEXT
}

void generateTrees(std::mt19937& engine, serverMap& world_serverMap) {
    unsigned short x = 0;
    while(true) {
        x += engine() % 4 + 6;
        if(x >= world_serverMap.getWorldWidth() - 5)
            break;
        if(world_serverMap.getBlock(x, world_serverMap.getWorldHeight() - heights[x] - 1).getType() != serverMap::blockType::GRASS_BLOCK)
            continue;
        unsigned short height = engine() % 5 + 10;
        unsigned short y;
        for(y = world_serverMap.getWorldHeight() - heights[x] - 2; y > world_serverMap.getWorldHeight() - heights[x] - height; y--)
            world_serverMap.getBlock(x, y).setType(serverMap::blockType::WOOD, false);
        
        for(unsigned short y_leave = y; y_leave < y + 5; y_leave++)
            for(unsigned short x_leave = x - 2; x_leave <= x + 2; x_leave++)
                if((x_leave != x - 2 || y_leave != y) && (x_leave != x + 2 || y_leave != y) && (x_leave != x - 2 || y_leave != y + 4) && (x_leave != x + 2 || y_leave != y + 4))
                    world_serverMap.getBlock(x_leave, y_leave).setType(serverMap::blockType::LEAVES, false);
        
        for(unsigned short y2 = world_serverMap.getWorldHeight() - heights[x] - engine() % 4 - 4; y2 > world_serverMap.getWorldHeight() - heights[x] - height + 5; y2 -= engine() % 4 + 2) {
            world_serverMap.getBlock(x - 1, y2).setType(serverMap::blockType::WOOD, false);
            world_serverMap.getBlock(x - 2, y2).setType(serverMap::blockType::LEAVES, false);
        }
        
        for(unsigned short y2 = world_serverMap.getWorldHeight() - heights[x] - engine() % 4 - 4; y2 > world_serverMap.getWorldHeight() - heights[x] - height + 5; y2 -= engine() % 4 + 2) {
            world_serverMap.getBlock(x + 1, y2).setType(serverMap::blockType::WOOD, false);
            world_serverMap.getBlock(x + 2, y2).setType(serverMap::blockType::LEAVES, false);
        }
        
        if(world_serverMap.getBlock(x - 1, world_serverMap.getWorldHeight() - heights[x] - 1).getType() == serverMap::blockType::GRASS_BLOCK && world_serverMap.getBlock(x - 2, world_serverMap.getWorldHeight() - heights[x] - 2).getType() == serverMap::blockType::AIR)
            world_serverMap.getBlock(x - 1, world_serverMap.getWorldHeight() - heights[x] - 2).setType(serverMap::blockType::WOOD, false);
        
        if(world_serverMap.getBlock(x + 1, world_serverMap.getWorldHeight() - heights[x] - 1).getType() == serverMap::blockType::GRASS_BLOCK && world_serverMap.getBlock(x + 2, world_serverMap.getWorldHeight() - heights[x] - 2).getType() == serverMap::blockType::AIR)
            world_serverMap.getBlock(x + 1, world_serverMap.getWorldHeight() - heights[x] - 2).setType(serverMap::blockType::WOOD, false);
    }
    LOADING_NEXT
}
