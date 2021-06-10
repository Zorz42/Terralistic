//
//  terrainGenerator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "SimplexNoise.h"
#include <random>
#include "serverMap.hpp"

// terrain generation parameters

// TERRAIN
#define TERRAIN_VERTICAL_MULTIPLIER 140
#define TERRAIN_HORIZONTAL_DIVIDER 4
#define TERRAIN_HORIZONT ((float)world_map.height / 2 - 50)
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

int serverMap::generateTerrain(unsigned int seed) {
    std::mt19937 engine(seed);
    SimplexNoise noise(engine());

    generating_current = 0;
    
    for (int x = 0; x < width; x++) {
        terrainGeneratorSwitch(x, noise);
    }
    for(unsigned short x = 0; x < width; x++)
        for(unsigned short y = 0; y < height; y++)
            getBlock(x, y).update();
    generating_current++;
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

void serverMap::terrainGeneratorSwitch(unsigned int x, SimplexNoise& noise) {
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
            generateSnowyTundra(x, noise);

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
            generatePlains(x, noise);
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
            generateDesert(x, noise);
            break;
        case 2://savana

            break;
        case 3://savana mountains

            break;
        }
        break;
    }
}

void serverMap::generatePlains(int x, SimplexNoise& noise) {
    int sliceHeight = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int dirtLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= dirtLayer) {
                if (y == sliceHeight)
                    getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::GRASS_BLOCK, false);
                else
                    getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::DIRT, false);
            }
            else
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
    }
}


void serverMap::generateDesert(int x, SimplexNoise& noise) {
    int sliceHeight = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int sandLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= sandLayer)
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::SAND, false);
            else
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
    }
}

void serverMap::generateSnowyTundra(int x, SimplexNoise& noise) {
    int sliceHeight = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 320);
    int snowLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= snowLayer) {
                if(y == snowLayer + 2)
                    getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::SNOWY_GRASS_BLOCK, false);
                else if (y < snowLayer + 2)
                    getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::DIRT, false);
                else
                    getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::SNOW_BLOCK, false);
            }
            else
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
    }
}
