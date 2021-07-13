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
#include <functional>
#endif

#include "print.hpp"
#include "serverNetworking.hpp"

Packet connection::getPacket() {
    return packet_manager.getPacket();
}

void connection::sendPacket(const Packet& packet) const {
    packet_manager.sendPacket(packet);
}

void connection::setSocket(int socket) {
    packet_manager.bindToSocket(socket);
}

int connection::getSocket() {
    return packet_manager.getSocket();
}

bool connection::isConnected() {
    return packet_manager.isSocketSet();
}

void serverNetworkingManager::sendToEveryone(const Packet& packet, connection* exclusion) {
    for(connection& conn : connections)
        if(conn.isConnected())
            if((!exclusion || conn.getSocket() != exclusion->getSocket()) && conn.registered)
                conn.sendPacket(packet);
}

void serverNetworkingManager::onPacket(Packet& packet, connection& conn) {
    ServerPacketEvent(packet, conn).call();
}

void serverNetworkingManager::listenerLoop() {
    listener_running = true;
    while(listener_running) {
        fd_set readfds;
        FD_ZERO(&readfds);

        FD_SET(server_fd, &readfds);
        int max_sd = server_fd;

        for(connection& conn : connections)
            if(conn.isConnected()) {
                FD_SET(conn.getSocket(), &readfds);
                if(conn.getSocket() > max_sd)
                    max_sd = conn.getSocket();
            }

        int activity = select(max_sd + 1, &readfds, nullptr, nullptr, nullptr);

        if((activity < 0) && (errno != EINTR))
            return;

        if(FD_ISSET(server_fd, &readfds)) {
            connection new_connection;
            sockaddr_in address;
            socklen_t addrlen = sizeof(address);
            int new_socket = accept(server_fd, (sockaddr*)&address, &addrlen);
            new_connection.ip = inet_ntoa(address.sin_addr);
            if((!accept_only_itself || new_connection.ip == "127.0.0.1") && new_socket != -1) {
                new_connection.setSocket(new_socket);
                for (connection& conn : connections)
                    if (!conn.isConnected()) {
                        conn = new_connection;
                        break;
                    }
            } else {
            #ifdef _WIN32
                closesocket(server_fd);
            #else
                close(server_fd);
            #endif
            }
        }

        for(connection& conn : connections)
            if(conn.isConnected()) {
                if (FD_ISSET(conn.getSocket(), &readfds)) {
                    Packet packet = conn.getPacket();
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

    addrinfo* result = NULL, hints;

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
