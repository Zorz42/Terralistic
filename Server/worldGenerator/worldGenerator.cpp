#include <fstream>
#include "worldGenerator.hpp"
#include "content.hpp"

int WorldGenerator::generateWorld(int world_width, int world_height, int seed) {
    if(world_width <= 0 || world_height <= 0)
        throw Exception("World width and height must be a positive integer.");
    
    surface_heights = new int[world_width];
    
    siv::PerlinNoise noise((unsigned int)seed);
    std::mt19937 seeded_random(seed);
    blocks->create(world_width, world_height);
    walls->create();
    liquids->create();
    biomes->create();

    loadAssets();
    if(seed == 1000) {
        generateStructureWorld();
    } else {
        generating_total = blocks->getWidth() * 4;
        loadBiomes();
        generateDefaultWorld(noise, seeded_random);
    }
    
    delete[] surface_heights;
    surface_heights = nullptr;
    
    return 0;
}

double turbulence(double x, double y, double size, siv::PerlinNoise& noise) {
    double value = 0, initialSize = size;

    while(size >= 8) {
        value += noise.noise2D(x / size, y / size) * size;
        size /= 2.0;
    }

    return value / initialSize;
}

int WorldGenerator::heatGeneratorInt(int x, siv::PerlinNoise &noise) {
    int biome_heat = (noise.noise1D((float)x / 2000.0 + 0.125) + 1.0) * 1.5;
    return biome_heat == 3 ? 2 : biome_heat;
}

int WorldGenerator::heightGeneratorInt(int x, siv::PerlinNoise& noise) {
    if (x < 100 || x > blocks->getWidth() - 100)
        return 0;
    else if (x < 150 || x > blocks->getWidth() - 150)
        return 1;
    else {
        int biome_heat = (noise.noise1D((float)x / 600.0 + 0.001) + 1) * 1.5 + 1;
        return std::min(biome_heat, 3);
    }
}

void WorldGenerator::generateBiomes(int x, siv::PerlinNoise& noise) {
    int biome_heat = heatGeneratorInt(x, noise);
    int biome_height = heightGeneratorInt(x, noise);
    //biomes->biomes[x] = (BiomeType)((biome_heat * 4) + biome_height);
    biomes->biomes[x] = BiomeType::DESERT;
}

void WorldGenerator::terrainGenerator(int x, siv::PerlinNoise& noise) {
    generateSurface(x, noise);
}

void WorldGenerator::placeStructures(siv::PerlinNoise &noise) {
    for(int x = 0; x < blocks->getWidth(); x++) {
        for(auto &checking_structure : loaded_biomes[(int) biomes->biomes[x]].structure_chances) {
            if((noise.noise2D((float) x + 0.5, (float) surface_heights[x] + 0.5) + 1) * checking_structure.chance <= 2 && x > checking_structure.x_of_last_instance + checking_structure.least_distance) {
                
                BlockType *block_left = nullptr, *block_right = nullptr;
                if(checking_structure.name == "tree_") {
                    block_left = blocks->getBlockType(x - 1, blocks->getHeight() - surface_heights[x] - 1);
                    block_right = blocks->getBlockType(x + 1, blocks->getHeight() - surface_heights[x] - 1);
                }
                
                generateStructure(checking_structure.name + std::to_string((int) ((noise.noise2D((float) x - 0.5, (float) surface_heights[x] - 0.5) + 1) / 2 * checking_structure.unique_structures)), x, surface_heights[x] - 1);
                
                if(checking_structure.name == "tree_") {
                    if(block_left != &blocks->air)
                        blocks->setBlockTypeSilently(x - 1, blocks->getHeight() - surface_heights[x] - 1, block_left);
                    else if(blocks->getBlockType(x - 1, blocks->getHeight() - surface_heights[x])->transparent || blocks->getBlockType(x - 2, blocks->getHeight() - surface_heights[x] - 1) == &content->blocks.grass_block)
                        blocks->setBlockTypeSilently(x - 1, blocks->getHeight() - surface_heights[x] - 1, &blocks->air);
                    
                    if(block_right != &blocks->air)
                        blocks->setBlockTypeSilently(x + 1, blocks->getHeight() - surface_heights[x] - 1, block_right);
                    else if(blocks->getBlockType(x + 1, blocks->getHeight() - surface_heights[x])->transparent || blocks->getBlockType(x + 2, blocks->getHeight() - surface_heights[x] - 1) == &content->blocks.grass_block)
                        blocks->setBlockTypeSilently(x + 1, blocks->getHeight() - surface_heights[x] - 1, &blocks->air);
                }
                
                checking_structure.x_of_last_instance = x;
            }
        }
    }
}

