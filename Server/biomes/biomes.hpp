#pragma once

#include "properties.hpp"
#include "blocks.hpp"
#include <vector>
#include <string>

struct BiomeLayer;
struct StructureChance;
enum class LayerHeightMode {PREVIOUS_LAYER, WORLD_HEIGHT};

struct Biome {
    BiomeType biome_name;
    int surface_height;
    int surface_height_variation;
    std::vector <BiomeLayer> ground_layers;
    std::vector <StructureChance> structure_chances;
    Biome(BiomeType name, int height, int height_variation, std::vector <BiomeLayer> layers, std::vector <StructureChance> structure_chance);
    Biome() = default;
};

struct BiomeLayer {
    BlockType block;
    LayerHeightMode layer_height_mode;
    int height;
    int height_variation;
    BiomeLayer(BlockType cblock, LayerHeightMode height_mode, int cheight, int variation);
};

struct StructureChance {
    std::string structure_name;
    float chance_on_each_block;
    int least_distance_between_instances;
    int x_of_last_instance;
    int unique_structures_of_type;
    StructureChance(std::string name, float chance_on_block, int least_distance,
                    int unique_structures);
};

inline std::vector <Biome> loaded_biomes;

class Biomes {
    Blocks* blocks;
public:
    Biomes(Blocks* blocks) : blocks(blocks) {}
    void create();
    BiomeType* biomes;
};
