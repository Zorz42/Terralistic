#include "worldGenerator.hpp"
#include "SimplexNoise.h"
#include <fstream>
#include <vector>
#include <string>
#include <math.h>
#include "biomes.hpp"

int worldGenerator::generateWorld(unsigned short world_width, unsigned short world_height, unsigned int seed) {
    SimplexNoise noise(seed);
    server_blocks->createWorld(world_width, world_height);
    
    loadAssets();
    if(seed == 1000) {
        generateStructureWorld();
    } else {
        generating_total = server_blocks->getWidth() * 2;
        loadBiomes();
        generateDeafultWorld(noise);
    }
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
    if (x < 100 || x > server_blocks->getWidth() - 100)
        return 0;
    else if (x < 150 || x > server_blocks->getWidth() - 150)
        return 1;
    else {
        int biome_heat = (noise.noise((float)x / 600.0 + 0.001) + 1) * 1.5 + 1;
        return std::min(biome_heat, 3);
    }
}

void worldGenerator::generateBiomes(unsigned int x, SimplexNoise& noise) {
    int biome_heat = heatGeneratorInt(x, noise);
    int biome_height = heightGeneratorInt(x, noise);
    server_blocks->biomes[x] = (Biome)((biome_heat * 4) + biome_height);
}

void worldGenerator::terrainGenerator(int x, SimplexNoise& noise) {
    generateSurface(x, noise);
    for(auto &checking_structure : loaded_biomes[(int)server_blocks->biomes[x]].structure_chances){
        if((noise.noise((float)x + 0.5, (float)server_blocks->surface_height[x] + 0.5) + 1) * checking_structure.chance_on_each_block <= 2 && x > checking_structure.x_of_last_instance + checking_structure.least_distance_between_instances) {
            structurePositions.emplace_back(structurePosition(checking_structure.structure_name +
                                         std::to_string((int)((noise.noise((float)x - 0.5, (float)server_blocks->surface_height[x] - 0.5) + 1) / 2 * checking_structure.unique_structures_of_type)),
                                         x, server_blocks->surface_height[x] - 1));
            checking_structure.x_of_last_instance = x;
        }
    }
}

void worldGenerator::calculateHeight(SimplexNoise& noise) {
    int biome_blend = 20;
    float divide_at_end;
    unsigned short no_blend_height[server_blocks->getWidth()];
    for(int current_slice = 0; current_slice < server_blocks->getWidth(); current_slice++) {
        no_blend_height[current_slice] = loaded_biomes[(int) server_blocks->biomes[current_slice]].surface_height;
    }

    for(int current_slice = 0; current_slice < server_blocks->getWidth(); current_slice++) {
        divide_at_end = 0;
        server_blocks->surface_height[current_slice] = 0;
        unsigned short variation = 0;
        for (int i = std::max(0, current_slice - biome_blend); i < std::min(server_blocks->getWidth() - 1, current_slice + biome_blend); i++) {
            server_blocks->surface_height[current_slice] += no_blend_height[i] * (1 - (float)std::abs(current_slice - i) / biome_blend);
            variation += loaded_biomes[(int) server_blocks->biomes[i]].surface_height_variation * (1 - (float)std::abs(current_slice - i) / biome_blend);
            divide_at_end += (1 - (float)std::abs(current_slice - i) / biome_blend);
        }
        server_blocks->surface_height[current_slice] /= divide_at_end;
        variation /= divide_at_end;
        server_blocks->surface_height[current_slice] += turbulence(current_slice + 0.003, 0, 64, noise) * variation;
    }
}

