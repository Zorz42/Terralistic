#include <thread>
#include <iostream>
#include <csignal>
#include <filesystem>
#include <fstream>
#include <utility>

#include "print.hpp"
#include "serverPlayers.hpp"
#include "server.hpp"
#include "graphics.hpp"
#include "compress.hpp"

#define TPS_LIMIT 60

Server* curr_server = nullptr;

void onInterrupt(int signum) {
    curr_server->stop();
    std::cout << std::endl;
}

Server::Server(std::string resource_path, std::string world_path, unsigned short port) :
    networking(port),
    blocks(&networking),
    biomes(&blocks),
    liquids(&blocks, &networking),
    generator(&blocks, &liquids, &biomes, std::move(resource_path)),
    entities(&blocks, &networking),
    items(&entities, &blocks, &networking),
    players(&blocks, &entities, &items, &networking),
    chat(&players, &networking),
    commands(&blocks, &players, &items, &entities, &chat),
    world_path(std::move(world_path)),
    seed(time(NULL))
{
    modules = {
        &networking,
        &blocks,
        &biomes,
        &liquids,
        &entities,
        &items,
        &players,
        &chat,
        &commands,
    };
}

void Server::loadWorld() {    
    std::ifstream world_file(world_path, std::ios::binary);
    std::vector<char> world_file_serial((std::istreambuf_iterator<char>(world_file)), std::istreambuf_iterator<char>());
    
    world_file_serial = decompress(world_file_serial);
    
    world_file.close();
    char* iter = &world_file_serial[0];
    
    iter = blocks.loadFromSerial(iter);
    liquids.create();
    biomes.create();
    iter = liquids.loadFromSerial(iter);
    
    while(iter < &world_file_serial[0] + world_file_serial.size())
        iter = players.addPlayerFromSerial(iter);
}

void Server::saveWorld() {
    std::vector<char> world_file_serial;
    blocks.serialize(world_file_serial);
    liquids.serialize(world_file_serial);
    
    for(const ServerPlayerData* player : players.getAllPlayers())
        player->serialize(world_file_serial);
    
    world_file_serial = compress(world_file_serial);
    
    std::ofstream world_file(world_path, std::ios::trunc | std::ios::binary);
    world_file.write(&world_file_serial[0], world_file_serial.size());
    world_file.close();
}

void Server::start() {
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
    
    for(ServerModule* module : modules)
        module->init();

    signal(SIGINT, onInterrupt);

    state = ServerState::RUNNING;
    print::info("Server has started!");
    
    unsigned int a, b = gfx::getTicks();
    
    int ms_per_tick = 1000 / TPS_LIMIT;

    while(running) {
        a = gfx::getTicks();
        float frame_length = a - b;
        if(frame_length < ms_per_tick)
            gfx::sleep(ms_per_tick - frame_length);
        b = a;
        
        for(ServerModule* module : modules)
            module->update(frame_length);
    }
    
    state = ServerState::STOPPING;
    print::info("Stopping server");

    saveWorld();
    
    for(ServerModule* module : modules)
        module->stop();

    state = ServerState::STOPPED;
}

void Server::stop() {
    running = false;
}

void Server::setPrivate(bool is_private) {
    networking.is_private = is_private;
}

unsigned int Server::getGeneratingTotal() const {
    return generator.getGeneratingTotal();
}

unsigned int Server::getGeneratingCurrent() const {
    return generator.getGeneratingCurrent();
}
