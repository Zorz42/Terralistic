#ifndef server_hpp
#define server_hpp

#include <string>
#include <utility>
#include "serverNetworking.hpp"
#include "worldGenerator.hpp"

enum class ServerState {NEUTRAL, STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED};

class Server {
    std::string working_dir;
    std::string world_path;
    ServerBlocks blocks;
    ServerItems items;
    Players players;
    ServerNetworkingManager networking_manager;
    
    worldGenerator generator;

    bool running = true;
    
    void saveWorld();
    void loadWorld();
public:
    ServerState state = ServerState::NEUTRAL;
    
    Server(std::string working_dir, std::string resource_path, std::string world_path) : working_dir(std::move(working_dir)), blocks(), items(&blocks), players(&blocks, &items), networking_manager(&blocks, &items, &players), generator(&blocks, std::move(resource_path)), world_path(world_path) {}
    
    void start(unsigned short port);
    void stop();
    
    void setPrivate(bool is_private);
    
    unsigned int getGeneratingTotal() const;
    unsigned int getGeneratingCurrent() const;
};

#endif
