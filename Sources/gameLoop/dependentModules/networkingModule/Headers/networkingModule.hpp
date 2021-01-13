//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#include <string>
#include "networkingEvents.hpp"

namespace networking {

struct packet {
    packet(packetType type, std::string contents="") : contents(contents), type(type) {}
    std::string contents;
    packetType type;
};

bool establishConnection(const std::string& ip);
packet getPacket();
void sendPacket(packet content);
void downloadWorld();

}

#endif /* networkingModule_hpp */
