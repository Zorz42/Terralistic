#include <filesystem>
#include "server.hpp"
#include "platform_folders.h"
#include "resourcePath.hpp"

int main(int argc, char **argv) {
    initProperties();
    
    std::string data_folder = sago::getDataHome() + "/Terralistic-Server/";
    
    if(!std::filesystem::exists(data_folder))
        std::filesystem::create_directory(data_folder);
    
    server main_server(data_folder, getResourcePath(argv[0]), 33770);
    main_server.start();
}
