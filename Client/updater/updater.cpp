#include <SFML/Network.hpp>
#include <iostream>
#include <fstream>
#include <filesystem>
#include "updater.hpp"
#include "versions.hpp"
#include "platform_folders.h"

bool checkForUpdatesMacOS(std::string exec_path);
bool checkForUpdatesWindows(std::string exec_path);
bool checkForUpdatesLinux(std::string exec_path);

bool checkForUpdates(const std::string& exec_path) {
#ifdef _WIN32
    return checkForUpdatesWindows(exec_path);
#endif
    
#ifdef __APPLE__
    return checkForUpdatesMacOS(exec_path);
#endif
    
#ifdef __linux__
    return checkForUpdatesLinux(exec_path);
#endif
}

void getPatchFromServer(const std::string& version) {
    sf::TcpSocket server_socket;
    if(server_socket.connect("jakob.zorz.si", 65431) != sf::Socket::Done)
        return;

    server_socket.send(version.c_str(), version.size());

    std::string buffer;
    char temp_buffer[1024];
    std::size_t received;
    while(server_socket.receive(temp_buffer, 1024, received) == sf::Socket::Done) {
        buffer.insert(buffer.end(), &temp_buffer[0], &temp_buffer[0] + received);
    }

    if(!buffer.empty()) {
        std::ofstream patch_file(sago::getDataHome() + "/Terralistic/update.patch", std::ios::binary | std::ios::trunc);
        patch_file.write(&buffer[0], buffer.size());
    }
}

bool checkForUpdatesMacOS(std::string exec_path) {
    getPatchFromServer((std::string)"MacOS-" + CURR_VERSION_STR);

    while(exec_path.back() != '/')
        exec_path.pop_back();
    
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/update.patch")) {
        std::system(((std::string)"cd \"" + exec_path + "../..\" && patch -t -p3 < \"" + sago::getDataHome() + "/Terralistic/update.patch\"").c_str());
        std::filesystem::remove(sago::getDataHome() + "/Terralistic/update.patch");
        return true;
    }
    
    return false;
}

bool checkForUpdatesWindows(std::string exec_path) {
    getPatchFromServer((std::string)"Linux-" + CURR_VERSION_STR);

    while(exec_path.back() != '/')
        exec_path.pop_back();

    /*if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/update.patch")) {
        std::system(((std::string)"cd \"" + exec_path + "\" && patch -t -p3 < \"" + sago::getDataHome() + "/Terralistic/update.patch\"").c_str());
        std::filesystem::remove(sago::getDataHome() + "/Terralistic/update.patch");
        return true;
    }*/

    return false;
}

bool checkForUpdatesLinux(std::string exec_path) {
    getPatchFromServer((std::string)"Linux-" + CURR_VERSION_STR);

    while(exec_path.back() != '/')
        exec_path.pop_back();

    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/update.patch")) {
        std::system(((std::string)"cd \"" + exec_path + "\" && patch -t -p3 < \"" + sago::getDataHome() + "/Terralistic/update.patch\"").c_str());
        std::filesystem::remove(sago::getDataHome() + "/Terralistic/update.patch");
        return true;
    }

    return false;
}
