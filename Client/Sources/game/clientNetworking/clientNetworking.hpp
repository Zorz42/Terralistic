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
    explicit packetListener(networkingManager* manager);
    virtual ~packetListener() = default;
    std::set<PacketType> listening_to;
    virtual void onPacket(Packet& packet) = 0;
};

class networkingManager {
    int sock = -1;
    bool listener_running = true;
    static void listenerLoop(networkingManager* manager);
    std::vector<packetListener*> listeners;
    PacketManager packet_manager;
public:
    bool establishConnection(const std::string& ip, unsigned short port);
    void closeConnection();

    void sendPacket(Packet& packet_) const;
    void registerListener(packetListener* listener);
};

#endif /* clientNetworking_hpp */
