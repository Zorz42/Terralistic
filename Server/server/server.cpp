#include <fstream>
#include <filesystem>
#include <signal.h>
#include <thread>
#include "print.hpp"
#include "server.hpp"
#include "compress.hpp"
#include "content.hpp"

#define TPS_LIMIT 60

Server* curr_server = nullptr;

void onInterrupt(int signum) {
    curr_server->stop();
    std::cout << std::endl;
}

Server::Server(const std::string& resource_path, const std::string& world_path, int port) :
    networking(port),
    world_saver(world_path),
    blocks(&networking, &world_saver),
    walls(&blocks, &world_saver, &networking),
    biomes(&blocks, &world_saver),
    liquids(&blocks, &networking, &world_saver),
    generator(&blocks, &walls, &liquids, &biomes, resource_path + "resourcePack/", &content),
    entities(&blocks, &networking),
    items(&entities, &blocks, &networking),
    players(&blocks, &entities, &items, &networking, &recipes, &world_saver),
    chat(&players, &networking),
    commands(&blocks, &players, &items, &entities, &chat),
    world_path(world_path),
    content(&blocks, &walls, &liquids, &items),
    resource_path(resource_path),
    seed((int)time(NULL))
{    
    modules = {
        &networking,
        &world_saver,
        &blocks,
        &walls,
        &biomes,
        &liquids,
        &entities,
        &items,
        &players,
        &chat,
        &commands,
    };
}

void Server::start() {
    curr_server = this;

    content.loadContent(&blocks, &walls, &liquids, &items, &recipes, resource_path + "resourcePack/");
    
    for(int i = 0; i < modules.size(); i++)
        modules[i]->init();
    
    if(std::filesystem::exists(world_path)) {
        state = ServerState::LOADING_WORLD;
        print::info("Loading world...");
        world_saver.load();
    } else {
        state = ServerState::GENERATING_WORLD;
        print::info("Generating world...");
        generator.generateWorld(4400, 1200, seed);
    }
    
    for(int i = 0; i < modules.size(); i++)
        modules[i]->postInit();
    
    content.blocks.addBlockBehaviour(&players);

    signal(SIGINT, onInterrupt);

    state = ServerState::RUNNING;
    print::info("Server has started!");
    
    int a, b = gfx::getTicks();
    
    int ms_per_tick = 1000 / TPS_LIMIT;
    
    while(running) {
        a = gfx::getTicks();
        float frame_length = a - b;
        if(frame_length < ms_per_tick)
            gfx::sleep(ms_per_tick - frame_length);
        b = a;
        
        for(int i = 0; i < modules.size(); i++)
            modules[i]->update(frame_length);
    }
    
    state = ServerState::STOPPING;
    
    print::info("Saving world...");
    world_saver.save();
    
    print::info("Stopping server...");
    for(int i = 0; i < modules.size(); i++)
        modules[i]->stop();

    state = ServerState::STOPPED;
}

void Server::stop() {
    running = false;
}

void Server::setPrivate(bool is_private) {
    networking.is_private = is_private;
}

int Server::getGeneratingTotal() const {
    return generator.getGeneratingTotal();
}

int Server::getGeneratingCurrent() const {
    return generator.getGeneratingCurrent();
}
