//
//  fileManager.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/04/2021.
//

#include <filesystem>

#include "fileManager.hpp"
#include "platform_folders.h"

static std::string data_path;

void fileManager::init() {
    // data path is path in filesystem, where terralistic worlds are saved and other things
    data_path = sago::getDataHome() + "/Terralistic/";

    std::filesystem::create_directory(data_path);

    std::string dirs_to_create[] = {getWorldsPath()};

    for(const std::string& dir : dirs_to_create)
        std::filesystem::create_directory(dir);
}

std::string fileManager::getDataPath() {
    return data_path;
}

std::string fileManager::getWorldsPath() {
    return data_path + "worlds/";
}

std::string fileManager::getResourcePath(std::string executable_path) {
    while(executable_path[executable_path.size()-1] != '/' && executable_path[executable_path.size()-1] != '\\')
        executable_path.pop_back();
    executable_path.pop_back();
    std::string parent_directory;
    while(!executable_path.empty() && executable_path[executable_path.size()-1] != '/' && executable_path[executable_path.size()-1] != '\\') {
        parent_directory.insert(parent_directory.begin(), executable_path[executable_path.size()-1]);
        executable_path.pop_back();
    }
    return parent_directory == "MacOS" ? executable_path + "Resources/" : executable_path + parent_directory + "/Resources/";
}
