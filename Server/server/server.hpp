//
//  server.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#ifndef server_hpp
#define server_hpp

#include <string>
#include "serverNetworking.hpp"
#include "serverMap.hpp"

void serverInit();

class server {
    std::string working_dir;
    serverNetworkingManager networking_manager;
    serverMap world_map;
public:
    enum serverState { NEUTRAL, STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED };
    serverState state = NEUTRAL;
    
    server(std::string working_dir, unsigned short port) : working_dir(working_dir), world_map(&networking_manager), networking_manager(port) {}
    
    void start();
    void stop();
    
    void setPrivate(bool is_private);
    
    inline unsigned short getPort() { return networking_manager.getPort(); }
};

#endif /* server_h */
