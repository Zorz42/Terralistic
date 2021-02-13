//
//  fileSystem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "fileSystem.hpp"
#include "platform_folders.h"
#include <sys/stat.h>
#include <dirent.h>
#ifdef __APPLE__
#include <unistd.h>
#endif

struct stat info;

// most of the functions explain themselves

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

std::string fileSystem::getResourcePath(std::string executable_path) {
    while(executable_path[executable_path.size()-1] != '/' && executable_path[executable_path.size()-1] != '\\')
        executable_path.pop_back();
    executable_path.pop_back();
    std::string parent_directory;
    while(executable_path[executable_path.size()-1] != '/' && executable_path[executable_path.size()-1] != '\\') {
        parent_directory.insert(parent_directory.begin(), executable_path[executable_path.size()-1]);
        executable_path.pop_back();
    }
    return parent_directory == "MacOS" ? executable_path + "Resources/" : executable_path + parent_directory + "/Resources/";
}

void fileSystem::setDataPath() {
    // data path is path in filesystem, where terralistic worlds are saved and other things
    data_path = sago::getDataHome() + "/Terralistic/";
    
    createDirIfNotExists(data_path);
    
    worlds_dir = data_path + "worlds/";
    std::string dirs_to_create[] = {worlds_dir};
    
    for(const std::string& dir : dirs_to_create)
        createDirIfNotExists(dir);
}

int fileSystem::removeDir(const std::string &path) {
    DIR *d = opendir(path.c_str());
    size_t path_len = strlen(path.c_str());
    int r = -1;

    // recursively go trough each folder and file
    if(d) {
        dirent *p;

        r = 0;
        while(!r && (p=readdir(d))) {
            int r2 = -1;
            char *buf;
            size_t len;

            /* Skip the names "." and ".." as we don't want to recurse on them. */
            if(!strcmp(p->d_name, ".") || !strcmp(p->d_name, ".."))
                continue;

            len = path_len + strlen(p->d_name) + 2;
            buf = (char *) malloc(len);

            if(buf) {
                struct stat statbuf{};
                snprintf(buf, len, "%s/%s", path.c_str(), p->d_name);
                if(!stat(buf, &statbuf))
                    r2 = S_ISDIR(statbuf.st_mode) ? removeDir(buf) : unlink(buf);
                free(buf);
            }
            r = r2;
        }
        closedir(d);
    }

    if(!r)
        r = rmdir(path.c_str());
    return r;
}

void fileSystem::removeFile(const std::string &path) {
    unlink(path.c_str());
}

bool fileSystem::fileExists(const std::string& path) {
    return !stat(path.c_str(), &info);
}
