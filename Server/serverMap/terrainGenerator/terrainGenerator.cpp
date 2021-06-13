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
#define PI 3.14159265

// CAVES

#define CAVE_START 0.15
#define CAVE_LENGTH 0.3

// STONE

#define STONE_START 0.1
#define STONE_LENGTH 1

int serverMap::generateTerrain(unsigned int seed) {
    std::mt19937 engine(seed);
    SimplexNoise noise(engine());

    generating_current = 0;
    
    for (int x = 0; x < width; x++) {
        biomeGeneratorSwitch(x, noise);
    }
    for (int x = 0; x < width; x++) {
        terrainGenerator(x, noise);
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
    int heat = (noise.noise((float)x / 2000.0 + 0.125) + 1.0) * 1.5;
    return heat == 3 ? 2 : heat;
}

int serverMap::heightGeneratorInt(unsigned int x, SimplexNoise& noise) {
    /*if (x < 50 || x > width - 50)
        return 0;
    else if (x < 100 || x > width - 100)
        return 1;
    else {
        int heat = (noise.noise((float)x / 600.0 + 0.001) + 1) * 2;
        return std::min(std::max(1, heat), 3);
    }*/
    int heat = (noise.noise((float)x / 2000.0 + 0.125) + 1.0) * 2;
    return heat == 4 ? 3 : heat;
}

void serverMap::biomeGeneratorSwitch(unsigned int x, SimplexNoise& noise) {
    /*
    sea level = 300, sea floor = 250
    plains = 325
    hills = 400
    mountains 600-700
    */

    int heat = heatGeneratorInt(x, noise);
    int biomeheight = 2;// heightGeneratorInt(x, noise);
    /*if (biomeheight > 1)
        biomeheight = 3 - biomeheight;*/
    
    switch (heat) {
    case 0:
        switch (biomeheight) {
        case 0://icy seas
            biomes[x] = biome::ICY_SEAS;
            break;
        case 1://snowy tundra
            biomes[x] = biome::SNOWY_TUNDRA;
            break;
        case 2://cold hills (with taiga trees?)
            biomes[x] = biome::COLD_HILLS;
            break;
        case 3://snowy mountains
            biomes[x] = biome::SNOWY_MOUNTAINS;
            break;
        }
        break;

    case 1:
        switch (biomeheight) {
        case 0://regular sea
            biomes[x] = biome::SEA;
            break;
        case 1://plains
            biomes[x] = biome::PLAINS;
            break;
        case 2://forest
            biomes[x] = biome::FOREST;
            break;
        case 3://regular mountain
            biomes[x] = biome::MOUNTAINS;
            break;
        }
        break;

    case 2:
        switch (biomeheight) {
        case 0://warm ocean
            biomes[x] = biome::WARM_OCEAN;
            break;
        case 1://desert
            biomes[x] = biome::DESERT;
            break;
        case 2://savana
            biomes[x] = biome::SAVANA;
            break;
        case 3://savana mountains
            biomes[x] = biome::SAVANA_MOUNTAINS;
            break;
        }
        break;
    }
}

void serverMap::terrainGenerator(int x, SimplexNoise& noise) {
    if (biomes[x] == biome::ICY_SEAS)
        generateIcySea(x, noise);
    else if (biomes[x] == biome::SNOWY_TUNDRA)
        generateSnowyTundra(x, noise);
    else if (biomes[x] == biome::COLD_HILLS)
        generateColdHills(x, noise);
    else if (biomes[x] == biome::SNOWY_MOUNTAINS)
        return;
    else if (biomes[x] == biome::SEA)
        generateSea(x, noise);
    else if (biomes[x] == biome::PLAINS)
        generatePlains(x, noise);
    else if (biomes[x] == biome::FOREST)
        generateForest(x, noise);
    else if (biomes[x] == biome::MOUNTAINS)
        return;
    else if (biomes[x] == biome::WARM_OCEAN)
        generateWarmOcean(x, noise);
    else if (biomes[x] == biome::DESERT)
        generateDesert(x, noise);
    else if (biomes[x] == biome::SAVANA)
        generateSavana(x, noise);
    else if (biomes[x] == biome::SAVANA_MOUNTAINS)
        return;
}

void serverMap::generatePlains(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
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
        else if (y < 300) {
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
    if (noise.noise(x + 0.5, sliceHeight + 0.5) >= 0.8)
        generateTree(x, sliceHeight);
}


void serverMap::generateDesert(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
    int sandLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= sandLayer)
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::SAND, false);
            else
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
        else if (y < 300) {
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}

void serverMap::generateSnowyTundra(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
    int snowLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.2, 0) * 2 + 8);
    int iceLayer = 300 - (noise.noise(x / 2.0f + 0.25, 0) * 2 + 3);
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
        else if (y < 300)
            if (y > iceLayer)
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::ICE, false);
            else {
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
            }
    }
}

