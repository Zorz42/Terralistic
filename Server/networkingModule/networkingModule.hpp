//
//  networkingModule.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#define MAX_PLAYERS 100

#define PACKET_LISTENER(packetType) static networking::registerPacketListener JOIN(listener_func, __LINE__) (packetType, [](packets::packet& packet, networking::connection& connection) {
#define PACKET_LISTENER_END });

namespace networking {

struct connection {
    std::string ip;
    int socket{-1};
    
    packets::packet getPacket() const;
    void sendPacket(const packets::packet& packet_) const;
    
    bool disconnected = false;
};

typedef void(*listenerFunction)(packets::packet&, networking::connection&);

struct registerPacketListener {
    registerPacketListener(packets::packetType type, listenerFunction function);
};

void spawnListener();
void sendToEveryone(const packets::packet& packet, connection* exclusion=nullptr);

inline networking::connection connections[MAX_PLAYERS];

}

#endif /* networkingModule_hpp */
