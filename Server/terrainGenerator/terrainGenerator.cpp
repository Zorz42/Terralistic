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
#define TERRAIN_HORIZONT ((float)world_map.getWorldHeight() / 2 - 50)
#define TURB_SIZE 64.0 //initial size of the turbulence

// CAVES

#define CAVE_START 0.15
#define CAVE_LENGTH 0.3

// STONE

#define STONE_START 0.1
#define STONE_LENGTH 1

void stackDirt(unsigned int x, unsigned int height);

void generateSurface(std::mt19937& engine, serverMap& world_map);
void generateCaves(std::mt19937& engine, serverMap& world_map);
void generateStone(std::mt19937& engine, serverMap& world_map);
void generateTrees(std::mt19937& engine, serverMap& world_map);

void terrainGeneratorSwitch(unsigned int x, SimplexNoise& noise, serverMap& world_map);
void generatePlains(int x, SimplexNoise& noise, serverMap& world_map);
void generateDesert(int x, SimplexNoise& noise, serverMap& world_map);
void generateSnowyTundra(int x, SimplexNoise& noise, serverMap& world_map);

static unsigned int highest_height = 0;
static unsigned short* heights;

#define LOADING_NEXT terrainGenerator::generating_current++;

int terrainGenerator::generateTerrainDaemon(unsigned int seed, serverMap* world_map) {
    std::mt19937 engine(seed);
    SimplexNoise noise(engine());

    generating_current = 0;
    
    for (int x = 0; x < world_map->getWorldWidth(); x++) {
        terrainGeneratorSwitch(x, noise, *world_map);
    }
    /*generateSurface(engine, *world_map);
    generateCaves(engine, *world_map);
    generateStone(engine, *world_map);
    generateTrees(engine, *world_map);*/
    world_map->setNaturalLight();
    LOADING_NEXT
    return 0;
}

void stackDirt(unsigned int x, unsigned int height, serverMap& world_map) {
    for(auto y = (unsigned int)(world_map.getWorldHeight() - 1); y > world_map.getWorldHeight() - height - 1; y--)
        world_map.getBlock((unsigned short)x, (unsigned short)y).setType(serverMap::blockType::DIRT, false);
    world_map.getBlock((unsigned short)x, (unsigned short)(world_map.getWorldHeight() - height - 1)).setType(serverMap::blockType::GRASS_BLOCK, false);
    serverMap::block water_block = world_map.getBlock((unsigned short)x, (unsigned short)(world_map.getWorldHeight() - height - 2));
    water_block.setType(serverMap::liquidType::WATER, false);
    water_block.setLiquidLevel(127);
}

double turbulence(double x, double y, double size, SimplexNoise& noise) {
    double value = 0.0, initialSize = size;
    
    while(size >= 2) {
        value += noise.noise(x / size, y / size) * size;
        size /= 2.0;
    }
    
    return value / initialSize;
}


float heatGeneratorFloat(unsigned int x, SimplexNoise& noise) {
    return (noise.noise((float)x / 1100 + 0.125) + 1) * 1.5;
}

int heatGeneratorInt(unsigned int x, SimplexNoise& noise) {
    int heat = heatGeneratorFloat(x, noise);
    if (heat == 3)
        heat = 2;
    return heat;
}

float heightGeneratorFloat(unsigned int x, SimplexNoise& noise) {
    return (noise.noise((float)x / 300 + 0.001) + 1) * 2;
}

int heightGeneratorInt(unsigned int x, SimplexNoise& noise) {
    int heat = heatGeneratorFloat(x, noise);
    if (heat == 4)
        heat = 3;
    return heat;
}



void generateSurface(std::mt19937& engine, serverMap& world_map) {
    SimplexNoise noise(engine());

    
    heights = new unsigned short[world_map.getWorldWidth()];
    
    // generate terrain
    for(unsigned int x = 0; x < world_map.getWorldWidth(); x++) {
        // apply multiple layers of perlin noise
        auto height = (unsigned int)(TERRAIN_HORIZONT + TERRAIN_VERTICAL_MULTIPLIER * turbulence((double) x / TERRAIN_HORIZONTAL_DIVIDER, 0.8, TURB_SIZE, noise));
        
        


        heights[x] = height;
        if(height > highest_height)
            highest_height = height;
    }
    LOADING_NEXT

    // apply terrain to world
    for(unsigned int x = 0; x < world_map.getWorldWidth(); x++) {
        stackDirt(x, heights[x], world_map);
        if(!(engine() % 7)) // generate stones
            world_map.getBlock((unsigned short)x, (unsigned short)(world_map.getWorldHeight() - heights[x] - 2)).setType(serverMap::blockType::STONE, false);
    }
    LOADING_NEXT
}

void generateCaves(std::mt19937& engine, serverMap& world_map) {
    SimplexNoise noise(engine());
    
    for(unsigned int y = 0; y < highest_height; y++)
        for(unsigned int x = 0; x < world_map.getWorldWidth(); x++)
            if(noise.noise((double)x / 10, (double)y / 10) > CAVE_START + CAVE_LENGTH - (double)y / highest_height * CAVE_LENGTH)
                world_map.getBlock((unsigned short)x, (unsigned short)(y + (world_map.getWorldHeight() - highest_height))).setType(serverMap::blockType::AIR, false);
    LOADING_NEXT
}