void serverMap::generateSea(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
        else if (y < 300) {
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}

void serverMap::generateIcySea(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
    int iceLayer = 300 - (noise.noise(x / 2.0f + 0.25, 0) * 2 + 3);

    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
        else if (y < 300)
            if(y > iceLayer)
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::ICE, false);
            else {
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
                getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
            }
    }
}

void serverMap::generateWarmOcean(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(blockType::STONE_BLOCK, false);
        }
        else if (y < 300) {
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}

void serverMap::generateForest(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
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
        else if (y < 300) {
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
    if (noise.noise(x + 0.5, sliceHeight + 0.5) >= 0.4)
        generateTree(x, sliceHeight);
}

void serverMap::generateColdHills(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
    int snowLayer = (int)sliceHeight - (noise.noise(x / 4.0f + 0.2, 0) * 2 + 8);
    for (int y = 0; y < height; y++) {
        if (y <= sliceHeight) {//generates surface
            if (y >= snowLayer) {
                if (y == snowLayer + 2)
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

void serverMap::generateSavana(int x, SimplexNoise& noise) {
    int sliceHeight = calculateHeight(x, noise);
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
        else if (y < 300) {
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setType(liquidType::WATER, false);
            getBlock((unsigned short)x, height - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}





void serverMap::generateTree(int x, int y) {
    for (int i = x - 2; i < x + 3; i++) {
        for (int j = y + 5; j < y + 10; j++) {
            getBlock((unsigned short)i, height - (unsigned short)j).setType(blockType::LEAVES, false);
        }
    }
    getBlock((unsigned short)x, height - (unsigned short)y - 2).setType(blockType::WOOD, false);
    getBlock((unsigned short)x, height - (unsigned short)y - 3).setType(blockType::WOOD, false);
    getBlock((unsigned short)x, height - (unsigned short)y - 4).setType(blockType::WOOD, false);
    getBlock((unsigned short)x, height - (unsigned short)y - 5).setType(blockType::WOOD, false);
    getBlock((unsigned short)x, height - (unsigned short)y - 6 ).setType(blockType::WOOD, false);
}






int serverMap::calculateHeight(int x, SimplexNoise& noise) {
    biome biome1 = biomes[x], biome2 = biome::NO_BIOME;
    int biomeDistance;
    int sliceHeight, slice1, slice2;

    for (int i = std::max(0, x - 1); i > std::max(0, x - 10); i--) {
        if (biomes[i] != biome1) {
            biome2 = biomes[i];
            biomeDistance = std::abs(x - i);
            break;
        }
    }
    if (biome2 == biome::NO_BIOME) {
        for (int i = std::min(width - 1, x + 1); i < std::min(width - 1, x + 10); i++) {
            if (biomes[i] != biome1) {
                biome2 = biomes[i];
                biomeDistance = std::abs(x - i);
                break;
            }
        }
    }

    if(biome2 == biome::NO_BIOME) {//deafult generation
        if (biome1 == biome::ICY_SEAS || biome1 == biome::SEA || biome1 == biome::WARM_OCEAN) {
            sliceHeight = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 60 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 6 + 260);
        }
        else if (biome1 == biome::SNOWY_TUNDRA || biome1 == biome::PLAINS || biome1 == biome::DESERT) {
            sliceHeight = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 325);
        }
        else if (biome1 == biome::COLD_HILLS || biome1 == biome::FOREST) {
            sliceHeight = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 70 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 380);
        }
        else if (biome1 == biome::SAVANA) {
            sliceHeight = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 40 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 360);
        }
    }
    else {
        if (biome1 == biome::ICY_SEAS || biome1 == biome::SEA || biome1 == biome::WARM_OCEAN) {
            slice1 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 60 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 6 + 260);
        }
        else if (biome1 == biome::SNOWY_TUNDRA || biome1 == biome::PLAINS || biome1 == biome::DESERT) {
            slice1 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 325);
        }
        else if (biome1 == biome::COLD_HILLS || biome1 == biome::FOREST) {
            slice1 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 70 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 380);
        }
        else if (biome1 == biome::SAVANA) {
            slice1 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 40 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 360);
        }


        if (biome2 == biome::ICY_SEAS || biome2 == biome::SEA || biome2 == biome::WARM_OCEAN) {
            slice2 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 60 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 6 + 260);
        }
        else if (biome2 == biome::SNOWY_TUNDRA || biome2 == biome::PLAINS || biome2 == biome::DESERT) {
            slice2 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 325);
        }
        else if (biome2 == biome::COLD_HILLS || biome2 == biome::FOREST) {
            slice2 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 70 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 380);
        }
        else if (biome2 == biome::SAVANA) {
            slice2= int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 40 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 360);
        }

        sliceHeight = (-cos((biomeDistance + 10) * PI / 20) / 2 + 0.5) * slice1 + (cos((biomeDistance + 10) * PI / 20) / 2 + 0.5) * slice2;
    }
    return(sliceHeight);
}