void WorldGenerator::calculateHeight(siv::PerlinNoise& noise) {
    int biome_blend = 20;
    float divide_at_end;
    int *no_blend_height = new int[blocks->getWidth()];
    for(int current_slice = 0; current_slice < blocks->getWidth(); current_slice++) {
        no_blend_height[current_slice] = loaded_biomes[(int) biomes->biomes[current_slice]].height;
    }

    for(int current_slice = 0; current_slice < blocks->getWidth(); current_slice++) {
        divide_at_end = 0;
        surface_heights[current_slice] = 0;
        int variation = 0;
        for (int i = std::max(0, current_slice - biome_blend); i < std::min(blocks->getWidth() - 1, current_slice + biome_blend); i++) {
            surface_heights[current_slice] = surface_heights[current_slice] +  no_blend_height[i] * (1 - (float)std::abs(current_slice - i) / biome_blend);
            variation += loaded_biomes[(int) biomes->biomes[i]].height_variation * (1 - (float)std::abs(current_slice - i) / biome_blend);
            divide_at_end += (1 - (float)std::abs(current_slice - i) / biome_blend);
        }
        surface_heights[current_slice] = surface_heights[current_slice] / divide_at_end;
        variation /= divide_at_end;
        surface_heights[current_slice] = surface_heights[current_slice] + turbulence(current_slice + 0.003, 0, 64, noise) * variation;
    }
    delete[] no_blend_height;

}

void WorldGenerator::generateSurface(int x, siv::PerlinNoise &noise) {
    int generate_from = std::max(blocks->getHeight() / 3 * 2, (int)surface_heights[x]);
    for(int y = generate_from; y > 0; y--){
        int changed_x = std::max(std::min((int)(x + noise.noise2D(x + 0.5, y + 0.5) * 8), (int)blocks->getWidth()), 0);
        switch ((int)biomes->biomes[changed_x]) {
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

void WorldGenerator::generateBlockIcyOcean(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x])
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
    else if(y > blocks->getHeight() / 3 * 2 - noise.noise1D((float)x / 4 + 0.125) - 2)
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.ice_block);
    else {
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockSnowyPlains(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]){
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 20)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 0.5) - 5)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.dirt);
        else if(y == surface_heights[x] + ceil(noise.noise1D((float)x / 3 + 0.15) * 0.5) - 5)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.snowy_grass_block);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.snow_block);
    }else
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.ice_block);
}

void WorldGenerator::generateBlockSnowyHills(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 20)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 0.5) - 5)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.dirt);
        else if(y == surface_heights[x] + ceil(noise.noise1D((float)x / 3 + 0.15) * 0.5) - 5)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.snowy_grass_block);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.snow_block);
    }else
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.ice_block);
}

void WorldGenerator::generateBlockSnowyMountains(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + noise.noise1D((float)x / 3 + 0.15) * 0.5 - 5)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.snow_block);
    }else
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.ice_block);
}

void WorldGenerator::generateBlockOcean(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x])
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
    else{
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockPlains(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 15)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else if(y < surface_heights[x])
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.dirt);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.grass_block);
    }else
    if(y < blocks->getHeight() / 3 * 2){
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockHills(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 15)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else if(y < surface_heights[x])
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.dirt);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.grass_block);
    }else
    if(y < blocks->getHeight() / 3 * 2){
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockMountains(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x])
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
    else {
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockWarmOcean(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x])
        blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
    else{
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockDesert(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 15)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.sand);
    }else{
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockSavanaHills(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 15)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else if(y < surface_heights[x])
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.dirt);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.grass_block);
    }else
    if(y < blocks->getHeight() / 3 * 2){
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateBlockSavanaMountains(int x, int y, siv::PerlinNoise &noise) {
    if(y <= surface_heights[x]) {
        if(y < surface_heights[x] + (noise.noise1D((float)x / 3 + 0.15) * 1.8) - 25)
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.stone_block);
        else if(y < surface_heights[x])
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.dirt);
        else
            blocks->setBlockTypeSilently(x, blocks->getHeight() - y, &content->blocks.grass_block);
    }else
    if(y < blocks->getHeight() / 3 * 2){
        liquids->setLiquidTypeSilently(x, blocks->getHeight() - y, &content->liquids.water);
        liquids->setLiquidLevelSilently(x, blocks->getHeight() - y, MAX_LIQUID_LEVEL);
    }
}

