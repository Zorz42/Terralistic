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
#define TPS_LIMIT 100

void inthand(int signum) {
    running = false;
    std::cout << std::endl;
}

void server::start() {
    state = STARTING;
    print::info("Initialising server");

    if(working_dir.back() != '/')
        working_dir.push_back('/');
    running = true;

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
        if(working_dir.substr(working_dir.length() - 16, 16) == "/StructureWorld/")
          generator.generateWorld(4400, 1200, 1000);
        else
          generator.generateWorld(4400, 1200, rand());
    }

    print::info("Post initializing modules...");

    server_blocks.setNaturalLight();

    signal(SIGINT, inthand);
    networking_manager.openSocket(port);

    state = RUNNING;
    print::info("Server has started!");
    long long a, b = ms_time();
    unsigned short tick_length;
    
    int ms_per_tick = 1000 / TPS_LIMIT;
    
    while(running) {
        a = ms_time();
        tick_length = a - b;
        if(tick_length < ms_per_tick)
            std::this_thread::sleep_for(std::chrono::milliseconds(ms_per_tick - tick_length));
        b = a;

        networking_manager.checkForNewConnections();
        networking_manager.getPacketsFromPlayers();
        server_items.updateItems(tick_length);
        server_players.lookForItems();
        server_players.updatePlayersBreaking(tick_length);
        server_players.updateBlocks();
    }

    if(!server_players.accept_itself) {
        sf::Packet kick_packet;
        kick_packet << PacketType::KICK << std::string("Server stopped!");
        networking_manager.sendToEveryone(kick_packet);
    }

    state = STOPPING;
    print::info("Stopping server");
    networking_manager.closeSocket();

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
    server_players.accept_itself = is_private;
}

unsigned int server::getGeneratingTotal() const {
    return generator.getGeneratingTotal();
}

unsigned int server::getGeneratingCurrent() const {
    return generator.getGeneratingCurrent();
}
