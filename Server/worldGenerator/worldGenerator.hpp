#pragma once
#include "perlinNoise.hpp"
#include "biomes.hpp"
#include "liquids.hpp"
#include "content.hpp"

class Structure {
public:
    std::string name;
    int x_size, y_size, y_offset;
    int* blocks;
    Structure(std::string cname, int x, int y, int offset, int* cBlocks) : name(std::move(cname)), x_size(x), y_size(y), y_offset(offset), blocks(cBlocks) {}
};

class WorldGenerator {
    Blocks* blocks;
    Walls* walls;
    Biomes* biomes;
    Liquids* liquids;
    
    std::vector<Structure> structures;
    
    void generateBiomes(int x, siv::PerlinNoise& noise);
    void calculateHeight(siv::PerlinNoise& noise);
    void terrainGenerator(int x, siv::PerlinNoise& noise);
    void generateSurface(int x, siv::PerlinNoise& noise);
    void generateCaves(siv::PerlinNoise &noise);
    void generateCaveLakes(std::mt19937& seeded_random);
    void generateLakeRecursively(int x, int y);
    void generateOres(siv::PerlinNoise& noise, std::mt19937& seeded_random);
    void generateOre(BlockType* type, float chance, int blob_distance, siv::PerlinNoise& noise, std::mt19937& seeded_random);
    void generateFoliage(std::mt19937& seeded_random);
    void placeStructures(siv::PerlinNoise& noise);
    void generateStructureWorld();
    void generateFlatTerrain();
    void generateStructuresForStrWorld();
    void generateDefaultWorld(siv::PerlinNoise& noise, std::mt19937& seeded_random);
    int heightGeneratorInt(int x, siv::PerlinNoise& noise);
    static int heatGeneratorInt(int x, siv::PerlinNoise& noise);
    void loadBiomes();
    
    void generateBlockIcyOcean(int x, int y, siv::PerlinNoise& noise);
    void generateBlockSnowyPlains(int x, int y, siv::PerlinNoise& noise);
    void generateBlockSnowyHills(int x, int y, siv::PerlinNoise& noise);
    void generateBlockSnowyMountains(int x, int y, siv::PerlinNoise& noise);
    void generateBlockOcean(int x, int y, siv::PerlinNoise& noise);
    void generateBlockPlains(int x, int y, siv::PerlinNoise& noise);
    void generateBlockHills(int x, int y, siv::PerlinNoise& noise);
    void generateBlockMountains(int x, int y, siv::PerlinNoise& noise);
    void generateBlockWarmOcean(int x, int y, siv::PerlinNoise& noise);
    void generateBlockDesert(int x, int y, siv::PerlinNoise& noise);
    void generateBlockSavanaHills(int x, int y, siv::PerlinNoise& noise);
    void generateBlockSavanaMountains(int x, int y, siv::PerlinNoise& noise);
    
    void generateStructure(const std::string& name, int x, int y);
    void loadAssets();
    
    void placeWalls();
    
    std::string resource_path;

    int generating_current = 0, generating_total = 1;
    int* surface_heights = nullptr;
    
    GameContent* content;
    
public:
    WorldGenerator(Blocks* blocks, Walls* walls, Liquids* liquids, Biomes* biomes, std::string resource_path, GameContent* content) : blocks(blocks), walls(walls), liquids(liquids), biomes(biomes), resource_path(std::move(resource_path)), content(content) {}

    int getGeneratingCurrent() const { return generating_current; }
    int getGeneratingTotal() const { return generating_total; }

    int generateWorld(int world_width, int world_height, int seed);
};
