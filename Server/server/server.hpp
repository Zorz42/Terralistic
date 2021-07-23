//
//  server.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#ifndef server_hpp
#define server_hpp

#include <string>
#include <utility>
#include "serverNetworking.hpp"
#include "worldGenerator.hpp"

enum class ServerState {NEUTRAL, STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED};

class Server {
    std::string working_dir;
    Blocks blocks;
    Items items;
    Players players;
    NetworkingManager networking_manager;
    
    worldGenerator generator;
public:
    ServerState state = ServerState::NEUTRAL;
    
    Server(std::string working_dir, std::string resource_path) : working_dir(std::move(working_dir)), blocks(), items(&blocks), players(&blocks, &items), networking_manager(&blocks, &items, &players), generator(&blocks, resource_path) {}
    
    void start(unsigned short port);
    static void stop();
    
    void setPrivate(bool is_private);
    
    unsigned int getGeneratingTotal() const;
    unsigned int getGeneratingCurrent() const;
    
    //inline unsigned short getPort() { return port; }
};

#endif /* server_h */
