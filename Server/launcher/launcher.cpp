//
//  launcher.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#include <Server/server.hpp>

int main(int argc, char **argv) {
    serverInit();
    
    server main_server;
    main_server.start();
}
