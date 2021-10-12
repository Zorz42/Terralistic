#ifndef worldGenerator_hpp
#define worldGenerator_hpp

#include <string>
#include <utility>
#include "serverBlocks.hpp"
#include "SimplexNoise.h"


struct structure {
    std::string name;
    int x_size, y_size, y_offset;
    BlockType* blocks;
    structure(std::string cname, int x, int y, int offset, BlockType* cBlocks) : name(std::move(cname)), x_size(x), y_size(y), y_offset(offset), blocks(cBlocks) {}
};

struct structurePosition {
    std::string name;
    int x, y;
    structurePosition(std::string cname, int cx, int cy) : name(std::move(cname)), x(cx), y(cy) {}
};

class worldGenerator {
    ServerBlocks* server_blocks;

    std::vector<structure> structures;
    std::vector<structurePosition> structurePositions;

    void generateBiomes(unsigned int x, SimplexNoise& noise);
    void calculateHeight(SimplexNoise& noise);
    void terrainGenerator(int x, SimplexNoise& noise);
    void generateSurface(unsigned int x, SimplexNoise& noise);
    void generateCaves(SimplexNoise &noise);
    void generateStructureWorld();
    void generateFlatTerrain();
    void generateStructuresForStrWorld();
    void updateBlocks();
    void generateDeafultWorld(SimplexNoise& noise);
    int heightGeneratorInt(unsigned int x, SimplexNoise& noise);
    static int heatGeneratorInt(unsigned int x, SimplexNoise& noise);
    void loadBiomes();

    void generateBlockIcyOcean(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockSnowyPlains(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockSnowyHills(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockSnowyMountains(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockOcean(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockPlains(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockHills(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockMountains(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockWarmOcean(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockDesert(unsigned int x, unsigned int y, SimplexNoise& noise);
    void generateBlockSavanaHills(unsigned int x, unsigned  int y, SimplexNoise& noise);
    void generateBlockSavanaMountains(unsigned int x, unsigned  int y, SimplexNoise& noise);

    void generateStructure(const std::string& name, int x, int y);
    void loadAssets();

    std::string resource_path;

    unsigned int generating_current = 0, generating_total = 1;

public:
    worldGenerator(ServerBlocks* server_blocks, std::string resource_path) : server_blocks(server_blocks), resource_path(std::move(resource_path)) {}

    unsigned int getGeneratingCurrent() const { return generating_current; }
    unsigned int getGeneratingTotal() const { return generating_total; }

    int generateWorld(unsigned short world_width, unsigned short world_height, unsigned int seed);
};

#endif
