//
// Created by nejc on 12.07.21.
//

#include "biomes.hpp"

#include <utility>

Biome::Biome(BiomeType name, int height, int height_variation, std::vector <BiomeLayer> layers, std::vector <StructureChance> structure_chance) {
    biome_name = name;
    surface_height = height;
    surface_height_variation = height_variation;
    ground_layers = std::move(layers);
    structure_chances = std::move(structure_chance);
}

BiomeLayer::BiomeLayer(BlockType cblock, LayerHeightMode height_mode, int cheight, int variation) {
    block = cblock;
    layer_height_mode = height_mode;
    height = cheight;
    height_variation = variation;
}

StructureChance::StructureChance(std::string name, float chance_on_block, int least_distance, int unique_structures) {
    structure_name = std::move(name);
    chance_on_each_block = chance_on_block;
    least_distance_between_instances =  least_distance;
    x_of_last_instance = -10000;
    unique_structures_of_type = unique_structures;

}

void Biomes::create() {
    biomes = new BiomeType[blocks->getWidth()];
}
