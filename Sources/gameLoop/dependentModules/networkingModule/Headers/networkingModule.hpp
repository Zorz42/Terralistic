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

bool establishConnection(const std::string& ip);
void downloadWorld();
void spawnListener();

packets::packet getPacket();
void sendPacket(packets::packet packet_);

}

#endif /* networkingModule_hpp */
