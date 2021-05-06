//
//  server.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#ifndef server_hpp
#define server_hpp

void serverInit();

class server {
public:
    enum serverState { STARTING, LOADING_WORLD, GENERATING_WORLD, RUNNING, SAVING_WORLD, STOPPED };
    serverState state;
    
    void start();
};

#endif /* server_h */
