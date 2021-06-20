//
//  worldSaver.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#include <filesystem>
#include <fstream>
#include "serverMap.hpp"

void serverMap::saveWorld(const std::string& world_path) {
    // saves world chunk by chunk and then inventory and position of every player
    std::filesystem::create_directory(world_path);
    std::filesystem::create_directory(world_path + "/playerdata");
    std::ofstream world_file(world_path + "/blockdata", std::ios::binary);

    for(int y = 0; y < height; y++) {
        char* world_buffer = new char[width * 3];
        for(int x = 0; x < width; x++) {
            serverMap::block curr_block = getBlock(x, y);
            int pos = x * 3;
            world_buffer[pos] = (char)curr_block.getType();
            world_buffer[pos + 1] = (char)curr_block.getLiquidType();
            world_buffer[pos + 2] = (char)curr_block.getLiquidLevel();
        }
        world_file.write(world_buffer, width * 3);
        delete[] world_buffer;
    }
    world_file.close();

    for(serverMap::player* player : all_players) {
        std::ofstream data_file(world_path + "/playerdata/" + player->name, std::ios::binary);
        for(auto& i : player->player_inventory.inventory_arr) {
            data_file << (char)i.getId();
            unsigned short stack = i.getStack();
            data_file.write((char*)&stack, sizeof(stack));
        }
        data_file.write((char*)&player->x, sizeof(player->x));
        data_file.write((char*)&player->y, sizeof(player->y));
        data_file.close();
    }
}

void serverMap::loadWorld(const std::string& world_path) {
    // loads world the same way it got saved

    std::ifstream world_file(world_path + "/blockdata", std::ios::binary);
    world_file.unsetf(std::ios::skipws);
    char c = 0;

    for(int y = 0; y < height; y++) {
        char* world_buffer = new char[width * 3];
        world_file.read(world_buffer, width * 3);

        for(int x = 0; x < width; x++) {
            serverMap::block curr_block = getBlock(x, y);
            int pos = x * 3;
            curr_block.setType((serverMap::blockType) world_buffer[pos], false);
            curr_block.setType((serverMap::liquidType) world_buffer[pos + 1], false);
            curr_block.setLiquidLevel(world_buffer[pos + 2]);
        }
        delete[] world_buffer;
    }
    world_file.close();

    for (const auto& entry : std::filesystem::directory_iterator(world_path + "/playerdata/")) {
        std::string player_name = entry.path().string();
        player_name = player_name.substr(player_name.find_last_of('/') + 1, player_name.size() - 1);
        serverMap::player* player = getPlayerByName(player_name);

        std::ifstream data_file(entry.path(), std::ios::binary);
        for(auto & i : player->player_inventory.inventory_arr) {
            data_file >> c;
            i.setId((serverMap::itemType)c);

            unsigned short stack;
            data_file.read((char*)&stack, sizeof(stack));
            i.setStack(stack);
        }

        data_file.read((char*)&player->x, sizeof(player->x));
        data_file.read((char*)&player->y, sizeof(player->y));
    }
}