void worldGenerator::generateSurface(unsigned int x, SimplexNoise &noise) {
    int generate_from = std::max(server_blocks->getHeight() / 3 * 2, (int)server_blocks->surface_height[x]);
    for(unsigned int y = generate_from; y > 0; y--){
        unsigned int changed_x = std::max(std::min((int)(x + noise.noise(x + 0.5, y + 0.5) * 8), (int)server_blocks->getWidth()), 0);
        switch ((int)server_blocks->biomes[changed_x]) {
            case 0:
                generateBlockIcyOcean(x, y, noise);
                break;
            case 1:
                generateBlockSnowyPlains(x, y, noise);
                break;
            case 2:
                generateBlockSnowyHills(x, y, noise);
                break;
            case 3:
                generateBlockSnowyMountains(x, y, noise);
                break;
            case 4:
                generateBlockOcean(x, y, noise);
                break;
            case 5:
                generateBlockPlains(x, y, noise);
                break;
            case 6:
                generateBlockHills(x, y, noise);
                break;
            case 7:
                generateBlockMountains(x, y, noise);
                break;
            case 8:
                generateBlockWarmOcean(x, y, noise);
                break;
            case 9:
                generateBlockDesert(x, y, noise);
                break;
            case 10:
                generateBlockSavanaHills(x, y, noise);
                break;
            case 11:
                generateBlockSavanaMountains(x, y, noise);
                break;
        }
    }
}

void worldGenerator::generateBlockIcyOcean(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x])
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
    else if(y > server_blocks->getHeight() / 3 * 2 - noise.noise((float)x / 4 + 0.125) - 2)
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::ICE);
    else{
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockSnowyPlains(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 20)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 0.5) - 5)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::DIRT);
        else if(y == server_blocks->surface_height[x] + ceil(noise.noise((float)x / 3 + 0.15) * 0.5) - 5)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::SNOWY_GRASS_BLOCK);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::SNOW_BLOCK);
    }else
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::ICE);
}

void worldGenerator::generateBlockSnowyHills(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 20)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 0.5) - 5)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::DIRT);
        else if(y == server_blocks->surface_height[x] + ceil(noise.noise((float)x / 3 + 0.15) * 0.5) - 5)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::SNOWY_GRASS_BLOCK);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::SNOW_BLOCK);
    }else
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::ICE);
}

void worldGenerator::generateBlockSnowyMountains(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + noise.noise((float)x / 3 + 0.15) * 0.5 - 5)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::SNOW_BLOCK);
    }else
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::ICE);
}

void worldGenerator::generateBlockOcean(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x])
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
    else{
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockPlains(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 15)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else if(y < server_blocks->surface_height[x])
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::DIRT);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::GRASS_BLOCK);
    }else
    if(y < server_blocks->getHeight() / 3 * 2){
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockHills(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 15)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else if(y < server_blocks->surface_height[x])
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::DIRT);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::GRASS_BLOCK);
    }else
    if(y < server_blocks->getHeight() / 3 * 2){
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockMountains(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x])
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
    else {
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockWarmOcean(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x])
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
    else{
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockDesert(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 15)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::SAND);
    }else{
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockSavanaHills(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 15)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else if(y < server_blocks->surface_height[x])
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::DIRT);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::GRASS_BLOCK);
    }else
    if(y < server_blocks->getHeight() / 3 * 2){
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateBlockSavanaMountains(unsigned int x, unsigned int y, SimplexNoise &noise) {
    if(y <= server_blocks->surface_height[x]){
        if(y < server_blocks->surface_height[x] + (noise.noise((float)x / 3 + 0.15) * 1.8) - 25)
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::STONE_BLOCK);
        else if(y < server_blocks->surface_height[x])
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::DIRT);
        else
            server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::GRASS_BLOCK);
    }else
    if(y < server_blocks->getHeight() / 3 * 2){
        server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, LiquidType::WATER);
        server_blocks->setLiquidLevelDirectly(x, server_blocks->getHeight() - y, 255);
    }
}

void worldGenerator::generateCaves(SimplexNoise &noise) {
    for(unsigned int x = 0; x < server_blocks->getWidth(); x++) {
        for (unsigned int y = server_blocks->surface_height[x]; y > 0; --y) {
            float value = turbulence((double)x / 2, (double)y, 64, noise) * std::min(((float)server_blocks->getHeight() / 3 * 2 - y) / 200, (float)1);
            if (value > 0.3) {
                server_blocks->setTypeDirectly(x, server_blocks->getHeight() - y, BlockType::AIR);
                if (y == server_blocks->surface_height[x])
                    server_blocks->surface_height--;
            }
        }
        generating_current++;
    }
}

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
        int nameSize = (unsigned char)assetData[counter];
        counter++;
        while (counter - previousEnd <= nameSize) {
            name += assetData[counter];
            counter++;
        }
        int x_size = (unsigned char)assetData[counter];
        counter++;
        int y_size = (unsigned char)assetData[counter];
        counter++;
        int y_offset = (unsigned char)assetData[counter];
        counter++;
        auto *blocks = new BlockType[x_size * y_size];
        for (int i = 0; i < x_size * y_size; i++) {
            blocks[i] = (BlockType)assetData[counter];
            counter++;
        }
        structures.emplace_back(name, x_size, y_size, y_offset, blocks);
        previousEnd = counter;
    }

    structureFile.close();
    delete[] assetData;
}

