//
//  networkingModule.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include "core.hpp"

#ifdef WIN32
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "print.hpp"
#include "playerHandler.hpp"

#define PORT 33770
#define PORT_STR "33770"

int server_fd;

struct listener {
    networking::listenerFunction function;
    packets::packetType type;
};

std::vector<listener>& getListeners() {
    static std::vector<listener> listeners;
    return listeners;
}

networking::registerPacketListener::registerPacketListener(packets::packetType type, listenerFunction function) {
    listener new_listener{};
    new_listener.function = function;
    new_listener.type = type;
    getListeners().push_back(new_listener);
}

packets::packet networking::connection::getPacket() const {
    return packets::getPacket(socket);
}

void networking::connection::sendPacket(const packets::packet& packet_) const {
    packets::sendPacket(socket, packet_);
}

void networking::sendToEveryone(const packets::packet& packet, connection* exclusion) {
    for(networking::connection& conn : connections)
        if(conn.socket != -1)
            if(!exclusion || conn.socket != exclusion->socket)
                conn.sendPacket(packet);
}

void onPacket(packets::packet& packet, networking::connection& conn) {
    for(listener& i : getListeners())
        if(i.type == packet.type)
            i.function(packet, conn);
}

void listenerLoop() {
    int addrlen, new_socket, activity;
    int max_sd;
    struct sockaddr_in address{};
         
    fd_set readfds;
    
    
    while(true) {
        FD_ZERO(&readfds);
     
        FD_SET(server_fd, &readfds);
        max_sd = server_fd;
            
        for(networking::connection& conn : networking::connections)
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
            
            networking::connection new_connection;
            new_connection.socket = new_socket;
            new_connection.ip = inet_ntoa(address.sin_addr);
            for(networking::connection& conn : networking::connections)
                if(conn.socket == -1) {
                    conn = new_connection;
                    break;
                }
        }
             
        for(networking::connection& conn : networking::connections)
            if(conn.socket != -1) {
                if (FD_ISSET(conn.socket, &readfds)) {
                    packets::packet packet = conn.getPacket();
                    onPacket(packet, conn);
                }
        }
    }
}

void networking::spawnListener() {
#ifdef WIN32
    WSADATA wsa_data;
    if(WSAStartup(MAKEWORD(2,2), &wsa_data) != 0) {
        print::error("WSAStartup failed");
        exit(EXIT_FAILURE);
    }
#endif
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
    address.sin_port = htons(PORT);

    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        print::error("bind to port failed");
        exit(EXIT_FAILURE);
    }

    if (listen(server_fd, SOMAXCONN) < 0) {
        print::error("listen failed");
        exit(EXIT_FAILURE);
    }

    std::thread listener_thread(listenerLoop);
    listener_thread.detach();
}