void WorldGenerator::generateCaves(siv::PerlinNoise &noise) {
    for(int x = 0; x < blocks->getWidth(); x++) {
        for (int y = blocks->getHeight() - surface_heights[x] - 1; y < blocks->getHeight(); y++) {
            float value = turbulence((double)x / 2, (double)y, 64, noise) * std::min(std::max((float)0, ((float)blocks->getHeight() / 3 - y) / 300), (float)1);
            if (value > 0.3) {
                blocks->setBlockTypeSilently(x, y, &blocks->air);
                if (y == blocks->getHeight() - surface_heights[x])
                    surface_heights[x] = surface_heights[x] - 1;
            }else {
                value = turbulence((double) x / 4 + blocks->getWidth() * 3, (double)y / 2 + blocks->getHeight() * 3, 64, noise);
                if (value > -0.05 && value < 0.05) {
                    blocks->setBlockTypeSilently(x, y, &blocks->air);
                    if (y == blocks->getHeight() - surface_heights[x])
                        surface_heights[x] = surface_heights[x] - 1;
                }
            }
        }
        generating_current += 2;
    }
}

void WorldGenerator::generateCaveLakes(std::mt19937& seeded_random) {
    for(int i = 0; i < 500; i++){
        int x = seeded_random() % blocks->getWidth();
        int y = seeded_random() % (blocks->getHeight() / 3 * 2) + blocks->getHeight() / 3;
        if(blocks->getBlockType(x, y) == &blocks->air) {
            while(y < blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1) == &blocks->air)
                y++;
            generateLakeRecursively(x, y);
        }else
            continue;
    }
}

void WorldGenerator::generateLakeRecursively(int x, int y) {
    liquids->setLiquidTypeSilently(x, y, &content->liquids.water);
    liquids->setLiquidLevelSilently(x, y, MAX_LIQUID_LEVEL);
    
    if(y != blocks->getHeight() - 1 && blocks->getBlockType(x, y + 1) == &blocks->air && liquids->getLiquidType(x, y + 1) == &liquids->empty)
        generateLakeRecursively(x, y + 1);
    if(x != 0 && blocks->getBlockType(x - 1, y) == &blocks->air && liquids->getLiquidType(x - 1, y) == &liquids->empty)
        generateLakeRecursively(x - 1, y);
    if(x != blocks->getWidth() - 1 && blocks->getBlockType(x + 1, y) == &blocks->air && liquids->getLiquidType(x + 1, y) == &liquids->empty)
        generateLakeRecursively(x + 1, y);
}

void WorldGenerator::generateOres(siv::PerlinNoise& noise, std::mt19937& seeded_random) {
    generateOre(&content->blocks.iron_ore, 0.75, 15, noise, seeded_random);
    generateOre(&content->blocks.copper_ore, 0.75, 15, noise, seeded_random);
}

void WorldGenerator::generateOre(BlockType* type, float chance, int blob_distance, siv::PerlinNoise& noise, std::mt19937& seeded_random){
    int offset_x = seeded_random() % 10000;
    int offset_y = seeded_random() % 10000;
    for(int x = 0; x < blocks->getWidth(); x++){
        for(int y = 0; y < blocks->getHeight(); y++){
            if(blocks->getBlockType(x, blocks->getHeight() - y - 1) == &content->blocks.stone_block &&
               noise.noise2D_0_1((float)x / blob_distance + offset_x, (float)y / blob_distance + offset_y) > chance){
                    blocks->setBlockTypeSilently(x, blocks->getHeight() - y - 1, type);
            }
        }
        if(x % 2)
            generating_current++;
    }
}

void WorldGenerator::generateFoliage(std::mt19937& seeded_random) {
    for(int x = 0; x < blocks->getWidth(); x++) {
        if(liquids->getLiquidType(x, blocks->getHeight() - surface_heights[x] - 1) == &liquids->empty && blocks->getBlockType(x, blocks->getHeight() - surface_heights[x] - 1) == &blocks->air) {
            if(seeded_random() % 6 == 0)
                blocks->setBlockTypeSilently(x, blocks->getHeight() - surface_heights[x] - 1, &content->blocks.stone);
            else if(seeded_random() % 3 == 0)
                blocks->setBlockTypeSilently(x, blocks->getHeight() - surface_heights[x] - 1, &content->blocks.grass);
        }
    }
}


void WorldGenerator::loadAssets() {
    std::ifstream structureFile;
    structureFile.open(resource_path + "misc/structures.asset", std::ios::in);
    if(!structureFile.is_open())
        throw Exception("Could not open structures file");

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
        int x_size = (assetData[counter] << 8) + assetData[counter + 1];
        counter += 2;
        int y_size = (assetData[counter] << 8) + assetData[counter + 1];
        counter += 2;
        int *blocks_ = new int[x_size * y_size];
        for (int i = 0; i < x_size * y_size; i++) {
            blocks_[i] = (assetData[counter] << 8) + assetData[counter + 1] - 1;
            int curr_block_i = (assetData[counter] << 8) + assetData[counter + 1] - 1;
            counter += 2;
        }
        structures.emplace_back(name, x_size, y_size, blocks_);
        previousEnd = counter;
    }

    structureFile.close();
    delete[] assetData;
}

