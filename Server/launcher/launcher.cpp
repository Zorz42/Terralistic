//
//  launcher.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#include <filesystem>
#include "server.hpp"
#include "properties.hpp"

int main(int argc, char **argv) {
    serverInit();
    initProperties();
    
    server main_server(std::filesystem::current_path(), std::filesystem::current_path(), 33770);
    main_server.start();
}
