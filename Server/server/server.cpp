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
#include "worldGenerator.hpp"

static bool running = false;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    running = false;
}

void serverInit() {
    initBlockEvents();
}

void server::start() {
    state = STARTING;
    print::info("Initialising server");
    
    if(working_dir.back() != '/')
        working_dir.push_back('/');
    running = true;
    
    server_blocks.createWorld(275, 75);
    
    std::string world_path = working_dir + "world/";
    if(std::filesystem::exists(world_path)) {
        state = LOADING_WORLD;
        print::info("Loading world...");
        server_blocks.loadFrom(world_path + "blockdata");
        server_players.loadFrom(world_path + "playerdata/");
    }
    else {
        state = GENERATING_WORLD;
        print::info("Generating world...");
        generator.generateTerrain(rand());
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
    std::filesystem::create_directory(world_path);
    server_blocks.saveTo(world_path + "blockdata");
    server_players.saveTo(world_path + "playerdata/");
    
    state = STOPPED;
}

void server::stop() {
    running = false;
}

void server::setPrivate(bool is_private) {
    networking_manager.accept_only_itself = is_private;
}

unsigned int server::getGeneratingTotal() const {
    return generator.getGeneratingTotal();
}

unsigned int server::getGeneratingCurrent() const {
    return generator.getGeneratingCurrent();
}
