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
#include "server.hpp"

static bool running = false;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

networkingManager* networking_manager = nullptr;
serverMap* world_serverMap = nullptr;

void inthand(int signum) {
    running = false;
}

void serverInit() {
    serverMap::initItems();
    serverMap::initBlocks();
    
    clickEvents::init();
}

void server::start() {
    state = STARTING;
    print::info("Initialising server");
    
    if(working_dir.back() != '/')
        working_dir.push_back('/');
    running = true;
    
    networking_manager = new networkingManager();
    world_serverMap = new serverMap(networking_manager);
    
    world_serverMap->createWorld(275, 75);
    
    if(worldSaver::worldExists(working_dir + "world")) {
        state = LOADING_WORLD;
        print::info("Loading world...");
        worldSaver::loadWorld(working_dir + "world", *world_serverMap);
    }
    else {
        state = GENERATING_WORLD;
        print::info("Generating world...");
        terrainGenerator::generateTerrainDaemon(0, world_serverMap);
        worldSaver::saveWorld(working_dir + "world", *world_serverMap);
    }
    
    print::info("Post initializing modules...");
    
    world_serverMap->setNaturalLight();
    
    for(unsigned short x = 0; x < world_serverMap->getWorldWidth(); x++)
        for(unsigned short y = 0; y < world_serverMap->getWorldHeight(); y++)
            world_serverMap->getBlock(x, y).update();
    
    signal(SIGINT, inthand);
    networking_manager->startListening();
    
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
        
        world_serverMap->updateItems(tick_length);
        world_serverMap->lookForItems(*world_serverMap);
        world_serverMap->updatePlayersBreaking(tick_length);
        world_serverMap->updateLight();
    }
    
    std::cout << std::endl;
    
    state = STOPPING;
    print::info("Stopping server");
    
    networking_manager->stopListening();
    
    print::info("Saving world...");
    worldSaver::saveWorld("world", *world_serverMap);
    
    delete networking_manager;
    delete world_serverMap;
    
    networking_manager = nullptr;
    world_serverMap = nullptr;
    
    state = STOPPED;
}

void server::stop() {
    running = false;
}
