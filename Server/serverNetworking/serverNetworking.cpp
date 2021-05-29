//
//  serverNetworking.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#include <functional>
#else
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "print.hpp"
#include "serverMap.hpp"

packets::packet connection::getPacket() const {
    return packets::getPacket(socket);
}

void serverNetworkingManager::registerListener(serverPacketListener *listener) {
    listeners.push_back(listener);
}

void connection::sendPacket(const packets::packet& packet_) const {
    packets::sendPacket(socket, packet_);
}

void serverNetworkingManager::sendToEveryone(const packets::packet& packet, connection* exclusion) {
    for(connection& conn : connections)
        if(conn.socket != -1)
            if((!exclusion || conn.socket != exclusion->socket) && conn.registered)
                conn.sendPacket(packet);
}

void serverNetworkingManager::onPacket(packets::packet& packet, connection& conn) {
    for(serverPacketListener* listener : listeners)
        if(listener->listening_to.find(packet.type) != listener->listening_to.end())
            listener->onPacket(packet, conn);
}

void serverNetworkingManager::listenerLoop() {
    int addrlen, new_socket, activity;
    int max_sd;
    struct sockaddr_in address{};
         
    fd_set readfds;
    
    
    while(listener_running) {
        FD_ZERO(&readfds);
     
        FD_SET(server_fd, &readfds);
        max_sd = server_fd;
            
        for(connection& conn : connections)
            if(conn.socket != -1) {
                FD_SET(conn.socket, &readfds);
                if(conn.socket > max_sd)
                    max_sd = conn.socket;
            }
     
        activity = select(max_sd + 1, &readfds, nullptr, nullptr, nullptr);
       
        if((activity < 0) && (errno != EINTR))
            return;
             
        if(FD_ISSET(server_fd, &readfds)) {
            connection new_connection;
            new_connection.ip = inet_ntoa(address.sin_addr);
            if (!accept_only_itself || new_connection.ip == "0.0.0.0") {
                new_socket = accept(server_fd, /*(sockaddr*)&address*/nullptr,/*(socklen_t*)&addrlen*/nullptr);
                if (new_socket != SOCKET_ERROR) {
                    new_connection.socket = new_socket;
                    for (connection& conn : connections)
                        if (conn.socket == -1) {
                            conn = new_connection;
                            break;
                        }
                }
            }
        }
             
        for(connection& conn : connections)
            if(conn.socket != -1) {
                if (FD_ISSET(conn.socket, &readfds)) {
                    packets::packet packet = conn.getPacket();
                    onPacket(packet, conn);
                }
        }
    }
#ifdef _WIN32
    closesocket(server_fd);
#else
    close(server_fd);
#endif
}


void serverNetworkingManager::startListening() {
#ifdef _WIN32
    WSADATA wsaData;
    int iResult;

    SOCKET ListenSocket = INVALID_SOCKET;

    struct addrinfo* result = NULL;
    struct addrinfo hints;

    int iSendResult;

    // Initialize Winsock
    iResult = WSAStartup(MAKEWORD(2, 2), &wsaData);
    if (iResult != 0) {
        printf("WSAStartup failed with error: %d\n", iResult);
        exit(EXIT_FAILURE);
    }

    ZeroMemory(&hints, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;
    hints.ai_flags = AI_PASSIVE;

    // Resolve the server address and port
    iResult = getaddrinfo(NULL, std::to_string(port).c_str(), &hints, &result);
    if (iResult != 0) {
        printf("getaddrinfo failed with error: %d\n", iResult);
        WSACleanup();
        exit(EXIT_FAILURE);
}

    // Create a SOCKET for connecting to server
    ListenSocket = socket(result->ai_family, result->ai_socktype, result->ai_protocol);
    if (ListenSocket == INVALID_SOCKET) {
        printf("socket failed with error: %ld\n", WSAGetLastError());
        freeaddrinfo(result);
        WSACleanup();
        exit(EXIT_FAILURE);
    }

    // Setup the TCP listening socket
    iResult = bind(ListenSocket, result->ai_addr, (int)result->ai_addrlen);
    if (iResult == SOCKET_ERROR) {
        printf("bind failed with error: %d\n", WSAGetLastError());
        freeaddrinfo(result);
        closesocket(ListenSocket);
        WSACleanup();
        exit(EXIT_FAILURE);
    }

    freeaddrinfo(result);

    iResult = listen(ListenSocket, SOMAXCONN);
    if (iResult == SOCKET_ERROR) {
        printf("listen failed with error: %d\n", WSAGetLastError());
        closesocket(ListenSocket);
        WSACleanup();
        exit(EXIT_FAILURE);
    }

    server_fd = ListenSocket;
#else
    int opt = 1;
    
    if ((server_fd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP)) == 0) {
        print::error("socket failed");
        exit(EXIT_FAILURE);
    }

    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, (const char *)(&opt), sizeof(opt))) {
        print::error("setsockopt failed");
        exit(EXIT_FAILURE);
    }

    sockaddr_in address{};
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    
    while(true) {
        address.sin_port = htons(port);
        if(bind(server_fd, (struct sockaddr *)&address, sizeof(address)) >= 0)
            break;
        else
            port++;
    }

    if (listen(server_fd, SOMAXCONN) < 0) {
        print::error("listen failed");
        exit(EXIT_FAILURE);
    }
#endif
    
    listener_running = true;
    listener_thread = std::thread(std::bind(&serverNetworkingManager::listenerLoop, this));
}

void serverNetworkingManager::stopListening() {
    listener_running = false;
#ifdef _WIN32
    closesocket(server_fd);
    WSACleanup();
#else
    close(server_fd);
#endif
    listener_thread.join();
}
