//
//  fileSystem.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#include "fileSystem.hpp"
#include <vector>
//#include <iostream>
//#include <stdexcept>
//#include <cstdio>
//#include <cstdlib>
#include <filesystem>

#ifndef _WIN32
#include <pwd.h>
#include <unistd.h>

static std::string getHome() {
    int uid = getuid();
    const char* home_env = std::getenv("HOME");
    if(uid != 0 && home_env)
        return home_env;
    
    struct passwd* pw = nullptr;
    struct passwd pwd;
    long bufsize = sysconf(_SC_GETPW_R_SIZE_MAX);
    if(bufsize < 0)
        bufsize = 16384;
    
    std::vector<char> buffer;
    buffer.resize(bufsize);
    
    int error_code = getpwuid_r(uid, &pwd, buffer.data(), buffer.size(), &pw);
    if(error_code)
        throw std::runtime_error("Unable to get passwd struct.");
    
    const char* temp_res = pw->pw_dir;
    if(!temp_res)
        throw std::runtime_error("User has no home directory");
    return temp_res;
}

#endif

#ifdef _WIN32

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN

#endif

#include <windows.h>
#include <winerror.h>
#include <stringapiset.h>
#include <shlobj.h>

#elif defined(__APPLE__)
#else

#include <map>
#include <fstream>
#include <sys/types.h>
#include <cstring>
#include <sstream>

#endif

void createDirIfNotExists(std::string path) {
    if(!std::filesystem::exists(path))
        std::filesystem::create_directory(path);
}

void fileSystem::setDataPath() {
#ifdef _WIN32
    data_path = GetAppData() + "/Terralistic/";
#elif defined(__APPLE__)
    data_path = getHome()+"/Library/Application Support/Terralistic/";
#else
    data_path = getLinuxFolderDefault("XDG_DATA_HOME", ".local/share") + "/Terralistic/";
#endif
    
    createDirIfNotExists(data_path);
    
    worlds_dir = data_path + "worlds/";
    std::string dirs_to_create[] = {worlds_dir};
    
    for(std::string dir : dirs_to_create)
        createDirIfNotExists(dir);
}
