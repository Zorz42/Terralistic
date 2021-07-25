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
#include "biomes.hpp"


#define PI 3.14159265

#include <iostream>

int worldGenerator::generateWorld(unsigned short world_width, unsigned short world_height, unsigned int seed) {
    //std::mt19937 engine(seed);
    SimplexNoise noise(seed);
    generating_current = 0;
    server_blocks->createWorld(world_width, world_height);

    loadAssets();
    if(seed == 1000){//structure generation
        generateStructureWorld();
    }else{//deafult generation
        loadBiomes();
        generateDeafultWorld(noise);
    }
    generating_current++;
    return 0;
}

double turbulence(double x, double y, double size, SimplexNoise& noise) {
    double value = 0, initialSize = size;

    while(size >= 8) {
        value += noise.noise(x / size, y / size) * size;
        size /= 2.0;
    }

    return value / initialSize;
}

int worldGenerator::heatGeneratorInt(unsigned int x, SimplexNoise &noise) {
    int biome_heat = (noise.noise((float)x / 2000.0 + 0.125) + 1.0) * 1.5;
    return biome_heat == 3 ? 2 : biome_heat;
}

int worldGenerator::heightGeneratorInt(unsigned int x, SimplexNoise& noise) {
    if (x < 50 || x > server_blocks->getWidth() - 50)
        return 0;
    else if (x < 100 || x > server_blocks->getWidth() - 100)
        return 1;
    else {
        int biome_heat = (noise.noise((float)x / 600.0 + 0.001) + 1) * 2;
        return std::min(std::max(1, biome_heat), 3);
    }
}

void worldGenerator::biomeGeneratorSwitch(unsigned int x, SimplexNoise& noise) {
    /*
    sea level = 300, sea floor = 250
    plains = 325
    hills = 400
    mountains 600-700
    */

    int biome_heat = heatGeneratorInt(x, noise);
    int biome_height = heightGeneratorInt(x, noise);
    server_blocks->biomes[x] = (Biome)((biome_heat * 4) + biome_height);
    //server_blocks->biomes[x] = Biome::ICY_SEAS;
}

void worldGenerator::terrainGenerator(int x, SimplexNoise& noise) {
    int surface_height = calculateHeight(x, noise);
    int last_layer = surface_height + 1;
    int generating_layer = 0;
    biome &slice_biome = loaded_biomes[(int)server_blocks->biomes[x]];
    for(int y = std::max(server_blocks->getHeight() / 3 * 2, surface_height); y > 0; y--) {
        if (y > surface_height) {
            server_blocks->getBlock(x, server_blocks->getHeight() - y).setTypeWithoutProcessing(LiquidType::WATER);
            server_blocks->getBlock(x, server_blocks->getHeight() - y).setLiquidLevelWithoutProcessing(127);
        }else{
            if (slice_biome.ground_layers[generating_layer].layer_height_mode == LayerHeightMode::PREVIOUS_LAYER) {
                if (y > last_layer - slice_biome.ground_layers[generating_layer].height +
                        noise.noise(x / 3 + 0.1, y * 2 + 0.5) *
                        slice_biome.ground_layers[generating_layer].height_variation)
                    server_blocks->getBlock(x, server_blocks->getHeight() - y).setTypeWithoutProcessing(slice_biome.ground_layers[generating_layer].block);
                else {
                    last_layer = y + 1;
                    generating_layer++;
                    y++;
                }
            } else {
                if (y < slice_biome.ground_layers[generating_layer + 1].height + noise.noise(x / 3 + 0.1, y * 2 + 0.5) * slice_biome.ground_layers[generating_layer + 1].height_variation && slice_biome.ground_layers.size() != generating_layer + 1) {
                    generating_layer++;
                    y++;
                } else if (y < slice_biome.ground_layers[generating_layer].height +
                               noise.noise(x / 3 + 0.1, y * 2 + 0.5) *
                               slice_biome.ground_layers[generating_layer].height_variation) {
                    server_blocks->getBlock(x, server_blocks->getHeight() - y).setTypeWithoutProcessing(slice_biome.ground_layers[generating_layer].block);
                }
                else{
                    server_blocks->getBlock(x, server_blocks->getHeight() - y).setTypeWithoutProcessing(LiquidType::WATER);
                    server_blocks->getBlock(x, server_blocks->getHeight() - y).setLiquidLevelWithoutProcessing(127);
                }
            }
        }
    }
}

