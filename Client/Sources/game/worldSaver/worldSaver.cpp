//
//  worldSaver.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#include "core.hpp"

#include "worldSaver.hpp"
#include "fileSystem.hpp"
#include "playerHandler.hpp"
#include "textScreen.hpp"

void worldSaver::saveWorld(const std::string& world_name) {
    // saves world chunk by chunk and then inventory
    std::cout << fileSystem::getWorldsPath() + world_name + ".world" << std::endl;
    std::ofstream world_file(fileSystem::getWorldsPath() + world_name + ".world");
    for(auto & i : playerHandler::player_inventory.inventory)
        world_file << (char)i.item_id << (char)i.getStack() << (char(i.getStack() >> 4));
    
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++)
            world_file << (char)blockEngine::getBlock(x, y).block_id;
    world_file.close();
}

void worldSaver::loadWorld(const std::string& world_name) {
    // loads world the same way it got saved but in reverse order
    
    std::ifstream world_file(fileSystem::getWorldsPath() + world_name + ".world");
    char c = 0;
    for(auto & i : playerHandler::player_inventory.inventory) {
        world_file >> std::noskipws >> c;
        i.item_id = (itemEngine::itemType)c;
        world_file >> std::noskipws >> c;
        unsigned short stack = (unsigned char)c;
        world_file >> std::noskipws >> c;
        i.setStack(stack + ((unsigned short)(c) >> 4));
    }
    
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++) {
            world_file >> std::noskipws >> c;
            blockEngine::getBlock(x, y).block_id = (blockEngine::blockType) c;
        }
    
    for(int y = 0; y < (blockEngine::world_height >> 4); y++)
        for(int x = 0; x < (blockEngine::world_width >> 4); x++)
            blockEngine::getChunkState(x, y) = blockEngine::loaded;
}