void WorldGenerator::generateStructure(const std::string& name, int x, int y) {
    for (auto & structure : structures) {
        if (name == structure.name) {
            x -= structure.x_size / 2;
            for(int j = 0; j < structure.y_size * structure.x_size; j++)
                if(structure.blocks[j] != -1)
                    blocks->setBlockTypeSilently(x + j % structure.x_size, blocks->getHeight() - y + (j - j % structure.x_size) / structure.x_size - structure.y_size - 1, blocks->getBlockTypeById(structure.blocks[j]));
            break;
        }
    }

}

void WorldGenerator::generateStructureWorld() {
    generateFlatTerrain();
    generateStructuresForStrWorld();
}

void WorldGenerator::generateFlatTerrain() {
    for (int x = 0; x < blocks->getWidth(); x++) {
        biomes->biomes[x] = BiomeType::PLAINS;
    }
    for (int x = 0; x < blocks->getWidth(); x++) {
        for (int y = 0; y < blocks->getHeight(); y++) {
            if (y <= 324) {
                blocks->setBlockTypeSilently(x, blocks->getHeight() - y - 1, &content->blocks.dirt);
            } else if(y == 325)
                blocks->setBlockTypeSilently(x, blocks->getHeight() - y - 1, &content->blocks.grass_block);
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
                if(structure.blocks[j] != -1)
                    blocks->setBlockTypeSilently(x + j % structure.x_size, blocks->getHeight() - 326 + (j - j % structure.x_size) / structure.x_size - structure.y_size, blocks->getBlockTypeById(structure.blocks[j]));
            x += structure.x_size + 1;
        }
    }
}

void WorldGenerator::placeWalls() {
    for(int x = 0; x < blocks->getWidth(); x++) {
        for (int y = 1; y < blocks->getHeight() - 1; y++)
            if (y > blocks->getHeight() - surface_heights[x - 1] && y > blocks->getHeight() - surface_heights[x] &&
                y > blocks->getHeight() - surface_heights[x + 1])
                walls->setWallTypeSilently(x, y, &content->walls.dirt);
        /*if(x % 8 == 0)
            generating_current++;*/
    }
}

void WorldGenerator::generateDefaultWorld(siv::PerlinNoise& noise, std::mt19937& seeded_random) {
    for (int x = 0; x < blocks->getWidth(); x++) {
        generateBiomes(x, noise);
    }
    calculateHeight(noise);
    for (int x = 0; x < blocks->getWidth(); x++) {
        terrainGenerator(x, noise);
        generating_current++;
    }
    placeWalls();
    generateCaves(noise);
    generateCaveLakes(seeded_random);
    generateOres(noise, seeded_random);
    placeStructures(noise);
    generateFoliage(seeded_random);
}

void WorldGenerator::loadBiomes() {
    loaded_biomes.push_back(Biome(BiomeType::ICY_SEAS, blocks->getHeight() / 3 * 2 - 50, 10,
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SNOWY_TUNDRA, blocks->getHeight() / 3 * 2 + 20, 4,
                                   {}));
    loaded_biomes.push_back(Biome(BiomeType::COLD_HILLS, blocks->getHeight() / 3 * 2 + 29, 15,
                                   {}));
    loaded_biomes.push_back(Biome(BiomeType::SNOWY_MOUNTAINS, blocks->getHeight() / 3 * 2 + 70, 37,
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SEA, blocks->getHeight() / 3 * 2 - 50, 10,
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::PLAINS, blocks->getHeight() / 3 * 2 + 22, 4,
                                  {StructureChance("tree_", 5, 20, 2)
                                  }));
    loaded_biomes.push_back(Biome(BiomeType::FOREST, blocks->getHeight() / 3 * 2 + 23, 10,
                                  {StructureChance("tree_", 3, 6, 2)}));
    loaded_biomes.push_back(Biome(BiomeType::MOUNTAINS, blocks->getHeight() / 3 * 2 + 64, 33,
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::WARM_OCEAN, blocks->getHeight() / 3 * 2 - 50, 10,
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::DESERT, blocks->getHeight() / 6 * 4 + 22, 4,
                                  {StructureChance("cactus_", 3, 5, 3)}));
    loaded_biomes.push_back(Biome(BiomeType::SAVANA, blocks->getHeight() / 3 * 2 + 26, 10,
                                  {}));
    loaded_biomes.push_back(Biome(BiomeType::SAVANA_MOUNTAINS, blocks->getHeight() / 3 * 2 + 50, 25,
                                  {}));
}
