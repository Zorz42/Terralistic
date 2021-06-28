//
//  properties.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#ifndef properties_hpp
#define properties_hpp

#define UNBREAKABLE -1

#include <string>
#include <vector>

enum class blockType {NOTHING = -1, AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, SAND, SNOWY_GRASS_BLOCK, SNOW_BLOCK, ICE, NUM_BLOCKS};
enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS, NUM_ITEMS};
enum class liquidType {EMPTY, WATER, NUM_LIQUIDS};
enum class flowDirection {NONE, LEFT, RIGHT, BOTH = LEFT | RIGHT};
enum class biome {NO_BIOME, ICY_SEAS, SNOWY_TUNDRA, COLD_HILLS, SNOWY_MOUNTAINS, SEA, PLAINS, FOREST, MOUNTAINS, WARM_OCEAN, DESERT, SAVANA, SAVANA_MOUNTAINS, NUM_BIOMES};

struct uniqueBlock {
    uniqueBlock() = default;
    uniqueBlock(std::string name, bool ghost, bool transparent, short break_time, std::vector<blockType> connects_to) : ghost(ghost), transparent(transparent), name(std::move(name)), break_time(break_time), connects_to(connects_to) {}
    
    bool ghost, transparent;
    std::string name;
    std::vector<blockType> connects_to;
    short break_time;
};

struct uniqueItem {
    uniqueItem() = default;
    uniqueItem(std::string  name, unsigned short stack_size, blockType places) : name(std::move(name)), stack_size(stack_size), places(places) {}
    
    std::string name;
    unsigned short stack_size;
    blockType places;
};

struct uniqueLiquid {
    uniqueLiquid() = default;
    uniqueLiquid(std::string name, unsigned short flow_time, float speed_multiplier) : name(name), flow_time(flow_time), speed_multiplier(speed_multiplier) {}
    
    std::string name;
    unsigned short flow_time;
    float speed_multiplier;
};

void initProperties();
const uniqueBlock& getUniqueBlock(blockType type);
const uniqueItem& getUniqueItem(itemType type);
const uniqueLiquid& getUniqueLiquid(liquidType type);

#endif /* properties_hpp */
