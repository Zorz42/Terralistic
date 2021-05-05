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
#include "serverNetworking.hpp"
#include "fileSystem.hpp"
#include "terrainGenerator.hpp"
#include "clickEvents.hpp"

static bool running = true;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    running = false;
}

int serverMain() {
    print::info("Starting server");
    
    print::info("Initializing modules");
    
    map::initItems();
    map::initBlocks();
    
    clickEvents::init();
    
    networkingManager networking_manager;
    
    map world_map(&networking_manager);
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
    
    world_map.setNaturalLight();
    
    for(unsigned short x = 0; x < world_map.getWorldWidth(); x++)
        for(unsigned short y = 0; y < world_map.getWorldHeight(); y++)
            world_map.getBlock(x, y).update();
    
    signal(SIGINT, inthand);
    networking_manager.startListening();
    
    print::info("Server has started!");
    long long a, b = ms_time();
    unsigned short tick_length;
    
    while(running) {
        a = ms_time();
        tick_length = a - b;
        if(tick_length < 50)
            std::this_thread::sleep_for(std::chrono::milliseconds(50 - tick_length));
        b = a;
        
        world_map.updateItems(tick_length);
        world_map.lookForItems(world_map);
        world_map.updatePlayersBreaking(tick_length);
        world_map.updateLight();
    }
    
    std::cout << std::endl;
    
    print::info("Stopping server");
    
    print::info("Saving world...");
    worldSaver::saveWorld("world", world_map);

    return 0;
}
