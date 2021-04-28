//
//  main.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 11/01/2021.
//

#include <thread>
#include <iostream>

#include "print.hpp"
#include "worldSaver.hpp"
#include "main.hpp"
#include "networkingModule.hpp"
#include "playerHandler.hpp"
#include "fileSystem.hpp"
#include "terrainGenerator.hpp"

volatile sig_atomic_t stop;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    stop = 1;
}

int main() {
    print::info("Starting server");
    
    print::info("Initializing modules");
    // LATER
    
    map::initItems();
    map::initBlocks();
    
    map world_map;
    world_map.createWorld(275, 75);
    
    if(worldSaver::worldExists("world")) {
        print::info("Loading world...");
        worldSaver::loadWorld("world", world_map);
    }
    else {
        print::info("Generating world...");
        terrainGenerator::generateTerrainDaemon(0, &world_map);
        worldSaver::saveWorld("world", world_map);
    }
    
    print::info("Post initializing modules...");
    
    signal(SIGINT, inthand);
    networking::spawnListener();
    
    print::info("Server has started!");
    long long a, b = ms_time();
    while(!stop) {
        a = ms_time();
        main_::frame_length = a - b;
        if(main_::frame_length < 50)
            std::this_thread::sleep_for(std::chrono::milliseconds(50 - main_::frame_length));
        b = a;
        
        world_map.updateItems(main_::frame_length);
        playerHandler::lookForItems(world_map);
        networking::updatePlayersBreaking(world_map);
    }
    
    std::cout << std::endl;
    
    print::info("Stopping server");
    
    print::info("Saving world...");
    worldSaver::saveWorld("world", world_map);

    return 0;
}
