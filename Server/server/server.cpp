#include <thread>
#include <iostream>
#include <csignal>
#include <filesystem>
#include <chrono>
#include <zlib.h>
#include <fstream>

#include "print.hpp"
#include "serverPlayers.hpp"
#include "server.hpp"
#include "graphics.hpp"

#define TPS_LIMIT 100

Server* curr_server = nullptr;

void onInterrupt(int signum) {
    curr_server->stop();
    std::cout << std::endl;
}

void Server::loadWorld() {
    blocks.createWorld(4400, 1200);
    
    std::ifstream world_file(world_path, std::ios::binary);
    std::vector<char> world_file_serial((std::istreambuf_iterator<char>(world_file)), std::istreambuf_iterator<char>());
    
    unsigned long uncompressed_size = *(unsigned long*)&world_file_serial[world_file_serial.size() - 8];
    world_file_serial.erase(world_file_serial.end() - 8, world_file_serial.end());
    char* world_file_serial_uncompressed = new char[uncompressed_size];
    
    uncompress((Bytef*)world_file_serial_uncompressed, &uncompressed_size, (Bytef*)&world_file_serial[0], world_file_serial.size());
    
    world_file.close();
    char* iter = world_file_serial_uncompressed;
    iter = blocks.loadFromSerial(iter);
    
    while(iter < world_file_serial_uncompressed + uncompressed_size - 1)
        iter = players.addPlayerFromSerial(iter);
    
    delete[] world_file_serial_uncompressed;
}

void Server::saveWorld() {
    std::vector<char> world_file_serial;
    blocks.serialize(world_file_serial);
    
    for(const ServerPlayer* player : players.getAllPlayers())
        player->serialize(world_file_serial);
    
    unsigned long compressed_size = world_file_serial.size() * 1.1 + 12;
    char* world_file_serial_compressed = new char[compressed_size];
    
    compress((Bytef*)world_file_serial_compressed, &compressed_size, (Bytef*)&world_file_serial[0], world_file_serial.size());
    
    *(unsigned long*)(world_file_serial_compressed + compressed_size) = world_file_serial.size();
    compressed_size += 8;
    
    std::ofstream world_file(world_path, std::ios::trunc | std::ios::binary);
    world_file.write(world_file_serial_compressed, compressed_size);
    world_file.close();
    
    delete[] world_file_serial_compressed;
}

void Server::start(unsigned short port) {
    curr_server = this;

    if(working_dir.back() != '/')
        working_dir.push_back('/');

    if(std::filesystem::exists(world_path)) {
        state = ServerState::LOADING_WORLD;
        print::info("Loading world...");
        loadWorld();
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
        items.updateItems(tick_length);
        players.lookForItemsThatCanBePickedUp();
        players.updatePlayersBreaking(tick_length);
        players.updateBlocksInVisibleAreas();
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
