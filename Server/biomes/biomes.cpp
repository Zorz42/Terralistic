#include "biomes.hpp"

Biome::Biome(BiomeType type, int height, int height_variation, std::vector<StructureChance> structure_chances) : type(type), height(height), height_variation(height_variation), structure_chances(structure_chances) {}

StructureChance::StructureChance(std::string name, float chance, int least_distance, int unique_structures) : name(name), chance(chance), least_distance(least_distance), x_of_last_instance(-10000), unique_structures(unique_structures) {}

void Biomes::create() {
    biomes = new BiomeType[blocks->getWidth()];
}
