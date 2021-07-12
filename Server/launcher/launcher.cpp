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
    initProperties();
    
    server main_server(std::filesystem::current_path().string(), std::filesystem::current_path().string(), 33770);
    main_server.start();
}
