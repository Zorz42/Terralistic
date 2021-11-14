#pragma once
#include "blocks.hpp"
#include "serverModule.hpp"

enum class BiomeType {NO_BIOME = -1, ICY_SEAS, SNOWY_TUNDRA, COLD_HILLS, SNOWY_MOUNTAINS, SEA, PLAINS, FOREST, MOUNTAINS, WARM_OCEAN, DESERT, SAVANA, SAVANA_MOUNTAINS, NUM_BIOMES};

class StructureChance;

class Biome {
public:
    BiomeType type;
    int height;
    int height_variation;
    std::vector <StructureChance> structure_chances;
    Biome(BiomeType type, int height, int height_variation, std::vector<StructureChance> structure_chances);
    Biome() = default;
};


class StructureChance {
public:
    std::string name;
    float chance;
    int least_distance;
    int x_of_last_instance;
    int unique_structures;
    StructureChance(std::string name, float chance, int least_distance, int unique_structures);
};

inline std::vector <Biome> loaded_biomes;

class Biomes : public ServerModule {
    Blocks* blocks;
public:
    Biomes(Blocks* blocks) : blocks(blocks) {}
    void create();
    BiomeType* biomes;
};
