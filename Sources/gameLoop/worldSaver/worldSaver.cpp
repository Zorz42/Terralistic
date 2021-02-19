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
#include "itemEngine.hpp"
#include "playerHandler.hpp"
#include "init.hpp"

ogl::texture loading_text, saving_text;

REGISTER_INIT_FUNC
    loading_text.loadFromText("Loading world", {255, 255, 255});
    loading_text.scale = 3;
    saving_text.loadFromText("Saving world", {255, 255, 255});
    saving_text.scale = 3;
REGISTER_INIT_FUNC_END

void worldSaver::saveWorld(const std::string& world_name) {
    swl::setDrawColor(0, 0, 0);
    swl::clear();
    saving_text.render();
    swl::update();

    // saves world chunk by chunk and then inventory
    
    std::ofstream world_file(fileSystem::worlds_dir + world_name + ".world");
    for(auto & i : playerHandler::player_inventory.inventory)
        world_file << (char)i.item_id << (char)i.getStack() << (char(i.getStack() >> 4));
    
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++)
            world_file << (char)blockEngine::getBlock(x, y).block_id;
    world_file.close();
}

void worldSaver::loadWorld(const std::string& world_name) {
    swl::setDrawColor(0, 0, 0);
    swl::clear();
    loading_text.render();
    swl::update();
    
    // loads world the same way it got saved but in reverse order
    
    std::ifstream world_file(fileSystem::worlds_dir + world_name + ".world");
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
    
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++)
            blockEngine::getBlock(x, y).update(x, y);
}
