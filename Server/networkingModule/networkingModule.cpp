//
//  networkingModule.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifdef WIN32
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include <thread>
#include "print.hpp"
#include "map.hpp"

#define PORT 33770
#define PORT_STR "33770"

packets::packet connection::getPacket() const {
    return packets::getPacket(socket);
}

void networkingManager::registerListener(packetListener *listener) {
    listeners.push_back(listener);
}

void connection::sendPacket(const packets::packet& packet_) const {
    packets::sendPacket(socket, packet_);
}

void networkingManager::sendToEveryone(const packets::packet& packet, connection* exclusion) {
    for(connection& conn : connections)
        if(conn.socket != -1)
            if(!exclusion || conn.socket != exclusion->socket)
                conn.sendPacket(packet);
}

void networkingManager::onPacket(packets::packet& packet, connection& conn, networkingManager& manager) {
    for(packetListener* listener : manager.listeners)
        if(listener->listening_to.find(packet.type) != listener->listening_to.end())
            listener->onPacket(packet, conn);
}

void networkingManager::listenerLoop(networkingManager* manager, int server_fd) {
    int addrlen, new_socket, activity;
    int max_sd;
    struct sockaddr_in address{};
         
    fd_set readfds;
    
    
    while(true) {
        FD_ZERO(&readfds);
     
        FD_SET(server_fd, &readfds);
        max_sd = server_fd;
            
        for(connection& conn : manager->connections)
            if(conn.socket != -1) {
                FD_SET(conn.socket, &readfds);
                if(conn.socket > max_sd)
                    max_sd = conn.socket;
            }
     
        activity = select(max_sd + 1, &readfds, nullptr, nullptr, nullptr);
       
        if((activity < 0) && (errno != EINTR))
            return;
             
        if(FD_ISSET(server_fd, &readfds)) {
            if((new_socket = accept(server_fd, (sockaddr*)&address, (socklen_t*)&addrlen)) < 0)
                return;
            
            connection new_connection;
            new_connection.socket = new_socket;
            new_connection.ip = inet_ntoa(address.sin_addr);
            for(connection& conn : manager->connections)
                if(conn.socket == -1) {
                    conn = new_connection;
                    break;
                }
        }
             
        for(connection& conn : manager->connections)
            if(conn.socket != -1) {
                if (FD_ISSET(conn.socket, &readfds)) {
                    packets::packet packet = conn.getPacket();
                    onPacket(packet, conn, *manager);
                }
        }
    }
}


void networkingManager::startListening() {
#ifdef WIN32
    WSADATA wsa_data;
    if(WSAStartup(MAKEWORD(2,2), &wsa_data) != 0) {
        print::error("WSAStartup failed");
        exit(EXIT_FAILURE);
    }
#endif
    int opt = 1, server_fd;
    
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
    address.sin_port = htons(PORT);

    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        print::error("bind to port failed");
        exit(EXIT_FAILURE);
    }

    if (listen(server_fd, SOMAXCONN) < 0) {
        print::error("listen failed");
        exit(EXIT_FAILURE);
    }
    
    std::thread listener_thread(networkingManager::listenerLoop, this, server_fd);
    listener_thread.detach();
}
