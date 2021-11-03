#ifndef worldGenerator_hpp
#define worldGenerator_hpp

#include <string>
#include <utility>
#include "blocks.hpp"
#include "perlinNoise.hpp"
#include "biomes.hpp"
#include "liquids.hpp"
#include <random>


struct structure {
    std::string name;
    int x_size, y_size, y_offset;
    BlockTypeOld* blocks;
    structure(std::string cname, int x, int y, int offset, BlockTypeOld* cBlocks) : name(std::move(cname)), x_size(x), y_size(y), y_offset(offset), blocks(cBlocks) {}
};

struct structurePosition {
    std::string name;
    int x, y;
    structurePosition(std::string cname, int cx, int cy) : name(std::move(cname)), x(cx), y(cy) {}
};

class WorldGenerator {
    Blocks* blocks;
    Biomes* biomes;
    Liquids* liquids;

    std::vector<structure> structures;
    std::vector<structurePosition> structurePositions;

    void generateBiomes(unsigned int x, siv::PerlinNoise& noise);
    void calculateHeight(siv::PerlinNoise& noise);
    void terrainGenerator(int x, siv::PerlinNoise& noise);
    void generateSurface(unsigned int x, siv::PerlinNoise& noise);
    void generateCaves(siv::PerlinNoise &noise);
    void generateCaveLakes(std::mt19937& seeded_random);
    void generateLakeRecursively(int x, int y);
    void generateOres(siv::PerlinNoise& noise, std::mt19937& seeded_random);
    void generateOre(BlockTypeOld type, float chance, int blob_distance, siv::PerlinNoise& noise, std::mt19937& seeded_random);
    void generateStones(std::mt19937& seeded_random);
    void placeStructures(siv::PerlinNoise& noise);
    void generateStructureWorld();
    void generateFlatTerrain();
    void generateStructuresForStrWorld();
    void generateDeafultWorld(siv::PerlinNoise& noise, std::mt19937& seeded_random);
    int heightGeneratorInt(unsigned int x, siv::PerlinNoise& noise);
    static int heatGeneratorInt(unsigned int x, siv::PerlinNoise& noise);
    void loadBiomes();

    void generateBlockIcyOcean(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockSnowyPlains(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockSnowyHills(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockSnowyMountains(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockOcean(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockPlains(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockHills(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockMountains(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockWarmOcean(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockDesert(unsigned int x, unsigned int y, siv::PerlinNoise& noise);
    void generateBlockSavanaHills(unsigned int x, unsigned  int y, siv::PerlinNoise& noise);
    void generateBlockSavanaMountains(unsigned int x, unsigned  int y, siv::PerlinNoise& noise);

    void generateStructure(const std::string& name, int x, int y);
    void loadAssets();

    std::string resource_path;

    unsigned int generating_current = 0, generating_total = 1;
    
    unsigned short* surface_height;

public:
    WorldGenerator(Blocks* blocks, Liquids* liquids, Biomes* biomes, std::string resource_path) : blocks(blocks), liquids(liquids), biomes(biomes), resource_path(std::move(resource_path)) {}

    unsigned int getGeneratingCurrent() const { return generating_current; }
    unsigned int getGeneratingTotal() const { return generating_total; }

    int generateWorld(unsigned short world_width, unsigned short world_height, unsigned int seed);
};

#endif
