//
//  worldGenerator.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#include "worldGenerator.hpp"
#include "SimplexNoise.h"
#include <random>
#include <fstream>
#include <vector>
#include <string>

#define PI 3.14159265


int worldGenerator::generateWorld(unsigned int seed) {
    std::mt19937 engine(seed);
    SimplexNoise noise(engine());
    generating_current = 0;

    loadAssets();
    if(seed == 1000){//structure generation
        generateStructureWorld();
    }else{//deafult generation
        generateDeafultWorld(noise);
    }
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

int worldGenerator::heatGeneratorInt(unsigned int x, SimplexNoise &noise) {
    int heat = (noise.noise((float)x / 2000.0 + 0.125) + 1.0) * 1.5;
    return heat == 3 ? 2 : heat;
}

int worldGenerator::heightGeneratorInt(unsigned int x, SimplexNoise& noise) {
    if (x < 50 || x > server_blocks->getWidth() - 50)
        return 0;
    else if (x < 100 || x > server_blocks->getWidth() - 100)
        return 1;
    else {
        int heat = (noise.noise((float)x / 600.0 + 0.001) + 1) * 2;
        return std::min(std::max(1, heat), 3);
    }
}

void worldGenerator::biomeGeneratorSwitch(unsigned int x, SimplexNoise& noise) {
    /*
    sea level = 300, sea floor = 250
    plains = 325
    hills = 400
    mountains 600-700
    */

    int heat = heatGeneratorInt(x, noise);
    int biome_height = heightGeneratorInt(x, noise);
    server_blocks->biomes[x] = (Biome)((biome_height << 2) + heat);
}

void worldGenerator::terrainGenerator(int x, SimplexNoise& noise) {
    if (server_blocks->biomes[x] == Biome::ICY_SEAS)
        generateIcySea(x, noise);
    else if (server_blocks->biomes[x] == Biome::SNOWY_TUNDRA)
        generateSnowyTundra(x, noise);
    else if (server_blocks->biomes[x] == Biome::COLD_HILLS)
        generateColdHills(x, noise);
    else if (server_blocks->biomes[x] == Biome::SNOWY_MOUNTAINS)
        return;
    else if (server_blocks->biomes[x] == Biome::SEA)
        generateSea(x, noise);
    else if (server_blocks->biomes[x] == Biome::PLAINS)
        generatePlains(x, noise);
    else if (server_blocks->biomes[x] == Biome::FOREST)
        generateForest(x, noise);
    else if (server_blocks->biomes[x] == Biome::MOUNTAINS)
        return;
    else if (server_blocks->biomes[x] == Biome::WARM_OCEAN)
        generateWarmOcean(x, noise);
    else if (server_blocks->biomes[x] == Biome::DESERT)
        generateDesert(x, noise);
    else if (server_blocks->biomes[x] == Biome::SAVANA)
        generateSavana(x, noise);
    else if (server_blocks->biomes[x] == Biome::SAVANA_MOUNTAINS)
        return;
}

void worldGenerator::generatePlains(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int dirtLayer = (int)slice_height - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            if (y >= dirtLayer) {
                if (y == slice_height)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::GRASS_BLOCK);
                else
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::DIRT);
            }
            else
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
    if (noise.noise(x + 0.5, slice_height + 0.5) >= 0.8 && x > 5)
        structurePositions.emplace_back("Tree", x, slice_height);
}


void worldGenerator::generateDesert(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int sandLayer = (int)slice_height - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            if (y >= sandLayer)
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::SAND);
            else
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}

void worldGenerator::generateSnowyTundra(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int snowLayer = (int)slice_height - (noise.noise(x / 4.0f + 0.2, 0) * 2 + 8);
    int iceLayer = 300 - (noise.noise(x / 2.0f + 0.25, 0) * 2 + 3);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            if (y >= snowLayer) {
                if(y == snowLayer + 2)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::SNOWY_GRASS_BLOCK);
                else if (y < snowLayer + 2)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::DIRT);
                else
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::SNOW_BLOCK);
            }
            else
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            if (y > iceLayer)
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::ICE);
            else {
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
            }
        }
    }
}

void worldGenerator::generateSea(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}

void worldGenerator::generateIcySea(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int iceLayer = 300 - (noise.noise(x / 2.0f + 0.25, 0) * 2 + 3);

    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            if(y > iceLayer)
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::ICE);
            else {
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
            }
        }
    }
}