void generateStone(std::mt19937& engine, serverMap& world_map) {
    SimplexNoise noise(engine());
    
    for(unsigned int y = world_map.getWorldHeight() - highest_height; y < world_map.getWorldHeight(); y++)
        for(unsigned int x = 0; x < world_map.getWorldWidth(); x++)
            if(world_map.getBlock((unsigned short)x, (unsigned short)y).getType() != serverMap::blockType::AIR && noise.noise((double)x / 10, (double)y / 10) > STONE_START + STONE_LENGTH - (double)y / highest_height * STONE_LENGTH)
                world_map.getBlock((unsigned short)x, (unsigned short)y).setType(serverMap::blockType::STONE_BLOCK, false);
    LOADING_NEXT
}

void generateTrees(std::mt19937& engine, serverMap& world_map) {
    unsigned short x = 0;
    while(true) {
        x += engine() % 4 + 6;
        if(x >= world_map.getWorldWidth() - 5)
            break;
        if(world_map.getBlock(x, world_map.getWorldHeight() - heights[x] - 1).getType() != serverMap::blockType::GRASS_BLOCK)
            continue;
        unsigned short height = engine() % 5 + 10;
        unsigned short y;
        for(y = world_map.getWorldHeight() - heights[x] - 2; y > world_map.getWorldHeight() - heights[x] - height; y--)
            world_map.getBlock(x, y).setType(serverMap::blockType::WOOD, false);
        
        for(unsigned short y_leave = y; y_leave < y + 5; y_leave++)
            for(unsigned short x_leave = x - 2; x_leave <= x + 2; x_leave++)
                if((x_leave != x - 2 || y_leave != y) && (x_leave != x + 2 || y_leave != y) && (x_leave != x - 2 || y_leave != y + 4) && (x_leave != x + 2 || y_leave != y + 4))
                    world_map.getBlock(x_leave, y_leave).setType(serverMap::blockType::LEAVES, false);
        
        for(unsigned short y2 = world_map.getWorldHeight() - heights[x] - engine() % 4 - 4; y2 > world_map.getWorldHeight() - heights[x] - height + 5; y2 -= engine() % 4 + 2) {
            world_map.getBlock(x - 1, y2).setType(serverMap::blockType::WOOD, false);
            world_map.getBlock(x - 2, y2).setType(serverMap::blockType::LEAVES, false);
        }
        
        for(unsigned short y2 = world_map.getWorldHeight() - heights[x] - engine() % 4 - 4; y2 > world_map.getWorldHeight() - heights[x] - height + 5; y2 -= engine() % 4 + 2) {
            world_map.getBlock(x + 1, y2).setType(serverMap::blockType::WOOD, false);
            world_map.getBlock(x + 2, y2).setType(serverMap::blockType::LEAVES, false);
        }
        
        if(world_map.getBlock(x - 1, world_map.getWorldHeight() - heights[x] - 1).getType() == serverMap::blockType::GRASS_BLOCK && world_map.getBlock(x - 2, world_map.getWorldHeight() - heights[x] - 2).getType() == serverMap::blockType::AIR)
            world_map.getBlock(x - 1, world_map.getWorldHeight() - heights[x] - 2).setType(serverMap::blockType::WOOD, false);
        
        if(world_map.getBlock(x + 1, world_map.getWorldHeight() - heights[x] - 1).getType() == serverMap::blockType::GRASS_BLOCK && world_map.getBlock(x + 2, world_map.getWorldHeight() - heights[x] - 2).getType() == serverMap::blockType::AIR)
            world_map.getBlock(x + 1, world_map.getWorldHeight() - heights[x] - 2).setType(serverMap::blockType::WOOD, false);
    }
    LOADING_NEXT
}

void terrainGeneratorSwitch(unsigned int x, SimplexNoise& noise, serverMap& world_map) {
    

    /*
    sea level = 300, sea floor = 250
    plains = 325
    hills = 400
    mountains 600-700
    */


    int heat = heatGeneratorInt(x, noise);
    int biomeheight = 1;//heightGeneratorInt(x, noise);

        switch (heat)
        {
        case 0:
            switch (biomeheight)
            {
            case 0://icy seas
                //
                break;
            case 1://snowy tundra
                generateSnowyTundra(x, noise, world_map);

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
                generatePlains(x, noise, world_map);
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
                generateDesert(x, noise, world_map);
                break;
            case 2://savana

                break;
            case 3://savana mountains

                break;
            }
            break;
        }


    return;
}

void generatePlains(int x, SimplexNoise& noise, serverMap& world_map) {
    int sliceHeight = (int)(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int dirtLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < world_map.getWorldHeight(); y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= dirtLayer) {
                if (y == sliceHeight)
                    world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::GRASS_BLOCK, false);
                else
                    world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::DIRT, false);
            }
            else
                world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::STONE_BLOCK, false);
        }//else
            //world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::AIR, false);
    }
}


void generateDesert(int x, SimplexNoise& noise, serverMap& world_map) {
    int sliceHeight = (int)(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int sandLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < world_map.getWorldHeight(); y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= sandLayer) {
                world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::SAND, false);
            }
            else
                world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::STONE_BLOCK, false);
        }//else
            //world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::AIR, false);
    }
}

void generateSnowyTundra(int x, SimplexNoise& noise, serverMap& world_map) {
    int sliceHeight = (int)(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int snowLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < world_map.getWorldHeight(); y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= snowLayer) {
                world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::SNOWY_GRASS_BLOCK, false);
            }
            else
                world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::STONE_BLOCK, false);
        }//else
            //world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::AIR, false);
    }
}
