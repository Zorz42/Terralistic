//
//  clientNetworking.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef clientNetworking_hpp
#define clientNetworking_hpp

#include "packets.hpp"

#include <vector>
#include <set>

class networkingManager;

class packetListener {
public:
    packetListener(networkingManager* manager);
    virtual ~packetListener() = default;
    std::set<packets::packetType> listening_to;
    virtual void onPacket(packets::packet packet) = 0;
};

class networkingManager {
    int sock = -1;
    bool listener_running = true;
    static void listenerLoop(networkingManager* manager);
    std::vector<packetListener*> listeners;
public:
    bool establishConnection(const std::string& ip, unsigned short port);
    void closeConnection();

    void sendPacket(packets::packet packet_);
    void registerListener(packetListener* listener);
};

#endif /* clientNetworking_hpp */
