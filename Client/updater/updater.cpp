#include <SFML/Network.hpp>
#include <iostream>
#include "updater.hpp"
#include "versions.hpp"

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

bool checkForUpdatesMacOS(std::string exec_path) {
    sf::TcpSocket server_socket;
    if(server_socket.connect("jakob.zorz.si", 65431) != sf::Socket::Done)
        return false;
    
    std::string version = "MacOS-";
    version += CURR_VERSION_STR;
    server_socket.send(version.c_str(), version.size());
    
    std::string buffer;
    char temp_buffer[1024];
    std::size_t received;
    while(server_socket.receive(temp_buffer, 1024, received) == sf::Socket::Done) {
        buffer.insert(buffer.end(), &temp_buffer[0], &temp_buffer[0] + received);
    }
    
    while(exec_path.back() != '/')
        exec_path.pop_back();
    
    if(!buffer.empty()) {
        std::system(((std::string)"cd " + exec_path + "../.. && patch -t -N -p3").c_str());
        return true;
    }
    return false;
}

bool checkForUpdatesWindows(std::string exec_path) {
    return false;
}

bool checkForUpdatesLinux(std::string exec_path) {
    return false;
}
