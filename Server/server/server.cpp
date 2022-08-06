#include <fstream>
#include <filesystem>
#include <csignal>
#include <thread>
#include "print.hpp"
#include "server.hpp"
#include "content.hpp"

#define TPS_LIMIT 20

Server* curr_server = nullptr;

void onInterrupt(int signum) {
    curr_server->stop();
    std::cout << std::endl;
}

Server::Server(const std::string& resource_path, const std::string& world_path, int port) :
    print(),
    networking(port, &print),
    world_saver(world_path, &print),
    blocks(&networking, &world_saver),
    walls(&blocks, &world_saver, &networking),
    biomes(&blocks, &world_saver),
    liquids(&blocks, &networking, &world_saver),
    generator(&blocks, &walls, &liquids, &biomes, resource_path + "resourcePack/", &content),
    entities(&blocks, &networking),
    items(&entities, &blocks, &walls, &networking),
    players(&blocks, &walls, &liquids, &entities, &items, &networking, &recipes, &world_saver),
    chat(&players, &networking, &print),
    commands(&blocks, &liquids, &players, &items, &entities, &chat),
    world_path(world_path),
    content(&blocks, &walls, &liquids, &items),
    resource_path(resource_path),
    seed((int)time(nullptr))
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
#ifdef __APPLE__
    pthread_setname_np("Server");
#endif

#ifdef __linux__
    pthread_setname_np(pthread_self(), "Server");
#endif
    curr_server = this;

    content.loadContent(&blocks, &walls, &liquids, &items, &recipes, resource_path + "resourcePack/");
    
    for(auto & module : modules)
        module->init();
    
    if(std::filesystem::exists(world_path)) {
        state = ServerState::LOADING_WORLD;
        print.info("Loading world...");
        world_saver.load();
    } else {
        state = ServerState::GENERATING_WORLD;
        print.info("Generating world...");
        generator.generateWorld(4400, 1200, seed);
    }
    
    for(auto & module : modules)
        module->postInit();
    
    content.blocks.addBlockBehaviour(&players);
    
    for(int x = 0; x < blocks.getWidth(); x++)
        for(int y = 0; y < blocks.getHeight(); y++)
            blocks.updateBlock(x, y);

    signal(SIGINT, onInterrupt);

    state = ServerState::RUNNING;
    print.info("Server has started!");
    
    int ms_per_tick = 1000 / TPS_LIMIT;
    
    float frame_length = 0;
    int frame_count = 0;
    
    ms_timer_counter = (int)ms_timer.getTimeElapsed();
    gfx::Timer tps_timer;
    while(running) {
        gfx::Timer timer;
        
        for(auto & module : modules)
            module->update(frame_length);
        
        while(ms_timer_counter < (int)ms_timer.getTimeElapsed()) {
            ms_timer_counter++;
            for(auto & module : modules)
                module->updateOnMs();
        }
        
        if((float)ms_per_tick > timer.getTimeElapsed())
            gfx::sleep((float)ms_per_tick - timer.getTimeElapsed());
        frame_length = timer.getTimeElapsed();
        
        frame_count++;
        if(tps_timer.getTimeElapsed() > 1000) {
            tps_timer.reset();
            Packet packet;
            packet << ServerPacketType::TPS << frame_count;
            networking.sendToEveryone(packet);
            frame_count = 0;
        }
    }
    
    state = ServerState::STOPPING;
    
    print.info("Saving world...");
    world_saver.save();
    
    print.info("Stopping server...");
    for(auto & module : modules)
        module->stop();

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

void Server::enableAutosave(bool autosave_enabled) {
    world_saver.autosave_enabled = autosave_enabled;
}
