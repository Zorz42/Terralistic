//
//  worldSaver.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#include "worldSaver.hpp"
#include "fileSystem.hpp"
#include <fstream>

void worldSaver::saveWorld(const std::string& world_path, map& world_map) {
    // saves world chunk by chunk and then inventory
    std::ofstream world_file(world_path + ".world");
    //for(auto & i : playerHandler::player_inventory.inventory)
        //world_file << (char)i.item_id << (char)i.getStack() << (char(i.getStack() >> 4));
    
    for(int y = 0; y < world_map.getWorldHeight(); y++)
        for(int x = 0; x < world_map.getWorldWidth(); x++)
            world_file << (char)world_map.getBlock(x, y).getType();
    world_file.close();
}

void worldSaver::loadWorld(const std::string& world_path, map& world_map) {
    // loads world the same way it got saved but in reverse order
    
    std::ifstream world_file(world_path + ".world");
    char c = 0;
    /*for(auto & i : playerHandler::player_inventory.inventory) {
        world_file >> std::noskipws >> c;
        i.item_id = (map::itemType)c;
        world_file >> std::noskipws >> c;
        unsigned short stack = (unsigned char)c;
        world_file >> std::noskipws >> c;
        i.setStack(stack + ((unsigned short)(c) >> 4));
    }*/
    
    for(int y = 0; y < world_map.getWorldHeight(); y++)
        for(int x = 0; x < world_map.getWorldWidth(); x++) {
            world_file >> std::noskipws >> c;
            world_map.getBlock(x, y).setType((map::blockType) c, false);
        }
    
    for(int y = 0; y < (world_map.getWorldHeight() >> 4); y++)
        for(int x = 0; x < (world_map.getWorldWidth() >> 4); x++)
            world_map.getChunkState(x, y) = map::chunkState::loaded;
}

bool worldSaver::worldExists(const std::string& world_path) {
    return fileSystem::fileExists(world_path + ".world");
}
