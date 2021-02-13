//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#include <string>
#include "packets.hpp"

namespace networking {

void init();
bool establishConnection(const std::string& ip);
void spawnListener();

packets::packet getPacket();
void sendPacket(packets::packet packet_);

typedef void(*listenerFunction)(packets::packet&);

struct registerListener {
    registerListener(listenerFunction function, packets::packetType type);
};

}

#endif /* networkingModule_hpp */
