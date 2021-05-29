//
//  clientNetworking.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#define WIN32_LEAN_AND_MEAN

#ifdef _WIN32
#include <iostream>
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
        packets::packet packet = packets::getPacket(manager->sock, manager->buffer, manager->bytes_received);
        for(packetListener* listener : manager->listeners)
            if(listener->listening_to.find(packet.type) != listener->listening_to.end())
                listener->onPacket(packet);
    }
#ifdef _WIN32
    closesocket(manager->sock);
    WSACleanup();
#else
    close(manager->sock);
#endif
}

bool networkingManager::establishConnection(const std::string &ip, unsigned short port) {
#ifdef _WIN32
    WSADATA wsaData;
    SOCKET ConnectSocket = INVALID_SOCKET;
    addrinfo* result = nullptr, hints;

    // Initialize Winsock
    if(WSAStartup(MAKEWORD(2, 2), &wsaData) != 0)
        return false;

    ZeroMemory(&hints, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;

    // Resolve the server address and port
    if(getaddrinfo(ip.c_str(), std::to_string(port).c_str(), &hints, &result) != 0) {
        WSACleanup();
        return false;
    }

    // Attempt to connect to an address until one succeeds
    for(addrinfo* ptr = result; ptr != NULL; ptr = ptr->ai_next) {
        // Create a SOCKET for connecting to server
        ConnectSocket = socket(ptr->ai_family, ptr->ai_socktype, ptr->ai_protocol);
        if(ConnectSocket == INVALID_SOCKET) {
            WSACleanup();
            return false;
        }

        // Connect to server.
        if(connect(ConnectSocket, ptr->ai_addr, (int)ptr->ai_addrlen) == SOCKET_ERROR) {
            closesocket(ConnectSocket);
            ConnectSocket = INVALID_SOCKET;
            continue;
        }
        break;
    }

    freeaddrinfo(result);

    if(ConnectSocket == INVALID_SOCKET) {
        WSACleanup();
        return false;
    }

    sock = ConnectSocket;
#else
    int curr_sock = sock;
    
    sockaddr_in serv_addr{};
    if((curr_sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
        return false;
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(port);

    if(inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) <= 0)
        return false;

    if(connect(curr_sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0)
        return false;
    
    sock = curr_sock;
#endif
    std::thread listener = std::thread(listenerLoop, this);
    listener.detach();

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
