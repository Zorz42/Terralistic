//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#define PACKET_LISTENER(packetType) networking::registerPacketListener UNIQUE_NAME(listener_func) (packetType, [](packets::packet& packet) {
#define PACKET_LISTENER_END });

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
