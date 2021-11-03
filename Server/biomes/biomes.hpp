#pragma once

#include "properties.hpp"
#include "blocks.hpp"
#include "serverModule.hpp"
#include <vector>
#include <string>

enum class BiomeType {NO_BIOME = -1, ICY_SEAS, SNOWY_TUNDRA, COLD_HILLS, SNOWY_MOUNTAINS, SEA, PLAINS, FOREST, MOUNTAINS, WARM_OCEAN, DESERT, SAVANA, SAVANA_MOUNTAINS, NUM_BIOMES};

struct StructureChance;

struct Biome {
    BiomeType biome_name;
    int surface_height;
    int surface_height_variation;
    std::vector <StructureChance> structure_chances;
    Biome(BiomeType name, int height, int height_variation, std::vector <StructureChance> structure_chance);
    Biome() = default;
};


struct StructureChance {
    std::string structure_name;
    float chance_on_each_block;
    int least_distance_between_instances;
    int x_of_last_instance;
    int unique_structures_of_type;
    StructureChance(std::string name, float chance_on_block, int least_distance, int unique_structures);
};

inline std::vector <Biome> loaded_biomes;

class Biomes : public ServerModule {
    Blocks* blocks;
public:
    Biomes(Blocks* blocks) : blocks(blocks) {}
    void create();
    BiomeType* biomes;
};
