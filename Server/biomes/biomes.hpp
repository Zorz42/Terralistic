#pragma once

#include "properties.hpp"
#include <vector>
#include <string>

struct layer;
struct structureChance;
enum class LayerHeightMode {PREVIOUS_LAYER, WORLD_HEIGHT};
class biome;


inline std::vector <biome> loaded_biomes;

class biome {
public:
    Biome biome_name;
    int surface_height;
    int surface_height_variation;
    std::vector <structureChance> structure_chances;
    biome(Biome name, int height, int height_variation, std::vector <structureChance> structure_chance);
};


struct structureChance{
    std::string structure_name;
    float chance_on_each_block;
    int least_distance_between_instances;
    int x_of_last_instance;
    int unique_structures_of_type;
    structureChance(std::string name, float chance_on_block, int least_distance, int unique_structures);
};