void worldGenerator::generateStructure(const std::string& name, int x, int y) {
    for (auto & structure : structures) {
        if (name == structure.name) {
            x -= structure.x_size / 2;
            y += structure.y_offset;
            for(int j = 0; j < structure.y_size * structure.x_size; j++)
                if(structure.blocks[j] != BlockType::NOTHING)
                    server_blocks->setTypeDirectly((unsigned short)(x + j % structure.x_size), (unsigned short)(server_blocks->getHeight() - y + (j - j % structure.x_size) / structure.x_size) - structure.y_size - 1, structure.blocks[j]);
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
                server_blocks->setTypeDirectly((unsigned short)x, server_blocks->getHeight() - y - 1, BlockType::DIRT);
            }else if(y == 325)
                server_blocks->setTypeDirectly((unsigned short)x, server_blocks->getHeight() - y - 1, BlockType::GRASS_BLOCK);
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
                    server_blocks->setTypeDirectly((unsigned short)(x + j % structure.x_size), (unsigned short)(server_blocks->getHeight() - 326 + (j - j % structure.x_size) / structure.x_size) - structure.y_size, structure.blocks[j]);
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
        generateBiomes(x, noise);
    }
    calculateHeight(noise);
    for (int x = 0; x < server_blocks->getWidth(); x++) {
        terrainGenerator(x, noise);
        generating_current++;
    }
    generateCaves(noise);
    for (const structurePosition& i : structurePositions) {
        generateStructure(i.name, i.x, i.y);
    }
    updateBlocks();
}

void worldGenerator::loadBiomes() {
    loaded_biomes.push_back(biome(Biome::ICY_SEAS, server_blocks->getHeight() / 3 * 2 - 50, 10,
                                  {}));
    loaded_biomes.push_back(biome(Biome::SNOWY_TUNDRA, server_blocks -> getHeight() / 3 * 2 + 20, 4,
                                   {}));
    loaded_biomes.push_back(biome(Biome::COLD_HILLS, server_blocks -> getHeight() / 3 * 2 + 29, 15,
                                   {}));
    loaded_biomes.push_back(biome(Biome::SNOWY_MOUNTAINS, server_blocks -> getHeight() / 3 * 2 + 70, 37,
                                  {}));
    loaded_biomes.push_back(biome(Biome::SEA, server_blocks->getHeight() / 3 * 2 - 50, 10,
                                  {}));
    loaded_biomes.push_back(biome(Biome::PLAINS, server_blocks -> getHeight() / 3 * 2 + 22, 4,
                                  {structureChance("tree_", 5, 20, 2)
                                  }));
    loaded_biomes.push_back(biome(Biome::FOREST, server_blocks -> getHeight() / 3 * 2 + 23, 10,
                                  {structureChance("tree_", 3, 6, 2)}));
    loaded_biomes.push_back(biome(Biome::MOUNTAINS, server_blocks -> getHeight() / 3 * 2 + 64, 33,
                                  {}));
    loaded_biomes.push_back(biome(Biome::WARM_OCEAN, server_blocks->getHeight() / 3 * 2 - 50, 10,
                                  {}));
    loaded_biomes.push_back(biome(Biome::DESERT, server_blocks -> getHeight() / 6 * 4 + 22, 4,
                                  {}));
    loaded_biomes.push_back(biome(Biome::SAVANA, server_blocks -> getHeight() / 3 * 2 + 26, 10,
                                  {}));
    loaded_biomes.push_back(biome(Biome::SAVANA_MOUNTAINS, server_blocks -> getHeight() / 3 * 2 + 50, 25,
                                  {}));
}
