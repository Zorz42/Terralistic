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
#include "players.hpp"
#include "worldGenerator.hpp"

class server {
    std::string working_dir;
    Blocks server_blocks;
    Items server_items;
    players server_players;
    
    worldGenerator generator;
    unsigned short port;
public:
    enum serverState { NEUTRAL, STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED };
    serverState state = NEUTRAL;
    
    server(std::string working_dir, std::string resource_path, unsigned short port) : working_dir(std::move(working_dir)), server_blocks(), server_items(&server_blocks), server_players(&server_blocks, &server_items), generator(&server_blocks, resource_path), port(port) {}
    
    void start();
    static void stop();
    
    void setPrivate(bool is_private);
    
    [[nodiscard]] unsigned int getGeneratingTotal() const;
    [[nodiscard]] unsigned int getGeneratingCurrent() const;
    
    inline unsigned short getPort() { return port; }
};

#endif /* server_h */
