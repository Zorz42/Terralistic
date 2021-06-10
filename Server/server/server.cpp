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
#include "serverMap.hpp"
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
    serverMap::initLiquids();
}

void server::start() {
    state = STARTING;
    print::info("Initialising server");
    
    if(working_dir.back() != '/')
        working_dir.push_back('/');
    running = true;
    
    world_map.createWorld(275, 75);
    
    if(std::filesystem::exists(working_dir + "world")) {
        state = LOADING_WORLD;
        print::info("Loading world...");
        world_map.loadWorld(working_dir + "world");
    }
    else {
        state = GENERATING_WORLD;
        print::info("Generating world...");
        world_map.generateTerrain(rand());
    }
    
    print::info("Post initializing modules...");
    
    world_map.setNaturalLight();
    
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
        world_map.updateBlocks();
    }
    
    std::cout << std::endl;
    
    if(!networking_manager.accept_only_itself) {
        packets::packet kick_packet(packets::KICK);
        kick_packet << (std::string)"Server stopped!";
        networking_manager.sendToEveryone(kick_packet);
    }
    
    state = STOPPING;
    print::info("Stopping server");
    
    networking_manager.stopListening();
    
    print::info("Saving world...");
    world_map.saveWorld(working_dir + "world");
    
    state = STOPPED;
}

void server::stop() {
    running = false;
}

void server::setPrivate(bool is_private) {
    networking_manager.accept_only_itself = is_private;
}

unsigned int server::getGeneratingTotal() {
    return world_map.generating_total;
}

unsigned int server::getGeneratingCurrent() {
    return world_map.generating_current;
}
