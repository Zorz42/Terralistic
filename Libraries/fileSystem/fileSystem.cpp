//
//  fileSystem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "fileSystem.hpp"
#include "platform_folders.h"
#include <sys/stat.h>

struct stat info;

bool fileSystem::dirExists(const std::string& path) {
    return stat(path.c_str(), &info ) == 0 && info.st_mode & S_IFDIR;
}

void fileSystem::createDirIfNotExists(const std::string& path) {
    if(!dirExists(path)) {
    #if defined(_WIN32)
        mkdir(path.c_str()); // can be used on Windows
    #else
        mode_t nMode = 0733; // UNIX style permissions
        mkdir(path.c_str(), nMode); // can be used on non-Windows
    #endif
    }
}

void fileSystem::setDataPath() {
    data_path = sago::getDataHome() + "/Terralistic/";
    
    createDirIfNotExists(data_path);
    
    worlds_dir = data_path + "worlds/";
    std::string dirs_to_create[] = {worlds_dir};
    
    for(const std::string& dir : dirs_to_create)
        createDirIfNotExists(dir);
}
