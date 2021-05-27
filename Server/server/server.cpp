//
//  main.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 11/01/2021.
//

#include <thread>
#include <iostream>
#include <signal.h>
#include <filesystem>

#include "print.hpp"
#include "worldSaver.hpp"
#include "terrainGenerator.hpp"
#include "server.hpp"

static bool running = false;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    running = false;
}

void serverInit() {
    serverMap::initItems();
    serverMap::initBlocks();
}

void server::start() {
    state = STARTING;
    print::info("Initialising server");
    
    if(working_dir.back() != '/')
        working_dir.push_back('/');
    running = true;
    
    world_map.createWorld(275, 75);
    
    if(worldSaver::worldExists(working_dir + "world")) {
        state = LOADING_WORLD;
        print::info("Loading world...");
        worldSaver::loadWorld(working_dir + "world", world_map);
    }
    else {
        state = GENERATING_WORLD;
        print::info("Generating world...");
        terrainGenerator::generateTerrainDaemon(0, &world_map);
        worldSaver::saveWorld(working_dir + "world", world_map);
    }
    
    print::info("Post initializing modules...");
    
    world_map.setNaturalLight();
    
    for(unsigned short x = 0; x < world_map.getWorldWidth(); x++)
        for(unsigned short y = 0; y < world_map.getWorldHeight(); y++)
            world_map.getBlock(x, y).update();
    
    signal(SIGINT, inthand);
    networking_manager.startListening();
    
    state = RUNNING;
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
    
    state = STOPPING;
    print::info("Stopping server");
    
    networking_manager.stopListening();
    
    print::info("Saving world...");
    worldSaver::saveWorld(working_dir + "world", world_map);
    
    state = STOPPED;
}

void server::stop() {
    running = false;
}

void server::setPrivate(bool is_private) {
    networking_manager.accept_only_itself = is_private;
}

unsigned int server::getGeneratingTotal() {
    return terrainGenerator::generating_total;
}

unsigned int server::getGeneratingCurrent() {
    return terrainGenerator::generating_current;
}