void worldGenerator::generateWarmOcean(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
}

void worldGenerator::generateForest(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int dirtLayer = (int)slice_height - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            if (y >= dirtLayer) {
                if (y == slice_height)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::GRASS_BLOCK);
                else
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::DIRT);
            }
            else
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
    if ( x%8 == 0 && noise.noise(x + 0.5, slice_height + 0.5) >= -0.8 && x > 5)
        structurePositions.emplace_back("Tree", x - 2/* + noise.noise(x) * 2*/, slice_height);
}

void worldGenerator::generateColdHills(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int snowLayer = (int)slice_height - (noise.noise(x / 4.0f + 0.2, 0) * 2 + 8);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            if (y >= snowLayer) {
                if (y == snowLayer + 2)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::SNOWY_GRASS_BLOCK);
                else if (y < snowLayer + 2)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::DIRT);
                else
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::SNOW_BLOCK);
            }
            else
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
    }
}

void worldGenerator::generateSavana(int x, SimplexNoise& noise) {
    int slice_height = calculateHeight(x, noise);
    int dirtLayer = (int)slice_height - (noise.noise(x / 4.0f + 0.25, 0) * 2 + 8);
    for (int y = 0; y < server_blocks->getHeight(); y++) {
        if (y <= slice_height) {//generates surface
            if (y >= dirtLayer) {
                if (y == slice_height)
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::GRASS_BLOCK);
                else
                    server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::DIRT);
            }
            else
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::STONE_BLOCK);
        }
        else if (y < 300) {
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setLiquidLevel(127);
        }
    }
    //if (noise.noise(x + 0.5, slice_height + 0.5) >= 0.7)
        //generateAccaciaTree(x, slice_height);
}





int worldGenerator::calculateHeight(int x, SimplexNoise& noise) {
    Biome biome1 = server_blocks->biomes[x], biome2 = Biome::NO_BIOME;
    int biomeDistance = 0;
    int slice_height = 0, slice1 = 0, slice2 = 0;

    for (int i = std::max(0, x - 1); i > std::max(0, x - 10); i--) {
        if (server_blocks->biomes[i] != biome1) {
            biome2 = server_blocks->biomes[i];
            biomeDistance = std::abs(x - i);
            break;
        }
    }
    if (biome2 == Biome::NO_BIOME) {
        for (int i = std::min(server_blocks->getWidth() - 1, x + 1); i < std::min(server_blocks->getWidth() - 1, x + 10); i++) {
            if (server_blocks->biomes[i] != biome1) {
                biome2 = server_blocks->biomes[i];
                biomeDistance = std::abs(x - i);
                break;
            }
        }
    }

    if(biome2 == Biome::NO_BIOME) {//deafult generation
        if (biome1 == Biome::ICY_SEAS || biome1 == Biome::SEA || biome1 == Biome::WARM_OCEAN) {
            slice_height = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 60 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 6 + 260);
        }
        else if (biome1 == Biome::SNOWY_TUNDRA || biome1 == Biome::PLAINS || biome1 == Biome::DESERT) {
            slice_height = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 325);
        }
        else if (biome1 == Biome::COLD_HILLS || biome1 == Biome::FOREST) {
            slice_height = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 70 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 380);
        }
        else if (biome1 == Biome::SAVANA) {
            slice_height = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 40 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 360);
        }
    }
    else {
        if (biome1 == Biome::ICY_SEAS || biome1 == Biome::SEA || biome1 == Biome::WARM_OCEAN) {
            slice1 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 60 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 6 + 260);
        }
        else if (biome1 == Biome::SNOWY_TUNDRA || biome1 == Biome::PLAINS || biome1 == Biome::DESERT) {
            slice1 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 325);
        }
        else if (biome1 == Biome::COLD_HILLS || biome1 == Biome::FOREST) {
            slice1 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 70 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 380);
        }
        else if (biome1 == Biome::SAVANA) {
            slice1 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 40 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 360);
        }


        if (biome2 == Biome::ICY_SEAS || biome2 == Biome::SEA || biome2 == Biome::WARM_OCEAN) {
            slice2 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 60 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 6 + 260);
        }
        else if (biome2 == Biome::SNOWY_TUNDRA || biome2 == Biome::PLAINS || biome2 == Biome::DESERT) {
            slice2 = int(turbulence(x / 20.0f + 0.3, 0, 64, noise) * 20 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 325);
        }
        else if (biome2 == Biome::COLD_HILLS || biome2 == Biome::FOREST) {
            slice2 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 70 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 380);
        }
        else if (biome2 == Biome::SAVANA) {
            slice2 = int(turbulence(x / 30.0f + 0.3, 0, 64, noise) * 40 + turbulence(x / 4.0f + 0.1, 0, 8, noise) * 2 + 360);
        }

        slice_height = (-cos((biomeDistance + 10) * PI / 20) / 2 + 0.5) * slice1 + (cos((biomeDistance + 10) * PI / 20) / 2 + 0.5) * slice2;
    }
    return slice_height;
}

