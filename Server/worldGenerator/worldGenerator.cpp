#include "worldGenerator.hpp"
#include "SimplexNoise.h"
#include <fstream>
#include <vector>
#include <string>
#include "biomes.hpp"

int WorldGenerator::generateWorld(unsigned short world_width, unsigned short world_height, unsigned int seed) {
    SimplexNoise noise(seed);
    blocks->create(world_width, world_height);
    liquids->create();
    biomes->create();
    
    loadAssets();
    if(seed == 1000) {
        generateStructureWorld();
    } else {
        generating_total = blocks->getWidth();
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

int WorldGenerator::heatGeneratorInt(unsigned int x, SimplexNoise &noise) {
    int biome_heat = (noise.noise((float)x / 2000.0 + 0.125) + 1.0) * 1.5;
    return biome_heat == 3 ? 2 : biome_heat;
}

int WorldGenerator::heightGeneratorInt(unsigned int x, SimplexNoise& noise) {
    if (x < 50 || x > blocks->getWidth() - 50)
        return 0;
    else if (x < 100 || x > blocks->getWidth() - 100)
        return 1;
    else {
        int biome_heat = (noise.noise((float)x / 600.0 + 0.001) + 1) * 2;
        return std::min(std::max(1, biome_heat), 3);
    }
}

void WorldGenerator::biomeGeneratorSwitch(unsigned int x, SimplexNoise& noise) {
    /*
    sea level = 300, sea floor = 250
    plains = 325
    hills = 400
    mountains 600-700
    */

    int biome_heat = heatGeneratorInt(x, noise);
    int biome_height = heightGeneratorInt(x, noise);
    biomes->biomes[x] = (BiomeType)((biome_heat * 4) + biome_height);
    //biomes->biomes[x] = Biome::ICY_SEAS;
}

void WorldGenerator::terrainGenerator(int x, SimplexNoise& noise) {
    int surface_height = calculateHeight(x, noise);
    generateSurface(x, surface_height, noise);
    for(auto &checking_structure : loaded_biomes[(int)biomes->biomes[x]].structure_chances){
        if((noise.noise((float)x + 0.5, (float)surface_height + 0.5) + 1) * checking_structure.chance_on_each_block <= 2 && x > checking_structure.x_of_last_instance + checking_structure.least_distance_between_instances) {
            structurePositions.emplace_back(structurePosition(checking_structure.structure_name +
                                         std::to_string((int)((noise.noise((float)x - 0.5, (float)surface_height - 0.5) + 1) / 2 * checking_structure.unique_structures_of_type)),
                                         x, surface_height - 1));
            checking_structure.x_of_last_instance = x;
        }
    }
}

void WorldGenerator::generateSurface(int x, int surface_height, SimplexNoise &noise) {
    int last_layer = surface_height + 1;
    int generating_layer = 0;
    Biome &slice_biome = loaded_biomes[(int)biomes->biomes[x]];
    for(int y = std::max(blocks->getHeight() / 3 * 2, surface_height); y > 0; y--) {
        if (y > surface_height) {
            liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, LiquidType::WATER);
            liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, 127);
        }else{
            if (slice_biome.ground_layers[generating_layer].layer_height_mode == LayerHeightMode::PREVIOUS_LAYER) {
                if (y > last_layer - slice_biome.ground_layers[generating_layer].height +
                        noise.noise(x / 3 + 0.1, y * 2 + 0.5) *
                        slice_biome.ground_layers[generating_layer].height_variation)
                    blocks->setBlockTypeSilently(x, blocks->getHeight() - y, slice_biome.ground_layers[generating_layer].block);
                else {
                    last_layer = y + 1;
                    generating_layer++;
                    y++;
                }
            } else {
                if (slice_biome.ground_layers.size() != generating_layer + 1 && y < slice_biome.ground_layers[generating_layer + 1].height + noise.noise(x / 3 + 0.1, y * 2 + 0.5) * slice_biome.ground_layers[generating_layer + 1].height_variation) {
                    generating_layer++;
                    y++;
                } else if (y < slice_biome.ground_layers[generating_layer].height +
                               noise.noise(x / 3 + 0.1, y * 2 + 0.5) *
                               slice_biome.ground_layers[generating_layer].height_variation) {
                    blocks->setBlockTypeSilently(x, blocks->getHeight() - y, slice_biome.ground_layers[generating_layer].block);
                }
                else {
                    liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, LiquidType::WATER);
                    liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, 127);
                }
            }
        }
    }
}

int WorldGenerator::calculateHeight(int x, SimplexNoise& noise) {
    int biome_blend = 20;
    int slice_height = 0;
    int slice_height_variation = 0;
    float divide_at_end = 0;
        for(int i = std::max(0, x - biome_blend); i < std::min(blocks->getWidth() - 1, x + biome_blend); i++){
            slice_height += loaded_biomes[(int)biomes->biomes[i]].surface_height * (1 - (float)std::abs(x - i) / biome_blend);
            slice_height_variation += loaded_biomes[(int)biomes->biomes[i]].surface_height_variation * (1 - (float)std::abs(x - i) / biome_blend);
            divide_at_end += (1 - (float)std::abs(x - i) / biome_blend);
        }

    return (slice_height + turbulence(x + 0.003, 0, 64, noise) * slice_height_variation) / divide_at_end;
}

void WorldGenerator::loadAssets() {
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
        auto *blocks_arr = new BlockType[x_size * y_size];
        for (int i = 0; i < x_size * y_size; i++) {
            blocks_arr[i] = (BlockType)assetData[counter];
            counter++;
        }
        structures.emplace_back(name, x_size, y_size, y_offset, blocks_arr);
        previousEnd = counter;
    }

    structureFile.close();
    delete[] assetData;
}

