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

class packetListener;

class connection {
public:
    std::string ip;
    int socket{-1};
    bool disconnected = false;
    
    packets::packet getPacket() const;
    void sendPacket(const packets::packet& packet_) const;
};

class networkingManager {
    bool listener_running = true;
    std::vector<packetListener*> listeners;
    std::thread listener_thread;
    void onPacket(packets::packet& packet, connection& conn);
    void listenerLoop(int server_fd);
public:
    connection connections[MAX_PLAYERS];
    
    void startListening();
    void stopListening();
    void sendToEveryone(const packets::packet& packet, connection* exclusion=nullptr);
    void registerListener(packetListener* listener);
};

class packetListener {
public:
    packetListener(networkingManager* manager) { manager->registerListener(this); }
    std::set<packets::packetType> listening_to;
    virtual void onPacket(packets::packet& packet, connection& conn) = 0;
};

#endif /* serverNetworking_hpp */
