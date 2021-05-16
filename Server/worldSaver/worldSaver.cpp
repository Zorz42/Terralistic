//
//  worldSaver.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#include <filesystem>
#include <fstream>
#include "worldSaver.hpp"

#include "print.hpp"

void worldSaver::saveWorld(const std::string& world_path, serverMap& world_map) {
    // saves world chunk by chunk and then inventory
    std::ofstream world_file(world_path + "/blockdata");
    std::filesystem::create_directory(world_path);
    std::filesystem::create_directory(world_path + "/playerdata");
    
    //for(auto & i : player_inventory.inventory)
        //world_file << (char)i.item_id << (char)i.getStack() << (char(i.getStack() >> 4));
    
    for(int y = 0; y < world_map.getWorldHeight(); y++)
        for(int x = 0; x < world_map.getWorldWidth(); x++)
            world_file << (char)world_map.getBlock(x, y).getType();
    world_file.close();
}

void worldSaver::loadWorld(const std::string& world_path, serverMap& world_map) {
    // loads world the same way it got saved but in reverse order
    
    std::ifstream world_file(world_path + "/blockdata");
    char c = 0;
    /*for(auto & i : player_inventory.inventory) {
        world_file >> std::noskipws >> c;
        i.item_id = (serverMap::itemType)c;
        world_file >> std::noskipws >> c;
        unsigned short stack = (unsigned char)c;
        world_file >> std::noskipws >> c;
        i.setStack(stack + ((unsigned short)(c) >> 4));
    }*/
    
    for(int y = 0; y < world_map.getWorldHeight(); y++)
        for(int x = 0; x < world_map.getWorldWidth(); x++) {
            world_file >> std::noskipws >> c;
            world_map.getBlock(x, y).setType((serverMap::blockType) c, false);
        }
    world_file.close();
}

bool worldSaver::worldExists(const std::string& world_path) {
    return std::filesystem::exists(world_path);
}
