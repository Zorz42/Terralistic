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
    // saves world chunk by chunk and then inventory and position of every player
    std::ofstream world_file(world_path + "/blockdata", std::ios::binary);
    std::filesystem::create_directory(world_path);
    std::filesystem::create_directory(world_path + "/playerdata");
    
    for(int y = 0; y < world_map.getWorldHeight(); y++)
        for(int x = 0; x < world_map.getWorldWidth(); x++)
            world_file << (char)world_map.getBlock(x, y).getType() << (char)world_map.getBlock(x, y).getLiquidType() << (char)world_map.getBlock(x, y).getLiquidLevel();
    world_file.close();
    
    for(serverMap::player* player : world_map.getAllPlayers()) {
        std::ofstream data_file(world_path + "/playerdata/" + player->name, std::ios::binary);
        for(auto& i : player->inventory.inventory_arr) {
            data_file << (char)i.getId();
            unsigned short stack = i.getStack();
            data_file.write((char*)&stack, sizeof(stack));
        }
        data_file.write((char*)&player->x, sizeof(player->x));
        data_file.write((char*)&player->y, sizeof(player->y));
        data_file.close();
    }
}

void worldSaver::loadWorld(const std::string& world_path, serverMap& world_map) {
    // loads world the same way it got saved
    
    std::ifstream world_file(world_path + "/blockdata", std::ios::binary);
    char c = 0;
    
    for(int y = 0; y < world_map.getWorldHeight(); y++)
        for(int x = 0; x < world_map.getWorldWidth(); x++) {
            world_file >> std::noskipws >> c;
            world_map.getBlock(x, y).setType((serverMap::blockType) c, false);
            world_file >> std::noskipws >> c;
            world_map.getBlock(x, y).setType((serverMap::liquidType) c, false);
            world_file >> std::noskipws >> c;
            world_map.getBlock(x, y).setLiquidLevel(c);
        }
    world_file.close();
    
    for (const auto& entry : std::filesystem::directory_iterator(world_path + "/playerdata/")) {
        std::string player_name = entry.path().string();
        player_name = player_name.substr(player_name.find_last_of("/") + 1, player_name.size() - 1);
        serverMap::player* player = world_map.getPlayerByName(player_name);
        
        std::ifstream data_file(entry.path(), std::ios::binary);
        for(auto & i : player->inventory.inventory_arr) {
            data_file >> std::noskipws >> c;
            i.setId((serverMap::itemType)c);
            
            unsigned short stack;
            data_file.read((char*)&stack, sizeof(stack));
            i.setStack(stack);
        }
        
        data_file.read((char*)&player->x, sizeof(player->x));
        data_file.read((char*)&player->y, sizeof(player->y));
    }
}

bool worldSaver::worldExists(const std::string& world_path) {
    return std::filesystem::exists(world_path);
}
