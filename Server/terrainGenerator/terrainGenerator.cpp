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

void terrainGeneratorSwitch(unsigned int x, SimplexNoise& noise, serverMap& world_map);
void generatePlains(int x, SimplexNoise& noise, serverMap& world_map);
void generateDesert(int x, SimplexNoise& noise, serverMap& world_map);
void generateSnowyTundra(int x, SimplexNoise& noise, serverMap& world_map);

#define LOADING_NEXT terrainGenerator::generating_current++;

int terrainGenerator::generateTerrainDaemon(unsigned int seed, serverMap* world_map) {
    std::mt19937 engine(seed);
    SimplexNoise noise(engine());

    generating_current = 0;
    
    for (int x = 0; x < world_map->getWorldWidth(); x++) {
        terrainGeneratorSwitch(x, noise, *world_map);
    }
    for(unsigned short x = 0; x < world_map->getWorldWidth(); x++)
        for(unsigned short y = 0; y < world_map->getWorldHeight(); y++)
            world_map->getBlock(x, y).update();
    LOADING_NEXT
    return 0;
}

double turbulence(double x, double y, double size, SimplexNoise& noise) {
    double value = 0, initialSize = size;
    
    while(size >= 2) {
        value += noise.noise(x / size, y / size) * size;
        size /= 2.0;
    }
    
    return value / initialSize;
}

int heatGeneratorInt(unsigned int x, SimplexNoise& noise) {
    int heat = (noise.noise((float)x / 1100.0 + 0.125) + 1.0) * 1.5;
    return heat == 3 ? 2 : heat;
}

int heightGeneratorInt(unsigned int x, SimplexNoise& noise) {
    int heat = (noise.noise((float)x / 300.0 + 0.001) + 1.0) * 2.0;
    return heat == 4 ? 3 : heat;
}

void terrainGeneratorSwitch(unsigned int x, SimplexNoise& noise, serverMap& world_map) {
    /*
    sea level = 300, sea floor = 250
    plains = 325
    hills = 400
    mountains 600-700
    */

    int heat = heatGeneratorInt(x, noise), biomeheight = 1; //heightGeneratorInt(x, noise);
    
    switch (heat) {
    case 0:
        switch (biomeheight) {
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
        switch (biomeheight) {
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
        switch (biomeheight) {
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
        }
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
        }
    }
}

void generateSnowyTundra(int x, SimplexNoise& noise, serverMap& world_map) {
    int sliceHeight = (int)(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int snowLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < world_map.getWorldHeight(); y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= snowLayer) {
                if (y < snowLayer + 3) {
                    world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::SNOWY_GRASS_BLOCK, false);
                }else
                    world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::SNOW_BLOCK, false);
            }
            else
                world_map.getBlock((unsigned short)x, world_map.getWorldHeight() - (unsigned short)y - 1).setType(serverMap::blockType::STONE_BLOCK, false);
        }
    }
}
