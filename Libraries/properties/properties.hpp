//
//  properties.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#ifndef properties_hpp
#define properties_hpp

enum class blockType {NOTHING = -1, AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, SAND, SNOWY_GRASS_BLOCK, SNOW_BLOCK, ICE, NUM_BLOCKS};
enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS, NUM_ITEMS};
enum class liquidType {EMPTY, WATER, NUM_LIQUIDS};
enum class flowDirection {NONE, LEFT, RIGHT, BOTH = LEFT | RIGHT};
enum class biome {NO_BIOME, ICY_SEAS, SNOWY_TUNDRA, COLD_HILLS, SNOWY_MOUNTAINS, SEA, PLAINS, FOREST, MOUNTAINS, WARM_OCEAN, DESERT, SAVANA, SAVANA_MOUNTAINS, NUM_BIOMES};

#endif /* properties_hpp */
