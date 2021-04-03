//
//  networkingModule.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include "core.hpp"

#ifdef WIN32
#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "networkingModule.hpp"
#include "game.hpp"
#include "playerHandler.hpp"

#define PORT 33770
#define PORT_STR "33770"

void networking::networkingManager::sendPacket(packets::packet packet_) {
    packets::sendPacket(sock, std::move(packet_));
}

void networking::networkingManager::listenerLoop(networking::networkingManager* manager) {
    while(manager->listener_running) {
        packets::packet packet = packets::getPacket(manager->sock);
        for(packetListener* listener : manager->listeners)
            if(listener->listening_to.find(packet.type) != listener->listening_to.end())
                listener->onPacket(packet);
    }
}

bool networking::networkingManager::startListening(const std::string &ip) {
    #ifdef WIN32
    WSADATA wsaData;
    if(WSAStartup(MAKEWORD(2,2), &wsaData) != 0)
        return false;
    #endif

    sockaddr_in serv_addr{};
    if((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
        return false;
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(PORT);

    #ifdef WIN32
    struct addrinfo *result = nullptr, hints{};
    if(getaddrinfo(ip.c_str(), (const char *)PORT_STR, &hints, &result) != 0)
        return false;

    if(connect(sock, result->ai_addr, (int)result->ai_addrlen) == SOCKET_ERROR)
        return false;
    #else
    if(inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) <= 0)
        return false;

    if(connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0)
        return false;
    #endif

    sendPacket(packets::PLAYER_JOIN);
    
    std::thread listener = std::thread(listenerLoop, this);
    listener.detach();
    
    return true;
}

void networking::networkingManager::stopListening() {
    listener_running = false;
}

void networking::networkingManager::registerListener(packetListener *listener) {
    listeners.push_back(listener);
}

networking::packetListener::packetListener(networkingManager* manager) {
    manager->registerListener(this);
}
