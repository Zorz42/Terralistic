#include <thread>
#include <iostream>
#include <csignal>
#include <filesystem>
#include <chrono>
#include <fstream>
#include <utility>

#include "print.hpp"
#include "serverPlayers.hpp"
#include "server.hpp"
#include "graphics.hpp"
#include "compress.hpp"

#define TPS_LIMIT 100

Server* curr_server = nullptr;

void onInterrupt(int signum) {
    curr_server->stop();
    std::cout << std::endl;
}

Server::Server(std::string resource_path, std::string world_path) :
    blocks(),
    items(&entities),
    players(&blocks, &entities, &items),
    networking_manager(&blocks, &entities, &players),
    generator(&blocks, std::move(resource_path)),
    world_path(std::move(world_path)),
    seed(std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch()).count()),
    entities(&blocks)
{}

void Server::loadWorld() {
    blocks.createWorld(4400, 1200);
    
    std::ifstream world_file(world_path, std::ios::binary);
    std::vector<char> world_file_serial((std::istreambuf_iterator<char>(world_file)), std::istreambuf_iterator<char>());
    
    world_file_serial = decompress(world_file_serial);
    
    world_file.close();
    char* iter = &world_file_serial[0];
    iter = blocks.loadFromSerial(iter);
    
    while(iter < &world_file_serial[0] + world_file_serial.size() + 7)
        iter = players.addPlayerFromSerial(iter);
}

void Server::saveWorld() {
    std::vector<char> world_file_serial;
    blocks.serialize(world_file_serial);
    
    for(const ServerPlayer* player : players.getAllPlayers())
        player->serialize(world_file_serial);
    
    world_file_serial = compress(world_file_serial);
    
    std::ofstream world_file(world_path, std::ios::trunc | std::ios::binary);
    world_file.write(&world_file_serial[0], world_file_serial.size());
    world_file.close();
}

void Server::start(unsigned short port) {
    curr_server = this;

    if(std::filesystem::exists(world_path)) {
        state = ServerState::LOADING_WORLD;
        print::info("Loading world...");
        loadWorld();
    } else {
        state = ServerState::GENERATING_WORLD;
        print::info("Generating world...");
        generator.generateWorld(4400, 1200, seed);
    }

    for(int x = 0; x < blocks.getWidth(); x++)
        blocks.setNaturalLight(x);
    
    print::info("Starting server...");
    state = ServerState::STARTING;

    signal(SIGINT, onInterrupt);
    networking_manager.openSocket(port);

    state = ServerState::RUNNING;
    print::info("Server has started!");
    unsigned int a, b = gfx::getTicks();
    unsigned short tick_length;
    
    int ms_per_tick = 1000 / TPS_LIMIT;

    while(running) {
        a = gfx::getTicks();
        tick_length = a - b;
        if(tick_length < ms_per_tick)
            gfx::sleep(ms_per_tick - tick_length);
        b = a;

        networking_manager.checkForNewConnections();
        networking_manager.getPacketsFromPlayers();
        entities.updateAllEntities(tick_length);
        players.lookForItemsThatCanBePickedUp();
        players.updatePlayersBreaking(tick_length);
        players.updateBlocksInVisibleAreas();
        networking_manager.flushPackets();
    }
    
    state = ServerState::STOPPING;

    if(!networking_manager.accept_itself) {
        sf::Packet kick_packet;
        kick_packet << PacketType::KICK << std::string("Server stopped!");
        networking_manager.sendToEveryone(kick_packet);
    }

    print::info("Stopping server");
    networking_manager.closeSocket();

    print::info("Saving world...");
    saveWorld();

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
