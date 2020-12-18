//
//  worldSaver.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/12/2020.
//

#include "worldSaver.hpp"
#include "blockEngine.hpp"
#include "fileSystem.hpp"
#include <fstream>
#include "objectedGraphicsLibrary.hpp"
#include "singleWindowLibrary.hpp"

ogl::texture loading_text, saving_text;

void worldSaver::init() {
    loading_text.loadFromText("Loading world", {255, 255, 255});
    loading_text.scale = 3;
    saving_text.loadFromText("Saving world", {255, 255, 255});
    saving_text.scale = 3;
}

void worldSaver::saveWorld(std::string world_name) {
    swl::setDrawColor(0, 0, 0);
    swl::clear();
    saving_text.render();
    swl::update();

    std::ofstream world_file(fileSystem::worlds_dir + world_name + ".world");
    for(int i = 0; i < blockEngine::world_width * blockEngine::world_height; i++)
        world_file << (char)blockEngine::world[i].block_id;
    world_file.close();
}

void worldSaver::loadWorld(std::string world_name) {
    swl::setDrawColor(0, 0, 0);
    swl::clear();
    loading_text.render();
    swl::update();
    
    std::ifstream world_file(fileSystem::worlds_dir + world_name + ".world");
    char c;
    for(int i = 0; i < blockEngine::world_width * blockEngine::world_height; i++) {
        world_file >> std::noskipws >> c;
        blockEngine::world[i].block_id = (blockEngine::blockType) c;
    }
    for(unsigned long i = 0; i < blockEngine::world_width * blockEngine::world_height; i++)
        blockEngine::world[i].update();
}
