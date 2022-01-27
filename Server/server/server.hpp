#pragma once
#include "worldGenerator.hpp"
#include "serverEntities.hpp"
#include "serverItems.hpp"
#include "serverBlocks.hpp"
#include "serverLiquids.hpp"
#include "commands.hpp"
#include "content.hpp"
#include "worldSaver.hpp"
#include "serverWalls.hpp"
#include "serverHealth.hpp"

enum class ServerState {NEUTRAL, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED, CRASHED};

class Server {
    std::string world_path;
    ServerNetworking networking;
    WorldSaver world_saver;
    ServerBlocks blocks;
    ServerWalls walls;
    Biomes biomes;
    ServerLiquids liquids;
    WorldGenerator generator;
    ServerItems items;
    ServerPlayers players;
    //ServerHealth health;
    ServerChat chat;
    Commands commands;
    ServerEntities entities;
    Recipes recipes;
    GameContent content;
    
    bool running = true;
    
    std::vector<ServerModule*> modules;
    std::string resource_path;
public:
    ServerState state = ServerState::NEUTRAL;
    
    Server(const std::string& resource_path, const std::string& world_path, int port);
    
    int seed;

    void start();
    void stop();
    
    void setPrivate(bool is_private);
    void enableAutosave(bool autosave_enabled);
    
    int getGeneratingTotal() const;
    int getGeneratingCurrent() const;
};
