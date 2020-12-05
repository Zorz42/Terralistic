//
//  worldSaver.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#include "worldSaver.hpp"
#include "blockEngine.hpp"
#include "fileSystem.hpp"
#include <fileSystem>
#include <fstream>

void worldSaver::saveWorld(std::string world_name) {
    std::string path = fileSystem::worlds_dir + world_name + "/";
    if(!std::filesystem::exists(path))
        std::filesystem::create_directory(path);
    std::ofstream world_file(path + "blockdata");
    for(int i = 0; i < blockEngine::world_width * blockEngine::world_height; i++)
        world_file << (char)blockEngine::world[i].block_id;
    world_file.close();
}

void worldSaver::loadWorld(std::string world_name) {
    std::string path = fileSystem::worlds_dir + world_name + "/";
    
    std::ifstream world_file(path + "blockdata");
    for(int i = 0; i < blockEngine::world_width * blockEngine::world_height; i++) {
        char c;
        world_file >> std::noskipws >> c;
        blockEngine::world[i].block_id = (unsigned short) c;
    }
    for(unsigned long i = 0; i < blockEngine::world_width * blockEngine::world_height; i++)
        blockEngine::world[i].update();
}
