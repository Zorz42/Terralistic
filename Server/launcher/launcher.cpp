//
//  launcher.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/05/2021.
//

#include <filesystem>
#include "server.hpp"
#include "properties.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"

int main(int argc, char **argv) {
    initProperties();
    
    std::string data_folder = sago::getDataHome() + "/Terralistic-Server/";
    std::string resource_path = getResourcePath(argv[0]);
    
    if(!std::filesystem::exists(data_folder))
        std::filesystem::create_directory(data_folder);
    
    server main_server(data_folder, resource_path, 33770);
    main_server.start();
}
