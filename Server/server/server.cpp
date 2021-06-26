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
#include "players.hpp"
#include "server.hpp"

static bool running = false;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    running = false;
}

void serverInit() {
    initItems();
    initBlocks();
    initLiquids();
    initBlockEvents();
}

void server::start() {
    state = STARTING;
    print::info("Initialising server");
    
    if(working_dir.back() != '/')
        working_dir.push_back('/');
    running = true;
    
    server_blocks.createWorld(275, 75);
    
    if(std::filesystem::exists(working_dir + "world")) {
        state = LOADING_WORLD;
        print::info("Loading world...");
        //world_map.loadWorld(working_dir + "world");
    }
    else {
        state = GENERATING_WORLD;
        print::info("Generating world...");
        //world_map.generateTerrain(rand());
    }
    
    print::info("Post initializing modules...");
    
    server_blocks.setNaturalLight();
    
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
        
        server_items.updateItems(tick_length);
        server_players.lookForItems();
        server_players.updatePlayersBreaking(tick_length);
        server_players.updateBlocks();
    }
    
    std::cout << std::endl;
    
    if(!networking_manager.accept_only_itself) {
        packets::packet kick_packet(packets::KICK, (int)std::string("Server stopped!").size() + 1);
        kick_packet << std::string("Server stopped!");
        networking_manager.sendToEveryone(kick_packet);
    }
    
    state = STOPPING;
    print::info("Stopping server");
    
    networking_manager.stopListening();
    
    print::info("Saving world...");
    //world_map.saveWorld(working_dir + "world");
    
    state = STOPPED;
}

void server::stop() {
    running = false;
}

void server::setPrivate(bool is_private) {
    networking_manager.accept_only_itself = is_private;
}

unsigned int server::getGeneratingTotal() const {
    //return world_map.generating_total;
    return 1;
}

unsigned int server::getGeneratingCurrent() const {
    //return world_map.generating_current;
    return 0;
}
