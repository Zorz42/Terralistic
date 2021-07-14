//
//  clientNetworking.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef clientNetworking_hpp
#define clientNetworking_hpp

#include <vector>
#include <set>

#include "packets.hpp"
#include "events.hpp"

class networkingManager {
    int sock = -1;
    bool listener_running = true;
    static void listenerLoop(networkingManager* manager);
    PacketManager packet_manager;
public:
    bool establishConnection(const std::string& ip, unsigned short port);
    void closeConnection();

    void sendPacket(Packet& packet_) const;
};

class ClientPacketEvent : public Event<ClientPacketEvent> {
public:
    ClientPacketEvent(Packet& packet) : packet(packet) {}
    Packet& packet;
};

#endif /* clientNetworking_hpp */
