//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#define CONCAT_(x,y) x##y
#define CONCAT(x,y) CONCAT_(x,y)
#define PACKET_LISTENER(packetType) networking::registerPacketListener CONCAT(listener_func, __LINE__) (packetType, [](packets::packet& packet) {
#define PACKET_LISTENER_END });

#include <string>
#include "packets.hpp"

namespace networking {

bool establishConnection(const std::string& ip);
void spawnListener();

packets::packet getPacket();
void sendPacket(packets::packet packet_);

typedef void(*listenerFunction)(packets::packet&);

struct registerPacketListener {
    registerPacketListener(packets::packetType type, listenerFunction function);
};

}

#endif /* networkingModule_hpp */
