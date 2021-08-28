#include <filesystem>

#include "fileManager.hpp"
#include "platform_folders.h"

static std::string data_path;

void fileManager::init() {
    data_path = sago::getDataHome() + "/Terralistic/";

    std::filesystem::create_directory(data_path);

    std::string dirs_to_create[] = {getWorldsPath()};
    for(const std::string& dir : dirs_to_create)
        std::filesystem::create_directory(dir);
}

std::string fileManager::getWorldsPath() {
    return data_path + "worlds/";
}

std::string fileManager::getConfigPath() {
    return data_path + "config.txt";
}
