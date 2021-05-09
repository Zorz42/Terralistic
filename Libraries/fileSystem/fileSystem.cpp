//
//  fileSystem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include <filesystem>

#include "fileSystem.hpp"

void fileSystem::removeDir(const std::string &path) {
    std::filesystem::remove_all(path);
}

void fileSystem::removeFile(const std::string &path) {
    std::filesystem::remove(path.c_str());
}

bool fileSystem::fileExists(const std::string& path) {
    return std::filesystem::exists(path) && (std::filesystem::is_block_file(path) || std::filesystem::is_regular_file(path) || std::filesystem::is_character_file(path));
}

bool fileSystem::dirExists(const std::string& path) {
    return std::filesystem::exists(path) && std::filesystem::is_directory(path);
}

void fileSystem::createDirIfNotExists(const std::string& path) {
    if(!dirExists(path))
        std::filesystem::create_directory(path);
}
