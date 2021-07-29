#include <thread>
#include <iostream>
#include <signal.h>
#include <filesystem>
#include <chrono>

#include "print.hpp"
#include "serverPlayers.hpp"
#include "server.hpp"
#include "worldGenerator.hpp"
#include "graphics.hpp"

static bool running = false;

#define PORT 33770
#define TPS_LIMIT 100

void onInterrupt(int signum) {
    running = false;
    std::cout << std::endl;
}

void Server::start(unsigned short port) {
    if(working_dir.back() != '/')
        working_dir.push_back('/');

    std::string world_path = working_dir + "world/";
    if(std::filesystem::exists(world_path)) {
        state = ServerState::LOADING_WORLD;
        print::info("Loading world...");
        blocks.loadFrom(world_path + "blockdata");
        for (const auto& file : std::filesystem::directory_iterator(world_path + "playerdata/"))
            players.addPlayerFromFile(file.path().string());
    } else {
        state = ServerState::GENERATING_WORLD;
        print::info("Generating world...");
        if(working_dir.size() >= 16 && working_dir.substr(working_dir.length() - 16, 16) == "/StructureWorld/")
          generator.generateWorld(4400, 1200, 1000);
        else
          generator.generateWorld(4400, 1200, (unsigned int)std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch()).count());
    }

    print::info("Starting server...");
    state = ServerState::STARTING;

    blocks.setNaturalLight();

    signal(SIGINT, onInterrupt);
    networking_manager.openSocket(port);

    state = ServerState::RUNNING;
    print::info("Server has started!");
    unsigned int a, b = gfx::getTicks();
    unsigned short tick_length;
    
    int ms_per_tick = 1000 / TPS_LIMIT;
    
    running = true;
    while(running) {
        a = gfx::getTicks();
        tick_length = a - b;
        if(tick_length < ms_per_tick)
            gfx::sleep(ms_per_tick - tick_length);
        b = a;

        networking_manager.checkForNewConnections();
        networking_manager.getPacketsFromPlayers();
        items.updateItems(tick_length);
        players.lookForItemsThatCanBePickedUp();
        players.updatePlayersBreaking(tick_length);
        players.updateBlocksInVisibleAreas();
        networking_manager.syncLightWithPlayers();
    }

    if(!networking_manager.accept_itself) {
        sf::Packet kick_packet;
        kick_packet << PacketType::KICK << std::string("Server stopped!");
        networking_manager.sendToEveryone(kick_packet);
    }

    state = ServerState::STOPPING;
    print::info("Stopping server");
    networking_manager.closeSocket();

    print::info("Saving world...");
    std::filesystem::create_directory(world_path);
    blocks.saveTo(world_path + "blockdata");
    std::filesystem::create_directory(world_path + "playerdata/");
    for(const ServerPlayer* player : players.getAllPlayers())
        player->saveTo(world_path + "playerdata/" + player->name);

    state = ServerState::STOPPED;
}

void Server::stop() {
    running = false;
}

void Server::setPrivate(bool is_private) {
    networking_manager.accept_itself = is_private;
}

unsigned int Server::getGeneratingTotal() const {
    return generator.getGeneratingTotal();
}

unsigned int Server::getGeneratingCurrent() const {
    return generator.getGeneratingCurrent();
}
