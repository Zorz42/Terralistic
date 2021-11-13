#include "biomes.hpp"

Biome::Biome(BiomeType name, int height, int height_variation, std::vector <StructureChance> structure_chance) {
    biome_name = name;
    surface_height = height;
    surface_height_variation = height_variation;
    structure_chances = std::move(structure_chance);
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
