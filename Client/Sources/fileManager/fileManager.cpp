#include <filesystem>
#include <set>

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

std::string fileManager::getDataPath() {
    return data_path;
}

std::string fileManager::getWorldsPath() {
    return data_path + "worlds/";
}

std::string removeEndUntilChars(std::string string, std::set<char> chars) {
    while(chars.count(string.back()) == 0 && !string.empty())
        string.pop_back();
    string.pop_back();
    return string;
}

std::string returnRndUntilChars(std::string string, std::set<char> chars) {
    int iter = (int)string.size() - 1;
    while(chars.count(string[iter]) == 0 && iter != 0)
        iter--;
    return string.substr(iter + 1, string.size());
}


std::string fileManager::getResourcePath(std::string executable_path) {
    std::string parent_directory = removeEndUntilChars(executable_path, {'/', '\\'});
    
    std::string parent_parent_directory = removeEndUntilChars(parent_directory, {'/', '\\'}) + "/";
    std::string parent_directory_name = returnRndUntilChars(parent_directory, {'/', '\\'});
    
    return parent_directory_name == "MacOS" ? parent_parent_directory + "Resources/" : parent_parent_directory + parent_directory_name + "/Resources/";
}
