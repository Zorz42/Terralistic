//
//  server.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#ifndef server_hpp
#define server_hpp

#include <string>

void serverInit();

class server {
    std::string working_dir;
public:
    enum serverState { NEUTRAL, STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, STOPPING, STOPPED };
    serverState state = NEUTRAL;
    
    server(std::string working_dir) : working_dir(working_dir) {}
    
    void start();
    void stop();
};

#endif /* server_h */
