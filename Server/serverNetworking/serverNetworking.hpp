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
    packets::packetBuffer buffer;
public:
    std::string ip;
    int socket{-1};
    bool disconnected = false, registered = false;
    
    packets::packet getPacket();
    void sendPacket(const packets::packet& packet_) const;
};

class serverNetworkingManager {
    bool listener_running = false;
    int server_fd;
    unsigned short port;
    std::vector<serverPacketListener*> listeners;
    std::thread listener_thread;
    void onPacket(packets::packet& packet, connection& conn);
    void listenerLoop();
public:
    serverNetworkingManager(unsigned short port) : port(port) {}
    
    bool accept_only_itself = false; // only accept 0.0.0.0 Used in singleplayer to basically have a private server
    connection connections[MAX_PLAYERS];
    
    void startListening();
    void stopListening();
    void sendToEveryone(const packets::packet& packet, connection* exclusion=nullptr);
    void registerListener(serverPacketListener* listener);
    
    inline unsigned short getPort() { return port; }
};

class serverPacketListener {
public:
    serverPacketListener(serverNetworkingManager* manager) { manager->registerListener(this); }
    std::set<packets::packetType> listening_to;
    virtual void onPacket(packets::packet& packet, connection& conn) = 0;
};

#endif /* serverNetworking_hpp */