#include "print.hpp"

void worldGenerator::loadAssets() {
    std::ifstream structureFile;
    structureFile.open(resource_path + "/Structures.asset", std::ios::in);

    structureFile.seekg(0, std::ios::end);
    int size = (int)structureFile.tellg();
    char* assetData = new char[size];
    structureFile.seekg(0, std::ios::beg);
    structureFile.read(assetData, size);

    int counter = 0;
    int previousEnd = 0;
    while (counter < size - 1) {
        std::string name;
        int nameSize = assetData[counter];
        counter++;
        while (counter - previousEnd <= nameSize) {
            name += assetData[counter];
            counter++;
        }
        int x_size = assetData[counter];
        counter++;
        int y_size = assetData[counter];
        counter++;
        auto *blocks = new BlockType[x_size * y_size];
        for (int i = 0; i < x_size * y_size; i++) {
            blocks[i] = (BlockType)assetData[counter];
            counter++;
        }
        structures.emplace_back(name, x_size, y_size, blocks);
        previousEnd = counter;
    }

    structureFile.close();
    delete[] assetData;
}

void worldGenerator::generateStructure(const std::string& name, int x, int y) {
    for (auto & structure : structures) {
        if (name == structure.name) {
            for(int j = 0; j < structure.y_size * structure.x_size; j++)
                if(structure.blocks[j] != BlockType::NOTHING)
                    server_blocks->getBlock((unsigned short)(x + j % structure.x_size), (unsigned short)(server_blocks->getHeight() - y + (j - j % structure.x_size) / structure.x_size) - structure.y_size - 1).setTypeWithoutProcessing(structure.blocks[j]);
            break;
        }
    }

}

void worldGenerator::generateStructureWorld(){
    generateFlatTerrain();
    generateStructuresForStrWorld();
    updateBlocks();
}

void worldGenerator::generateFlatTerrain() {
    for (int x = 0; x < server_blocks->getWidth(); x++) {
        server_blocks->biomes[x] = Biome::PLAINS;
    }
    for (int x = 0; x < server_blocks->getWidth(); x++) {
        for (int y = 0; y < server_blocks->getHeight(); y++) {
            if (y <= 324) {//generates surface
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::DIRT);
            }else if(y == 325)
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - (unsigned short)y - 1).setTypeWithoutProcessing(BlockType::GRASS_BLOCK);
        }
    }
}

void worldGenerator::generateStructuresForStrWorld() {
    int x = 0;
    while(x < server_blocks->getWidth()){
        for (auto & structure : structures) {
            for(int j = 0; j < structure.y_size * structure.x_size; j++)
                if(structure.blocks[j] != BlockType::NOTHING)
                    server_blocks->getBlock((unsigned short)(x + j % structure.x_size), (unsigned short)(server_blocks->getHeight() - 326 + (j - j % structure.x_size) / structure.x_size) - structure.y_size).setTypeWithoutProcessing(structure.blocks[j]);
            x += structure.x_size + 1;
        }
    }
}

void worldGenerator::updateBlocks() {
    for(unsigned short x = 0; x < server_blocks->getWidth(); x++)
        for(unsigned short y = 0; y < server_blocks->getHeight(); y++)
            server_blocks->getBlock(x, y).update();
}

void worldGenerator::generateDeafultWorld(SimplexNoise& noise) {
    for (int x = 0; x < server_blocks->getWidth(); x++) {
        biomeGeneratorSwitch(x, noise);
    }
    for (int x = 0; x < server_blocks->getWidth(); x++) {
        terrainGenerator(x, noise);
    }
    for (const structurePosition& i : structurePositions) {
        generateStructure(i.name, i.x, i.y);
    }
    updateBlocks();
}
