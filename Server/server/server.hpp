#ifndef server_hpp
#define server_hpp

#include <string>
#include <utility>
#include "serverNetworking.hpp"
#include "worldGenerator.hpp"
#include "serverEntities.hpp"
#include "serverItems.hpp"
#include "serverPlayers.hpp"
#include "serverBlocks.hpp"
#include "serverLiquids.hpp"
#include "serverChat.hpp"
#include "commands.hpp"

enum class ServerState {NEUTRAL, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED};

class Server {
    std::string world_path;
    ServerNetworking networking;
    ServerBlocks blocks;
    Biomes biomes;
    ServerLiquids liquids;
    WorldGenerator generator;
    ServerItems items;
    ServerPlayers players;
    ServerChat chat;
    Commands commands;
    ServerEntities entities;

    bool running = true;
    
    void saveWorld();
    void loadWorld();
    
    std::vector<ServerModule*> modules;
public:
    ServerState state = ServerState::NEUTRAL;
    
    Server(std::string resource_path, std::string world_path, unsigned short port);
    
    unsigned short seed;
    
    void start();
    void stop();
    
    void setPrivate(bool is_private);
    
    unsigned int getGeneratingTotal() const;
    unsigned int getGeneratingCurrent() const;
};

#endif