int worldGenerator::calculateHeight(int x, SimplexNoise& noise) {
    int biome_blend = 20;
    int slice_height = 0;
    int slice_height_variation = 0;
    float divide_at_end = 0;
        for(int i = std::max(0, x - biome_blend); i < std::min(server_blocks->getWidth() - 1, x + biome_blend); i++){
            slice_height += loaded_biomes[(int)server_blocks->biomes[i]].surface_height * (1 - (float)std::abs(x - i) / biome_blend);
            slice_height_variation += loaded_biomes[(int)server_blocks->biomes[i]].surface_height_variation * (1 - (float)std::abs(x - i) / biome_blend);
            divide_at_end += (1 - (float)std::abs(x - i) / biome_blend);
        }

    return (slice_height + turbulence(x + 0.003, 0, 64, noise) * slice_height_variation) / divide_at_end;
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
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - y - 1).setTypeWithoutProcessing(BlockType::DIRT);
            }else if(y == 325)
                server_blocks->getBlock((unsigned short)x, server_blocks->getHeight() - y - 1).setTypeWithoutProcessing(BlockType::GRASS_BLOCK);
        }
    }
}

void worldGenerator::generateStructuresForStrWorld() {
    int x = 0;
    while(x < server_blocks->getWidth()){
        for (auto & structure : structures) {
            if(structure.y_size + x >= server_blocks->getWidth())
                return;
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

void worldGenerator::loadBiomes() {
    loaded_biomes.push_back(biome(Biome::ICY_SEAS, server_blocks->getHeight() / 3 * 2, 0,
                                  {layer(BlockType::ICE, LayerHeightMode::PREVIOUS_LAYER, 3, 1),
                                  layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight() / 3 * 2 - 50, 10)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::SNOWY_TUNDRA, server_blocks -> getHeight() / 3 * 2 + 20, 4,
                                  {layer(BlockType::SNOW_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   layer(BlockType::SNOWY_GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                   {}));
    loaded_biomes.push_back(biome(Biome::COLD_HILLS, server_blocks -> getHeight() / 3 * 2 + 29, 15,
                                  {layer(BlockType::SNOW_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   layer(BlockType::SNOWY_GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 1, 0),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 4, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                   {}));
    loaded_biomes.push_back(biome(Biome::SNOWY_MOUNTAINS, server_blocks -> getHeight() / 3 * 2 + 70, 37,
                                  {layer(BlockType::SNOW_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 2, 1),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::SEA, server_blocks->getHeight() / 3 * 2 - 50, 10,
                                  {layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::PLAINS, server_blocks -> getHeight() / 6 * 4 + 22, 4,
                                  {layer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::FOREST, server_blocks -> getHeight() / 3 * 2 + 26, 10,
                                  {layer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::MOUNTAINS, server_blocks -> getHeight() / 3 * 2 + 70, 33,
                                  {layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::WARM_OCEAN, server_blocks->getHeight() / 3 * 2 - 50, 10,
                                  {layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::DESERT, server_blocks -> getHeight() / 6 * 4 + 22, 4,
                                  {layer(BlockType::SAND, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::SAVANA, server_blocks -> getHeight() / 3 * 2 + 26, 10,
                                  {layer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(biome(Biome::SAVANA_MOUNTAINS, server_blocks -> getHeight() / 3 * 2 + 50, 25,
                                  {layer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   layer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   layer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, server_blocks->getHeight(), 0)},
                                  {}));
}