void WorldGenerator::generateStructure(const std::string& name, int x, int y) {
    for (auto & structure : structures) {
        if (name == structure.name) {
            x -= structure.x_size / 2;
            y += structure.y_offset;
            for(int j = 0; j < structure.y_size * structure.x_size; j++)
                if(structure.blocks[j] != BlockType::NOTHING)
                    blocks->setBlockTypeSilently((unsigned short)(x + j % structure.x_size), (unsigned short)(blocks->getHeight() - y + (j - j % structure.x_size) / structure.x_size) - structure.y_size - 1, structure.blocks[j]);
            break;
        }
    }

}

void WorldGenerator::generateStructureWorld(){
    generateFlatTerrain();
    generateStructuresForStrWorld();
    updateBlocks();
}

void WorldGenerator::generateFlatTerrain() {
    for (int x = 0; x < blocks->getWidth(); x++) {
        biomes->biomes[x] = BiomeType::PLAINS;
    }
    for (int x = 0; x < blocks->getWidth(); x++) {
        for (int y = 0; y < blocks->getHeight(); y++) {
            if (y <= 324) {//generates surface
                blocks->setBlockTypeSilently((unsigned short)x, blocks->getHeight() - y - 1, BlockType::DIRT);
            }else if(y == 325)
                blocks->setBlockTypeSilently((unsigned short)x, blocks->getHeight() - y - 1, BlockType::GRASS_BLOCK);
        }
    }
}

void WorldGenerator::generateStructuresForStrWorld() {
    int x = 0;
    while(x < blocks->getWidth()){
        for (auto & structure : structures) {
            if(structure.y_size + x >= blocks->getWidth())
                return;
            for(int j = 0; j < structure.y_size * structure.x_size; j++)
                if(structure.blocks[j] != BlockType::NOTHING)
                    blocks->setBlockTypeSilently((unsigned short)(x + j % structure.x_size), (unsigned short)(blocks->getHeight() - 326 + (j - j % structure.x_size) / structure.x_size) - structure.y_size, structure.blocks[j]);
            x += structure.x_size + 1;
        }
    }
}

void WorldGenerator::updateBlocks() {
    //for(unsigned short x = 0; x < blocks->getWidth(); x++)
        //for(unsigned short y = 0; y < blocks->getHeight(); y++)
            //blocks->getBlock(x, y).update();
}

void WorldGenerator::generateDeafultWorld(SimplexNoise& noise) {
    for (int x = 0; x < blocks->getWidth(); x++) {
        biomeGeneratorSwitch(x, noise);
    }
    for (int x = 0; x < blocks->getWidth(); x++) {
        terrainGenerator(x, noise);
        generating_current++;
    }
    for (const structurePosition& i : structurePositions) {
        generateStructure(i.name, i.x, i.y);
    }
    updateBlocks();
}

void WorldGenerator::loadBiomes() {
    loaded_biomes.push_back(Biome(BiomeType::ICY_SEAS, blocks->getHeight() / 3 * 2, 0,
                                  {BiomeLayer(BlockType::ICE, LayerHeightMode::PREVIOUS_LAYER, 3, 1),
                                  BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight() / 3 * 2 - 50, 10)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SNOWY_TUNDRA, blocks -> getHeight() / 3 * 2 + 20, 4,
                                  {BiomeLayer(BlockType::SNOW_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   BiomeLayer(BlockType::SNOWY_GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                   {}));
    loaded_biomes.push_back(Biome(BiomeType::COLD_HILLS, blocks -> getHeight() / 3 * 2 + 29, 15,
                                  {BiomeLayer(BlockType::SNOW_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   BiomeLayer(BlockType::SNOWY_GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 1, 0),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 4, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                   {}));
    loaded_biomes.push_back(Biome(BiomeType::SNOWY_MOUNTAINS, blocks -> getHeight() / 3 * 2 + 70, 37,
                                  {BiomeLayer(BlockType::SNOW_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 2, 1),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SEA, blocks->getHeight() / 3 * 2 - 50, 10,
                                  {BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::PLAINS, blocks -> getHeight() / 6 * 4 + 22, 4,
                                  {BiomeLayer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {StructureChance("tree_", 5, 20, 2)
                                  }));
    loaded_biomes.push_back(Biome(BiomeType::FOREST, blocks -> getHeight() / 3 * 2 + 26, 10,
                                  {BiomeLayer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {StructureChance("tree_", 3, 6, 2)}));
    loaded_biomes.push_back(Biome(BiomeType::MOUNTAINS, blocks -> getHeight() / 3 * 2 + 70, 33,
                                  {BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::WARM_OCEAN, blocks->getHeight() / 3 * 2 - 50, 10,
                                  {BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::DESERT, blocks -> getHeight() / 6 * 4 + 22, 4,
                                  {BiomeLayer(BlockType::SAND, LayerHeightMode::PREVIOUS_LAYER, 6, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SAVANA, blocks -> getHeight() / 3 * 2 + 26, 10,
                                  {BiomeLayer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SAVANA_MOUNTAINS, blocks -> getHeight() / 3 * 2 + 50, 25,
                                  {BiomeLayer(BlockType::GRASS_BLOCK, LayerHeightMode::PREVIOUS_LAYER, 2, 0),
                                   BiomeLayer(BlockType::DIRT, LayerHeightMode::PREVIOUS_LAYER, 5, 2),
                                   BiomeLayer(BlockType::STONE_BLOCK, LayerHeightMode::WORLD_HEIGHT, blocks->getHeight(), 0)},
                                  {}));
}
