//
//  launcher.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#include <Server/server.hpp>

int main(int argc, char **argv) {
    serverInit();
    
    std::string working_directory = argv[0];
    while(working_directory.back() != '/')
        working_directory.pop_back();
    
    server main_server(working_directory, 33770);
    main_server.start();
}
