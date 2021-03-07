//
//  worldSaver.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include "core.hpp"

#include "worldSaver.hpp"
#include "blockEngine.hpp"

void worldSaver::createWorld() {
    terrainGenerator::generateTerrainDaemon(0);
    saveWorld();
}

void worldSaver::loadWorld() {
    std::ifstream world_file("world/blockdata");
    char c = 0;
    
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++) {
            world_file >> std::noskipws >> c;
            blockEngine::getBlock(x, y).block_id = (blockEngine::blockType) c;
        }
    
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++)
            blockEngine::getBlock(x, y).update();
}

void worldSaver::saveWorld() {
    fileSystem::createDirIfNotExists("world");
    std::ofstream world_file("world/blockdata");
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++) {
            world_file << (char)blockEngine::getBlock(x, y).block_id;
        }
    world_file.close();
}
