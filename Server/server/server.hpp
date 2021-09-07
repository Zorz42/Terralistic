#ifndef server_hpp
#define server_hpp

#include <string>
#include <utility>
#include "serverNetworking.hpp"
#include "worldGenerator.hpp"
#include "serverEntities.hpp"

enum class ServerState {NEUTRAL, STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED};

class Server {
    std::string world_path;
    ServerBlocks blocks;
    ServerItems items;
    Players players;
    ServerNetworkingManager networking_manager;
    ServerEntityManager entity_manager;
    
    worldGenerator generator;

    bool running = true;
    
    void saveWorld();
    void loadWorld();
public:
    ServerState state = ServerState::NEUTRAL;
    
    Server(std::string resource_path, std::string world_path);
    
    unsigned short seed;
    
    void start(unsigned short port);
    void stop();
    
    void setPrivate(bool is_private);
    
    unsigned int getGeneratingTotal() const;
    unsigned int getGeneratingCurrent() const;
};

#endif
