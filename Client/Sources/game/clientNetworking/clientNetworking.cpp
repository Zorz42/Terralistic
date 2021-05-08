//
//  clientNetworking.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifdef WIN32
#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "clientNetworking.hpp"
#include <thread>

void networkingManager::sendPacket(packets::packet packet_) {
    packets::sendPacket(sock, std::move(packet_));
}

void networkingManager::listenerLoop(networkingManager* manager) {
    while(manager->listener_running) {
        packets::packet packet = packets::getPacket(manager->sock);
        for(packetListener* listener : manager->listeners)
            if(listener->listening_to.find(packet.type) != listener->listening_to.end())
                listener->onPacket(packet);
    }
}

bool networkingManager::establishConnection(const std::string &ip, unsigned short port) {
    #ifdef WIN32
    WSADATA wsaData;
    if(WSAStartup(MAKEWORD(2,2), &wsaData) != 0)
        return false;
    #endif
    
    int curr_sock = sock;
    
    sockaddr_in serv_addr{};
    if((curr_sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
        return false;
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(port);

    #ifdef WIN32
    struct addrinfo *result = nullptr, hints{};
    if(getaddrinfo(ip.c_str(), std::to_string(port).c_str(), &hints, &result) != 0)
        return false;
    
    if(connect(sock, result->ai_addr, (int)result->ai_addrlen) == SOCKET_ERROR)
        return false;
    #else
    if(inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) <= 0)
        return false;

    if(connect(curr_sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0)
        return false;
    #endif
    
    std::thread listener = std::thread(listenerLoop, this);
    listener.detach();
    
    sock = curr_sock;
    
    return true;
}

void networkingManager::closeConnection() {
    listener_running = false;
}

void networkingManager::registerListener(packetListener *listener) {
    listeners.push_back(listener);
}

packetListener::packetListener(networkingManager* manager) {
    manager->registerListener(this);
}
