//
//  serverNetworking.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#define MAX_PLAYERS 100

#include <string>
#include <set>
#include "packets.hpp"
#include <thread>
#include <vector>

class serverPacketListener;

class connection {
    PacketManager packet_manager;
public:
    std::string ip;
    bool disconnected = false, registered = false;
    
    int getSocket();
    void setSocket(int socket);
    
    Packet getPacket();
    void sendPacket(const Packet& packet_) const;
};

class serverNetworkingManager {
    bool listener_running = false;
    int server_fd;
    unsigned short port;
    std::vector<serverPacketListener*> listeners;
    std::thread listener_thread;
    void onPacket(Packet& packet, connection& conn);
    void listenerLoop();
public:
    serverNetworkingManager(unsigned short port) : port(port) {}
    
    bool accept_only_itself = false; // only accept 0.0.0.0 Used in singleplayer to basically have a private server
    connection connections[MAX_PLAYERS];
    
    void startListening();
    void stopListening();
    void sendToEveryone(const Packet& packet, connection* exclusion=nullptr);
    void registerListener(serverPacketListener* listener);
    
    inline unsigned short getPort() { return port; }
};

class serverPacketListener {
public:
    serverPacketListener(serverNetworkingManager* manager) { manager->registerListener(this); }
    std::set<PacketType> listening_to;
    virtual void onPacket(Packet& packet, connection& conn) = 0;
};

#endif /* serverNetworking_hpp */
