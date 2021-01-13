//
//  networkingModule.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef networkingModule_hpp
#define networkingModule_hpp

#include <string>

namespace networking {

bool establishConnection(const std::string& ip);
std::string getPacket();
void sendPacket(std::string content);
void downloadWorld();

}

#endif /* networkingModule_hpp */
